#ifndef INTERCOM_CPP_DETAIL_POSIX_CLASS_MAPPING_H
#define INTERCOM_CPP_DETAIL_POSIX_CLASS_MAPPING_H

#include <link.h>

#include <iostream>
#include <mutex>
#include <tuple>
#include <unordered_set>
#include <unordered_map>
#include <vector>

#include "../../guiddef.h"
#include "../declarations.hpp"
#include "../../runtime_error.h"
#include "../dlwrapper.h"
#include "../utility.h"

namespace intercom
{
    //! Declaration for a generic hash function for intercom types.
    template< typename THashed >
    struct hash;
}

namespace intercom
{
namespace detail
{

/**
 * @brief Holds a mapping from class identities to the libraries that host them.
 *
 */
class LibraryIndex
{
private:

    /**
     * @brief A container for the names of the libraries that have associated classes.
     *
     */
    typedef std::unordered_set< std::string > LIBRARIES;

    /**
     * @brief Defines the association between the classes and the libraries that implement them.
     *
     */
    typedef std::unordered_map< intercom::CLSID, const char*, intercom::hash< intercom::IID > > ASSOCIATED_CLASSES;

    typedef std::tuple< std::unordered_set< std::string >*,ASSOCIATED_CLASSES*, std::string* > DL_ITERATE_PARAMETER;

public:

    //! Searches for a library associated with the specified class identity.
    const char* find_library(
        const intercom::CLSID& class_id
    ) const
    {
        std::unique_lock< std::mutex > lock( m_guard );

        // Refresh the list of libraries the class is unknonw.
        ASSOCIATED_CLASSES::const_iterator library = m_associated_classes.find( class_id );
        if( library == m_associated_classes.end() )
        {
            refresh_associations();

            // The libraries that implement COM classes must be linked dynamically
            // to the application for the discovery to work.
            // Otherwise the "intercom" cannot find them.
            // Assume something has gone wrong if the caller wants to find a library
            // but none are available / linked to the executable.
            if( m_associated_classes.empty() )
            {
                std::cerr << "WARNING: None of the libraries linked to the application expose COM classes." << std::endl;

                std::cerr << "WARNING: Only the following libraries were identified:" << std::endl;
                for( const std::string& library_name : m_libraries )
                    std::cerr << "    " << library_name << std::endl;
            }

            library = m_associated_classes.find( class_id );
        }

        if( library == m_associated_classes.end() )
            return nullptr;
        else
            return library->second;
    }

    /**
     * @brief Attempts to register a library for the "intercom".
     *
     * @param library_name The name of the library to register
     * @param expected_classes_begin A pointer to the beginning of an array of classes the library is expected to implement.
     * @param expected_classes_end A pointer to the end of an array of classes the library is expected to implement.
     * @return Returns true if registering the library succeed and all the expected classes are availab.e.
     */
    inline bool try_register_library(
        const char* library_name,
        const intercom::CLSID* expected_classes_begin,
        const intercom::CLSID* expected_classes_end
    )
    {
        // Try loading the library.
        // This function is called during static initialization.
        // We want to avoid throwing exceptions at that time.
        try
        {
            intercom::posix::DlWrapper library( library_name,
                    intercom::posix::DlWrapper::rtld::lazy );

            refresh_associations();

            // Ensure that the library implements each class the caller expects.
            // It is possible that the application has been compiled against a newer
            // version of the Rust COM library than what is available on the system.

            std::unique_lock< std::mutex > lock( m_guard );

            bool all_expected_classes_found = true;
            for( const intercom::CLSID* current = expected_classes_begin;
                current != expected_classes_end; ++current )
            {
                if( m_associated_classes.find( *current ) == m_associated_classes.end() )
                {
                    std::cerr << "WARNING: The library \"" << library_name << "\" does not implement the class \"";
                    ::intercom::operator<<( std::cerr, *current );
                    std::cerr << "\"" << std::endl;
                    all_expected_classes_found = false;

                    // Do not break the loop here to ensure each missing class gets reported.
                }
            }

            return all_expected_classes_found;
        }
        catch( ... )
        {
            std::cerr << "WARNING: Registering the library \"" << library_name << "\" failed." << std::endl;

            return false;
        }
    }

private:

