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

  if (!s1)
    s1 = "";
  if (!s2)
    s2 = "";

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
int32_t gul_print_float(double f) { return printf("%f\n", f); }

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
// User Input Operations
// ============================================================================

// Read string from stdin (default input)
int64_t gul_input_str() {
  char *buffer = malloc(1024);
  if (!buffer) {
    fprintf(stderr, "GUL Runtime: Out of memory in input_str\n");
    exit(1);
  }

  if (fgets(buffer, 1024, stdin)) {
    // Remove trailing newline
    size_t len = strlen(buffer);
    if (len > 0 && buffer[len - 1] == '\n') {
      buffer[len - 1] = '\0';
    }
  } else {
    buffer[0] = '\0';
  }

  return (int64_t)buffer;
}

// Read integer from stdin
int64_t gul_input_int() {
  long value = 0;
  if (scanf("%ld", &value) != 1) {
    fprintf(stderr, "GUL Runtime: Failed to read integer\n");
    value = 0;
  }
  // Clear remaining input buffer
  int c;
  while ((c = getchar()) != '\n' && c != EOF)
    ;
  return (int64_t)value;
}

// Read float from stdin
double gul_input_flt() {
  double value = 0.0;
  if (scanf("%lf", &value) != 1) {
    fprintf(stderr, "GUL Runtime: Failed to read float\n");
    value = 0.0;
  }
  // Clear remaining input buffer
  int c;
  while ((c = getchar()) != '\n' && c != EOF)
    ;
  return value;
}

// ============================================================================
// Debug/Trace (optional)
// ============================================================================

#ifdef GUL_DEBUG
void gul_debug_print(const char *msg) {
  fprintf(stderr, "[GUL DEBUG] %s\n", msg);
}
#endif

// ============================================================================
// Math Functions (for ML and numerical computing)
// ============================================================================

#include <math.h>

// Trigonometric
double gul_math_sin(double x) { return sin(x); }
double gul_math_cos(double x) { return cos(x); }
double gul_math_tan(double x) { return tan(x); }
double gul_math_asin(double x) { return asin(x); }
double gul_math_acos(double x) { return acos(x); }
double gul_math_atan(double x) { return atan(x); }
double gul_math_atan2(double y, double x) { return atan2(y, x); }

// Exponential and logarithmic
double gul_math_exp(double x) { return exp(x); }
double gul_math_log(double x) { return log(x); }
double gul_math_log10(double x) { return log10(x); }
double gul_math_log2(double x) { return log2(x); }

// Power and roots
double gul_math_pow(double x, double y) { return pow(x, y); }
double gul_math_sqrt(double x) { return sqrt(x); }
double gul_math_cbrt(double x) { return cbrt(x); }

// Rounding
double gul_math_floor(double x) { return floor(x); }
double gul_math_ceil(double x) { return ceil(x); }
double gul_math_round(double x) { return round(x); }
double gul_math_trunc(double x) { return trunc(x); }

// Absolute value
double gul_math_abs(double x) { return fabs(x); }
int64_t gul_math_abs_int(int64_t x) { return x < 0 ? -x : x; }

// Min/Max
double gul_math_min(double a, double b) { return a < b ? a : b; }
double gul_math_max(double a, double b) { return a > b ? a : b; }

// ML-specific activation functions
double gul_ml_sigmoid(double x) { return 1.0 / (1.0 + exp(-x)); }
double gul_ml_tanh(double x) { return tanh(x); }
double gul_ml_relu(double x) { return x > 0.0 ? x : 0.0; }

// ============================================================================
// Tensor Primitives (for ML)
// ============================================================================

// Allocate tensor data (flat array)
int64_t gul_tensor_alloc(int64_t num_elements) {
  double *data = (double *)malloc(num_elements * sizeof(double));
  if (!data) {
    fprintf(stderr, "GUL Runtime: Out of memory allocating tensor\n");
    exit(1);
  }
  return (int64_t)data;
}

// Free tensor data
void gul_tensor_free(int64_t ptr) { free((void *)ptr); }

// Fill tensor with value
void gul_tensor_fill(int64_t ptr, int64_t size, double value) {
  double *data = (double *)ptr;
  for (int64_t i = 0; i < size; i++) {
    data[i] = value;
  }
}

// Element-wise add
void gul_tensor_add(int64_t dst, int64_t a, int64_t b, int64_t size) {
  double *d = (double *)dst;
  double *x = (double *)a;
  double *y = (double *)b;
  for (int64_t i = 0; i < size; i++) {
    d[i] = x[i] + y[i];
  }
}

// Element-wise multiply
void gul_tensor_mul(int64_t dst, int64_t a, int64_t b, int64_t size) {
  double *d = (double *)dst;
  double *x = (double *)a;
  double *y = (double *)b;
  for (int64_t i = 0; i < size; i++) {
    d[i] = x[i] * y[i];
  }
}

// Matrix multiply: C[m,n] = A[m,k] * B[k,n]
void gul_tensor_matmul(int64_t c_ptr, int64_t a_ptr, int64_t b_ptr, int64_t m,
                       int64_t k, int64_t n) {
  double *C = (double *)c_ptr;
  double *A = (double *)a_ptr;
  double *B = (double *)b_ptr;

  for (int64_t i = 0; i < m; i++) {
    for (int64_t j = 0; j < n; j++) {
      double sum = 0.0;
      for (int64_t l = 0; l < k; l++) {
        sum += A[i * k + l] * B[l * n + j];
      }
      C[i * n + j] = sum;
    }
  }
}

