#ifndef PLANTUML_ENCODING_H
#define PLANTUML_ENCODING_H

#ifdef __cplusplus
extern "C"
{
#endif // __cplusplus

    struct Result
    {
        const char *ptr;
        size_t      len;
    };

    Result plantuml_encode(const char *input, size_t length);

    Result plantuml_decode(const char *input, size_t length);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif
