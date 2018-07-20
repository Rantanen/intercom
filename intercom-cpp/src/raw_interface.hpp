
#ifndef INTERCOM_CPP_RAWINTERFACE_H
#define INTERCOM_CPP_RAWINTERFACE_H


#include <cassert>
#include <type_traits>

namespace intercom
{

/**
 * @brief Specifies an
 *
 * @tparam TInterface
 */
template< typename TInterface >
class RawInterface
{
public:

    /**
     * @brief Initialize new empty RawInterface-
     *
     * @param interface
     */
    constexpr RawInterface() noexcept :
        m_interface( nullptr )
    {
    }

    /**
     * @brief Initialize new RawInterface from an existing interface.
     *
     * @param interface
     */
    explicit constexpr RawInterface(
        TInterface* itf
    ) noexcept :
        m_interface( itf )
    {
        if( m_interface != nullptr )
            m_interface->AddRef();
    }

    /**
     * @brief Assigns the interface from source to this object.
     *
     * @param interface Interface assigned to this wrapper.
     * @returns Returns a reference to this object.
     */
    RawInterface& operator=(
        TInterface* itf
    ) noexcept
    {
        this->reset( itf );
        return *this;
    }

    ~RawInterface()
    {
        if( m_interface != nullptr )
            m_interface->Release();
    }

    /**
     * @brief Checks whether *this owns an interface, i.e. whether get() != nullptr.
     *
     * @return operator bool const
     */
    explicit operator bool() const noexcept { return m_interface != nullptr; }

// Methods for accessing the interface.
public:

    /**
     * @brief Accesses the internal interface.
     *
     * @return TInterface* Returns the interface.
     */
    typename std::add_lvalue_reference< TInterface >::type operator*() const noexcept { return *m_interface; }

    /**
     * @brief Accesses the internal interface.
     *
     * @return TInterface* Returns the interface.
     */
    TInterface* operator->() const noexcept { return m_interface; }

    /**
     * @brief Returns a pointer to the managed interface or nullptr if no interface is owned.
     *
     * @return TInterface* The owner interface if any.
     */
    TInterface* get() const noexcept { return m_interface; }

     /**
     * @brief Accesses the memory address that holds a reference to the interface.
     *
     * Use this method to receive interfaces from methods.
     *
     * @return TInterface* Returns the interface.
     */
    TInterface** operator&() noexcept { assert( m_interface == nullptr ); return &m_interface; }

    /**
     * @brief Accesses the memory address that holds a reference to the interface.
     *
     * Use this method to receive interfaces from methods.
     *
     * @return TInterface** out
     */
    void** out() noexcept { assert( m_interface == nullptr ); return (void**) &m_interface; }

// Methods for modifying the value.
public:

    /**
     * @brief Returns a pointer to the managed object and releases the ownership
     *
     * @return TInterface* release
     */
    TInterface* release() noexcept
    {
        TInterface* itf = m_interface;
        m_interface = nullptr;
        return itf;
    }

    /**
     * @brief Replaces the interface.
     *
     * @param interface Replacement.
     */
    void reset(
        TInterface* itf = nullptr
    ) noexcept
    {
        // No-op?
        if( m_interface == itf )
            return;

        // Release previous reference.
        if( m_interface != nullptr )
            m_interface->Release();

        // Store the new reference.
        m_interface = itf;
        if( m_interface != nullptr )
            m_interface->AddRef();
    }

// Move and copy operators.
public:

    /**
     * @brief Moves the interface from source to this object.
     *
     * @param source Source.
     */
    RawInterface(
        RawInterface&& source
    ) noexcept :
        m_interface( source.m_interface )
    {
        source.m_interface = nullptr;
    }

    /**
     * @brief Moves the interface from source to this object.
     *
     * @param source Source.
     * @returns Returns a reference to this object.
     */
    RawInterface& operator=(
        RawInterface&& source
    ) noexcept
    {
        if( this != std::addressof( source ) )
        {
            m_interface = source.m_interface;
            source.m_interface = nullptr;
        }
        return *this;
    }

    /**
     * @brief Assigns interface from source to this object and increments the reference count.
     *
     * @param source Source.
     * @returns Returns a reference to this object.
     */
    RawInterface(
        const RawInterface& source
    ) noexcept :
        m_interface( source.m_interface )
    {
        if( m_interface != nullptr )
            m_interface->AddRef();
    }

    /**
     * @brief Assigns interface from source to this object and increments the reference count.
     *
     * @param source Source.
     * @returns Returns a reference to this object.
     */
    RawInterface& operator=(
        const RawInterface& source
    ) noexcept
    {
        if( this != std::addressof( source ) )
        {
            this->reset( source.m_interface );
        }
        return *this;
    }

private:

    // Allow the swap function to do the swap efficiently.
    template< typename T >
    friend void std::swap(
        intercom::RawInterface< T >&,
        intercom::RawInterface< T >&
    ) noexcept;

    TInterface* m_interface;  //!< Holds reference to the actual interface.
};

}

namespace std
{
    /**
     * @brief Implements hash function for intercom::RawInterface
     *
     * @tparam T
     */
    template< typename TInterface >
    struct hash< intercom::RawInterface< TInterface > >
    {
        std::size_t operator()(
            intercom::RawInterface< TInterface > const& s
        ) const noexcept
        {
            return std::hash< TInterface* >{}( s.get() );
        }
    };

    /**
     * @brief Implements swap function for intercom::RawInterface
     *
     * @tparam TInterface Specifies the raw interface.
     * @param lhs Swapped value
     * @param rhs Swapped value
     */
    template< typename TInterface >
    void swap(
        intercom::RawInterface< TInterface >& lhs,
        intercom::RawInterface< TInterface >& rhs
    ) noexcept
    {
        TInterface* temp = lhs.m_interface;
        lhs.m_interface = rhs.m_interface;
        rhs.m_interface = temp;
    }
}

#endif
