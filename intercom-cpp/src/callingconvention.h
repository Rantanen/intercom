#ifndef INTERCOM_CPP_CALLINGCONCENTION_H
#define INTERCOM_CPP_CALLINGCONCENTION_H

// On Windows platform the calling convention set by COM is used.
#ifdef _MSC_VER

// stdcall
#define INTERCOM_CC __stdcall
#else

// cdecl
#define INTERCOM_CC

#endif

#endif