    /**
     * @brief Walks through the libraries linked to the applications and searches for COM classes.
     *
     */
    void refresh_associations() const
    {
        // Send the state of the index to the iterator.
        LIBRARIES* libraries = &m_libraries;
        ASSOCIATED_CLASSES* associated_classes = &m_associated_classes;
        std::string error;
        DL_ITERATE_PARAMETER parameter = std::make_tuple( libraries, associated_classes, &error );

        int result = dl_iterate_phdr(
                [](
                    struct dl_phdr_info *info,
                    size_t size, void *data
                ) -> int {

                    DL_ITERATE_PARAMETER& parameter = *static_cast< DL_ITERATE_PARAMETER* >( data );

                    // C-libraries are not prepared for exceptions.
                    // => Process the possible exceptions here.
                    try
                    {
                        refresh_library_associations( info,
                                *std::get< 0 >( parameter ), *std::get< 1 >( parameter ) );
                    }
                    catch( intercom::RuntimeError& ex )
                    {
                        *std::get< 2 >( parameter ) = ex.what();
                        return ex.error_code();
                    }
                    catch( std::bad_alloc& ex )
                    {
                        *std::get< 2 >( parameter ) = ex.what();
                        return intercom::EC_OUTOFMEMORY;
                    }
                    catch( std::exception& ex )
                    {
                        *std::get< 2 >( parameter ) = ex.what();
                        return intercom::EC_FAIL;
                    }
                    catch( ... )
                    {
                        return intercom::EC_FAIL;
                    }

                    // By always returning zero we ensure that all the loaded libraries are iterated.
                    return 0;
                },
                &parameter
        );

        if( result != 0 )
            throw intercom::RuntimeError( result, error.c_str() );
    }

    /**
     * @brief Gets all libraries loaded into the process.
     *
     * @param info Information about the current library.
     * @param libraries  Collection of libraries.
     * @param associated_classes  Collection of classes that are associated with libraries.
     */
    static void refresh_library_associations(
        struct dl_phdr_info *info,
        std::unordered_set< std::string >& libraries,
        ASSOCIATED_CLASSES& associated_classes
    )
    {
        // Associate the classes available within the library if we haven't built the associations yet.
        std::pair< LIBRARIES::iterator, bool > inserted = libraries.insert( info->dlpi_name );
        if( inserted.second )
        {
            const char* library_name = inserted.first->c_str();
            for( intercom::CLSID class_id : get_classes( library_name ) )
                associated_classes[ class_id ] = library_name;
        }
    }

    /**
     * @brief Gets all available classes associated with the library.
     *
     * @param library_name  Specifies the library.
     * @return std::vector< intercom::CLSID > Returns the classes the library implements.
     */
    static std::vector< intercom::CLSID > get_classes(
        const char* library_name
    )
    {
        // Ignore libraries that do not implement "IntercomListClassObjects" function.
        // All Rust libraries that expose COM methods should implement
        // the "IntercomListClassObjects" function.
        std::vector< intercom::CLSID > classes;
        intercom::posix::DlWrapper library( library_name,
                    intercom::posix::DlWrapper::rtld::lazy );
        intercom::detail::IntercomListClassObjectsFunc list_class_objects;
        if( library.try_load_function( "IntercomListClassObjects", &list_class_objects ) )
        {
            // NOTE: Listing the class objects should always succeed as that function is generated automatically and
            // canot fail.
            size_t class_count;
            intercom::CLSID* found_classes;
            intercom::HRESULT hr = list_class_objects( &class_count, &found_classes );
            if( hr != intercom::SC_OK )
                throw intercom::RuntimeError( hr, "Calling \"IntercomListClassObjects\" failed." );

            classes.reserve( class_count );
            for( size_t s = 0; s < class_count; ++s )
                classes.push_back( found_classes[ s ] );
        }

        return classes;
    }

    mutable std::mutex m_guard;  //!< Protects access to the containers.
    mutable std::unordered_set< std::string > m_libraries;  //!< Append-only list of libraries found in the process.
    mutable ASSOCIATED_CLASSES m_associated_classes;  //!< Maps class identities to the libaries. The name of the library is stored in m_libraries.

};

}
}

#endif