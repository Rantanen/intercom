
#ifndef INTERCOM_CPP_MSVC_UTILITY_H
#define INTERCOM_CPP_MSVC_UTILITY_H


#include <cstring>
#include <iomanip>
#include <sstream>
#include <type_traits>

// Include minimal Win32 API.
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#define INTERCOM_WIN32_LEAN_AND_MEAN_DEFINED
#endif

#ifndef VC_EXTRALEAN
#define VC_EXTRALEAN
#define INTERCOM_VC_EXTRALEAN_DEFINED
#endif

#include <Windows.h>

namespace intercom
{
namespace msvc
{
    //! Attempts to get the error message for GetLastError()
    inline std::string get_last_error()
    {
        DWORD dwLastError = ::GetLastError();
        char* buffer = nullptr;
        DWORD dwResult = ::FormatMessageA( FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM,
                nullptr, dwLastError, 0, (LPSTR) &buffer, 0, nullptr );
        if( dwResult == 0 )
            throw std::exception();

        std::string error = buffer;
        ::LocalFree( buffer );

        return buffer;
    }

    //! Throws an exception if last call to dl* failed.
    [[ noreturn ]]
    inline void throw_win32_error()
    {
        std::string error = get_last_error();
        throw std::runtime_error( error );
    }
}
}

// Undefine WIN32_LEAN_AND_MEAN if we defined it to avoid causing problems for other developers.
#ifdef INTERCOM_WIN32_LEAN_AND_MEAN_DEFINED
#undef WIN32_LEAN_AND_MEAN
#undef INTERCOM_WIN32_LEAN_AND_MEAN_DEFINED
#endif

// Undefine VC_EXTRALEAN if we defined it to avoid causing problems for other developers.
#ifdef INTERCOM_VC_EXTRALEAN_DEFINED
#undef VC_EXTRALEAN
#undef INTERCOM_VC_EXTRALEAN_DEFINED
#endif

#endif