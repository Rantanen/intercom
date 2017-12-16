
// Defines for miscellaneous Windows specific macros for other platforms.
// Required to compile MIDL interfaces generated with midl.exe until we have cross-platform header generator.
#define OUT
#define interface class
#define EXTERN_C extern "C"
#define MIDL_INTERFACE( x ) struct
#define STDMETHODCALLTYPE __attribute__((stdcall))
#define DECLSPEC_UUID( x )