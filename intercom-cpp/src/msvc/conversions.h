
#ifndef INTERCOM_CPP_MSVC_CONVERSIONS_H
#define INTERCOM_CPP_MSVC_CONVERSIONS_H

#include <cstdlib>

#include "memory.h"
#include "../detail/bstr_buffer.h"
#include "../detail/char_buffer.h"

namespace intercom
{
    /**
     * @brief Converts UTF-8 string to BSTR.
     *
     * The received BSTR must be deallocated with "intercom::free_bstr".
     */
    inline void utf8_to_bstr(
        const char* utf8_string,
        intercom::BSTR* bstr_string
    )
    {
        // TODO.
        std::terminate();
    }

    /**
     * @brief Converts BSTR string to UTF-8.
     *
     * The received char* must be deallocated with "free".
     */
    inline void bstr_to_utf8(
        const intercom::BSTR bstr_string,
        char** utf8_string
    )
    {
        // TODO.
        std::terminate();
    }
}

#endif
