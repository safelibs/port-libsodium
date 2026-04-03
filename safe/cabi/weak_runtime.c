#include <stddef.h>

#if defined(__clang__) || defined(__GNUC__)
# define SODIUM_STACK_ALLOCA __builtin_alloca
#else
# include <alloca.h>
# define SODIUM_STACK_ALLOCA alloca
#endif

typedef struct CPUFeatures_ {
    int initialized;
    int has_neon;
    int has_sse2;
    int has_sse3;
    int has_ssse3;
    int has_sse41;
    int has_avx;
    int has_avx2;
    int has_avx512f;
    int has_pclmul;
    int has_aesni;
    int has_rdrand;
} CPUFeatures;

static CPUFeatures cpu_features;

#define SODIUM_WEAK_EXPORT __attribute__((weak, visibility("default")))
#define SODIUM_EXPORT __attribute__((visibility("default")))
#define SODIUM_HIDDEN __attribute__((visibility("hidden")))

void sodium_memzero(void * const pnt, const size_t len);

SODIUM_HIDDEN int
_sodium_runtime_get_cpu_features(void)
{
    int ret = -1;

#if defined(__aarch64__) || defined(__arm__)
# if defined(__ARM_NEON) || defined(__ARM_NEON__)
    cpu_features.has_neon = 1;
# else
    cpu_features.has_neon = 0;
# endif
    ret = 0;
#else
    cpu_features.has_neon = 0;
#endif

#if defined(__x86_64__) || defined(__i386__)
# if defined(__GNUC__) || defined(__clang__)
    __builtin_cpu_init();
    cpu_features.has_sse2 = __builtin_cpu_supports("sse2");
    cpu_features.has_sse3 = __builtin_cpu_supports("sse3");
    cpu_features.has_ssse3 = __builtin_cpu_supports("ssse3");
    cpu_features.has_sse41 = __builtin_cpu_supports("sse4.1");
    cpu_features.has_avx = __builtin_cpu_supports("avx");
    cpu_features.has_avx2 = __builtin_cpu_supports("avx2");
    cpu_features.has_avx512f = __builtin_cpu_supports("avx512f");
    cpu_features.has_pclmul = __builtin_cpu_supports("pclmul");
    cpu_features.has_aesni = __builtin_cpu_supports("aes");
    cpu_features.has_rdrand = __builtin_cpu_supports("rdrnd");
    ret = 0;
# endif
#endif

    cpu_features.initialized = 1;
    return ret;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_neon(void)
{
    return cpu_features.has_neon;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_sse2(void)
{
    return cpu_features.has_sse2;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_sse3(void)
{
    return cpu_features.has_sse3;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_ssse3(void)
{
    return cpu_features.has_ssse3;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_sse41(void)
{
    return cpu_features.has_sse41;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_avx(void)
{
    return cpu_features.has_avx;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_avx2(void)
{
    return cpu_features.has_avx2;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_avx512f(void)
{
    return cpu_features.has_avx512f;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_pclmul(void)
{
    return cpu_features.has_pclmul;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_aesni(void)
{
    return cpu_features.has_aesni;
}

SODIUM_WEAK_EXPORT int
sodium_runtime_has_rdrand(void)
{
    return cpu_features.has_rdrand;
}

SODIUM_EXPORT void
sodium_stackzero(const size_t len)
{
    if (len > 0U) {
        sodium_memzero(SODIUM_STACK_ALLOCA(len), len);
    }
}
