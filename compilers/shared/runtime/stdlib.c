// GUL Runtime Library
// Shared between gul_stable and gul_nightly compilers
// Linked with compiled GUL programs via cc

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ============================================================================
// String Operations
// ============================================================================

// String Concatenation: returns newly allocated string
int64_t gul_string_concat(int64_t a, int64_t b) {
    char *s1 = (char *)a;
    char *s2 = (char *)b;

    if (!s1) s1 = "";
    if (!s2) s2 = "";

    size_t l1 = strlen(s1);
    size_t l2 = strlen(s2);

    char *res = malloc(l1 + l2 + 1);
    if (!res) {
        fprintf(stderr, "GUL Runtime: Out of memory in string_concat\n");
        exit(1);
    }

    strcpy(res, s1);
    strcat(res, s2);

    return (int64_t)res;
}

// Integer to String conversion
int64_t gul_int_to_string(int64_t n) {
    char *res = malloc(32);
    if (!res) {
        fprintf(stderr, "GUL Runtime: Out of memory in int_to_string\n");
        exit(1);
    }
    sprintf(res, "%ld", n);
    return (int64_t)res;
}

// ============================================================================
// Float Operations
// ============================================================================

// Print float with newline
int32_t gul_print_float(double f) {
    return printf("%f\n", f);
}

// Float to String conversion
int64_t gul_float_to_string(double f) {
    char *res = malloc(64);
    if (!res) {
        fprintf(stderr, "GUL Runtime: Out of memory in float_to_string\n");
        exit(1);
    }
    sprintf(res, "%f", f);
    return (int64_t)res;
}

// ============================================================================
// Memory Operations (for collections)
// ============================================================================

// Note: malloc, realloc, memmove are used directly via libc
// These wrappers can be added for custom allocators in the future

// ============================================================================
// Debug/Trace (optional)
// ============================================================================

#ifdef GUL_DEBUG
void gul_debug_print(const char *msg) {
    fprintf(stderr, "[GUL DEBUG] %s\n", msg);
}
#endif