// Sum all elements
double gul_tensor_sum(int64_t ptr, int64_t size) {
  double *data = (double *)ptr;
  double sum = 0.0;
  for (int64_t i = 0; i < size; i++) {
    sum += data[i];
  }
  return sum;
}

// Mean of all elements
double gul_tensor_mean(int64_t ptr, int64_t size) {
  return gul_tensor_sum(ptr, size) / (double)size;
}

// ============================================================================
// SIMD Vector Operations (4-element float vectors)
// ============================================================================

#ifdef __SSE__
#include <xmmintrin.h>
#define GUL_HAS_SSE 1
#else
#define GUL_HAS_SSE 0
#endif

// Vec4f: 4 x float32 vector
typedef struct {
  float x, y, z, w;
} GulVec4f;

// Create vector
GulVec4f gul_simd_vec4f(float x, float y, float z, float w) {
  return (GulVec4f){x, y, z, w};
}

// Vector addition
GulVec4f gul_simd_add(GulVec4f a, GulVec4f b) {
#if GUL_HAS_SSE
  __m128 va = _mm_set_ps(a.w, a.z, a.y, a.x);
  __m128 vb = _mm_set_ps(b.w, b.z, b.y, b.x);
  __m128 vr = _mm_add_ps(va, vb);
  float r[4];
  _mm_storeu_ps(r, vr);
  return (GulVec4f){r[0], r[1], r[2], r[3]};
#else
  return (GulVec4f){a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w};
#endif
}

// Vector subtraction
GulVec4f gul_simd_sub(GulVec4f a, GulVec4f b) {
#if GUL_HAS_SSE
  __m128 va = _mm_set_ps(a.w, a.z, a.y, a.x);
  __m128 vb = _mm_set_ps(b.w, b.z, b.y, b.x);
  __m128 vr = _mm_sub_ps(va, vb);
  float r[4];
  _mm_storeu_ps(r, vr);
  return (GulVec4f){r[0], r[1], r[2], r[3]};
#else
  return (GulVec4f){a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w};
#endif
}

// Vector multiply (element-wise)
GulVec4f gul_simd_mul(GulVec4f a, GulVec4f b) {
#if GUL_HAS_SSE
  __m128 va = _mm_set_ps(a.w, a.z, a.y, a.x);
  __m128 vb = _mm_set_ps(b.w, b.z, b.y, b.x);
  __m128 vr = _mm_mul_ps(va, vb);
  float r[4];
  _mm_storeu_ps(r, vr);
  return (GulVec4f){r[0], r[1], r[2], r[3]};
#else
  return (GulVec4f){a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w};
#endif
}

// Dot product
float gul_simd_dot(GulVec4f a, GulVec4f b) {
  GulVec4f m = gul_simd_mul(a, b);
  return m.x + m.y + m.z + m.w;
}

// Magnitude (length)
float gul_simd_magnitude(GulVec4f v) { return sqrtf(gul_simd_dot(v, v)); }

// Normalize
GulVec4f gul_simd_normalize(GulVec4f v) {
  float mag = gul_simd_magnitude(v);
  if (mag > 0.0f) {
    return (GulVec4f){v.x / mag, v.y / mag, v.z / mag, v.w / mag};
  }
  return v;
}

// Cross product (3D, ignores w)
GulVec4f gul_simd_cross(GulVec4f a, GulVec4f b) {
  return (GulVec4f){a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z,
                    a.x * b.y - a.y * b.x, 0.0f};
}

// ============================================================================
// SIMD-accelerated tensor operations (process 4 elements at a time)
// ============================================================================

void gul_tensor_add_simd(int64_t dst, int64_t a, int64_t b, int64_t size) {
  float *d = (float *)dst;
  float *x = (float *)a;
  float *y = (float *)b;

  int64_t i = 0;
#if GUL_HAS_SSE
  // Process 4 floats at a time
  for (; i + 4 <= size; i += 4) {
    __m128 va = _mm_loadu_ps(x + i);
    __m128 vb = _mm_loadu_ps(y + i);
    __m128 vr = _mm_add_ps(va, vb);
    _mm_storeu_ps(d + i, vr);
  }
#endif
  // Handle remaining elements
  for (; i < size; i++) {
    d[i] = x[i] + y[i];
  }
}

void gul_tensor_mul_simd(int64_t dst, int64_t a, int64_t b, int64_t size) {
  float *d = (float *)dst;
  float *x = (float *)a;
  float *y = (float *)b;

  int64_t i = 0;
#if GUL_HAS_SSE
  for (; i + 4 <= size; i += 4) {
    __m128 va = _mm_loadu_ps(x + i);
    __m128 vb = _mm_loadu_ps(y + i);
    __m128 vr = _mm_mul_ps(va, vb);
    _mm_storeu_ps(d + i, vr);
  }
#endif
  for (; i < size; i++) {
    d[i] = x[i] * y[i];
  }
}
