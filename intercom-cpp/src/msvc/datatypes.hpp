#ifndef INTERCOM_CPP_MSVC_DATATYPES_H
#define INTERCOM_CPP_MSVC_DATATYPES_H

#include <cstdint>

// Use predefined set if available.
#include<WinDef.h>

// The generated C++ headers and classes expect the data types in intercom namespace.
namespace intercom
{
    namespace _internal
    {
        template< typename TTarget, typename TSource >
        TTarget checked_cast( TSource source )
        {
			if( std::numeric_limits< TTarget >::is_signed &&
				! std::numeric_limits< TSource >::is_signed )
			{
				// Target signed, Source not signed.
				// -> No need to check the min bound.
				//
				// Min bound check would result in bad checks due to type
				// conversion to signed, etc.
				if( source > ( std::numeric_limits< TTarget >::max )() )
				{
					_ASSERTE( false );
					throw std::runtime_error( "Value out of range" );
				}
			}
			else
			{
				// Every other case.
				if( source < ( std::numeric_limits< TTarget >::min )() )
				{
					_ASSERTE( false );
					throw std::runtime_error( "Value out of range" );
				}
				if( source > ( std::numeric_limits< TTarget >::max )() )
				{
					_ASSERTE( false );
					throw std::runtime_error( "Value out of range" );
				}
			}

            return static_cast< TTarget >( source );
        }
    }

    typedef INT8 INT8;
    typedef UINT8 UINT8;
    typedef INT16 INT16;
    typedef UINT16 UINT16;
    typedef INT32 INT32;
    typedef UINT32 UINT32;
    typedef INT64 INT64;
    typedef UINT64 UINT64;

    typedef BOOL BOOL;
    typedef BYTE BYTE;
    typedef ULONG ULONG;
    typedef DWORD DWORD;
    typedef WORD WORD;

    typedef OLECHAR OLECHAR;
    typedef BSTR BSTR;

    typedef HRESULT HRESULT;

    //! 32-bit reference counter. unsigned long is 32-bit in Windows and 64-bit on Unix.
    typedef unsigned long REF_COUNT_32;
}

#endif
