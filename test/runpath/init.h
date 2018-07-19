
// Functions need to be explicitly exported on Windows in order to the generate import library file.
#ifdef _MSC_VER
    #define EXPORT_API __declspec( dllexport )
#else
    #define EXPORT_API
#endif

/**
 * @brief Expose a symbol which other binaries can callo to ensure libpath is linked.
 *
 */
#ifdef __cplusplus
extern "C" {
#endif

    EXPORT_API void init_runpath();

#ifdef __cplusplus
}
#endif
