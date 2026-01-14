// GUL Runtime Library
// Shared between gul_stable and gul_nightly compilers
// Linked with compiled GUL programs via cc

#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ============================================================================
// String Operations
// ============================================================================

// Print string with newline
int32_t print(int64_t s_ptr) {
  char *s = (char *)s_ptr;
  if (!s)
    return printf("\n");
  return printf("%s\n", s);
}

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

// String to Bool conversion (Auto conversion)
int64_t gul_str_to_bool(int64_t s_ptr) {
  char *s = (char *)s_ptr;
  if (!s)
    return 0;
  if (strcmp(s, "true") == 0)
    return 1;
  return 0;
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
// ============================================================================
// Auto-Differentiation (Tape / Wengert List)
// ============================================================================

#define MAX_TAPE_NODES 10000

typedef enum {
  OP_NONE,
  OP_ADD,
  OP_SUB,
  OP_MUL,
  OP_DIV,
  OP_SIN,
  OP_COS,
  OP_EXP,
  OP_LOG,
  OP_POW
} OpType;

typedef struct {
  OpType op;
  double value;
  double grad;
  int parents[2]; // Indices into tape, -1 if leaf/none
  int computed;   // For topological sort/visitation if needed
} Node;

typedef struct {
  Node nodes[MAX_TAPE_NODES];
  int count;
  int active;
} Tape;

Tape global_tape = {.count = 0, .active = 0};

// Reset and enable tape
void gul_autograd_begin() {
  global_tape.count = 0;
  global_tape.active = 1;
  // node 0 can be null/unused or just start from 0
}

// Disable tape
void gul_autograd_end() { global_tape.active = 0; }

// Helper to add node
int tape_add_node(OpType op, double val, int p1, int p2) {
  if (!global_tape.active)
    return -1;
  if (global_tape.count >= MAX_TAPE_NODES) {
    fprintf(stderr, "GUL Runtime: Autograd tape overflow\n");
    return -1; // Fail gracefully-ish?
  }
  int idx = global_tape.count++;
  Node *n = &global_tape.nodes[idx];
  n->op = op;
  n->value = val;
  n->grad = 0.0;
  n->parents[0] = p1;
  n->parents[1] = p2;
  return idx;
}

// Primitives wrappers that record checking global_tape.active
// These are what the builtins will call if simple scalar AD is used.
// For tensors, we'd need similar logic but with tensor pointers.

double gul_grad_add(double a, double b, int ia, int ib) {
  double res = a + b;
  return res;
  // Wait, we need to return the index if we are tracking indices?
  // The current GUL runtime passes values, not node indices.
  // This implies immediate mode AD requires value objects (structs) to hold
  // indices. Since GUL variables are just values (f64), we can't easily attach
  // gradients to them without changing the memory model of a 'float' to be a
  // struct { val, index }.
  //
  // ALTERNATIVE: The "Tape" just records operations on *values*?
  // No, Wengert lists need to identify which implementation variable
  // corresponds to which node.
  //
  // Given GUL's current stage (bare primitives), true AD requires wrapping
  // floats in a struct. We can't do that easily without changing the compiler
  // to emit struct usage for floats inside @grad blocks.
  //
  // COMPROMISE for Milestone 4:
  // We implement the Runtime Metadata part mainly for TENSORS (which are
  // pointers/structs). For Scalars, unless we boxed them, we can't track them.
  //
  // Let's assume this AD system is primarily for TENSORS or specific `Var`
  // objects. But the user example verifying `sin(x)` implies scalars.
  //
  // If we want scalar AD, we need a `GradientVar` struct.
  // Let's implement `gul_backward()` which assumes the last node is the loss.
}

// Scalar AD Helper
// Wrapper for a scalar that is tracked on the tape
typedef struct {
  double value;
  int index; // Index in the tape
} ScalarVar;

// Constructor
int64_t gul_make_var(double val) {
  ScalarVar *v = malloc(sizeof(ScalarVar));
  if (!v) {
    fprintf(stderr, "GUL Runtime: Out of memory in gul_make_var\n");
    exit(1);
  }
  if (global_tape.active) {
    v->index = tape_add_node(OP_NONE, val, -1, -1);
  } else {
    v->index = -1;
  }
  v->value = val;
  return (int64_t)v;
}

// Get value
double gul_var_val(int64_t handle) {
  if (!handle)
    return 0.0;
  return ((ScalarVar *)handle)->value;
}

// Get grad
double gul_var_grad(int64_t handle) {
  if (!handle)
    return 0.0;
  ScalarVar *v = (ScalarVar *)handle;
  if (v->index >= 0 && v->index < global_tape.count) {
    return global_tape.nodes[v->index].grad;
  }
  return 0.0;
}

// Operations
int64_t gul_var_add(int64_t a_ptr, int64_t b_ptr) {
  ScalarVar *a = (ScalarVar *)a_ptr;
  ScalarVar *b = (ScalarVar *)b_ptr;
  double val = a->value + b->value;

  ScalarVar *res = (ScalarVar *)gul_make_var(val);
  if (global_tape.active && a->index >= 0 && b->index >= 0) {
    ((ScalarVar *)res)->index = tape_add_node(OP_ADD, val, a->index, b->index);
  }
  return (int64_t)res;
}

int64_t gul_var_mul(int64_t a_ptr, int64_t b_ptr) {
  ScalarVar *a = (ScalarVar *)a_ptr;
  ScalarVar *b = (ScalarVar *)b_ptr;
  double val = a->value * b->value;

  ScalarVar *res = (ScalarVar *)gul_make_var(val);
  if (global_tape.active && a->index >= 0 && b->index >= 0) {
    ((ScalarVar *)res)->index = tape_add_node(OP_MUL, val, a->index, b->index);
  }
  return (int64_t)res;
}

int64_t gul_var_sin(int64_t a_ptr) {
  ScalarVar *a = (ScalarVar *)a_ptr;
  double val = sin(a->value);

  ScalarVar *res = (ScalarVar *)gul_make_var(val);
  if (global_tape.active && a->index >= 0) {
    ((ScalarVar *)res)->index = tape_add_node(OP_SIN, val, a->index, -1);
  }
  return (int64_t)res;
}

void gul_run_backward(int root_idx);

void gul_backward(int64_t root_ptr) {
  ScalarVar *root = (ScalarVar *)root_ptr;
  if (root && root->index >= 0) {
    gul_run_backward(root->index);
  }
}

// For now, let's implement the Tape for potential future use or for Tensor AD
// where we *do* have handles (int64_t pointers).

void gul_run_backward(int root_idx) {
  if (root_idx < 0 || root_idx >= global_tape.count)
    return;

  global_tape.nodes[root_idx].grad = 1.0;

  for (int i = root_idx; i >= 0; i--) {
    Node *n = &global_tape.nodes[i];
    if (n->grad == 0.0)
      continue;

    double g = n->grad;
    int p1 = n->parents[0];
    int p2 = n->parents[1];

    switch (n->op) {
    case OP_ADD:
      if (p1 >= 0)
        global_tape.nodes[p1].grad += g;
      if (p2 >= 0)
        global_tape.nodes[p2].grad += g;
      break;
    case OP_MUL:
      // z = x * y
      // dx = y * dz
      if (p1 >= 0)
        global_tape.nodes[p1].grad +=
            g * (p2 >= 0 ? global_tape.nodes[p2].value
                         : 0.0); // Wait, need value of p2
      if (p2 >= 0)
        global_tape.nodes[p2].grad +=
            g * (p1 >= 0 ? global_tape.nodes[p1].value : 0.0);
      break;
    case OP_SIN:
      // z = sin(x) -> dz/dx = cos(x)
      if (p1 >= 0)
        global_tape.nodes[p1].grad += g * cos(global_tape.nodes[p1].value);
      break;
    default:
      break;
    }
  }
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
// File I/O Operations (for data loading)
// ============================================================================

// Open file
int64_t gul_file_open(int64_t path_ptr, int64_t mode_ptr) {
  const char *path = (const char *)path_ptr;
  const char *mode = (const char *)mode_ptr;
  FILE *f = fopen(path, mode);
  if (!f)
    return 0;
  return (int64_t)f;
}

// Close file
void gul_file_close(int64_t file_ptr) {
  if (file_ptr)
    fclose((FILE *)file_ptr);
}

// Read line from file
int64_t gul_file_read_line(int64_t file_ptr) {
  if (!file_ptr)
    return 0;
  char *buffer = malloc(4096); // 4KB support for now
  if (fgets(buffer, 4096, (FILE *)file_ptr)) {
    // Remove trailing newline
    size_t len = strlen(buffer);
    if (len > 0 && buffer[len - 1] == '\n')
      buffer[len - 1] = '\0';
    return (int64_t)buffer;
  }
  free(buffer);
  return 0; // EOF or error
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

// ============================================================================
// Autograd - Automatic Differentiation
// Comprehensive Phase 4 tensor functions are at the end of this file

// Mean of all elements - Commented out, see comprehensive tensor ops below
// double gul_tensor_mean(int64_t ptr, int64_t size) {
//   return gul_tensor_sum(ptr, size) / (double)size;
// }

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

// ============================================================================
// String Utilities
// ============================================================================

// String Length
int64_t gul_string_len(int64_t s_ptr) {
  char *s = (char *)s_ptr;
  if (!s)
    return 0;
  return (int64_t)strlen(s);
}

// Substring
int64_t gul_string_substr(int64_t s_ptr, int64_t start, int64_t length) {
  char *s = (char *)s_ptr;
  if (!s)
    return (int64_t)"";

  size_t full_len = strlen(s);
  if (start < 0 || start >= full_len)
    return (int64_t)"";

  size_t actual_len = length;
  if (start + length > full_len) {
    actual_len = full_len - start;
  }

  char *res = malloc(actual_len + 1);
  if (!res) {
    fprintf(stderr, "GUL Runtime: Out of memory in substr\n");
    exit(1);
  }

  strncpy(res, s + start, actual_len);
  res[actual_len] = '\0';
  return (int64_t)res;
}

// Char at index (returns string of length 1)
int64_t gul_string_get(int64_t s_ptr, int64_t index) {
  return gul_string_substr(s_ptr, index, 1);
}

// Foreign Code Execution Stub
void gul_exec_foreign(int64_t lang_ptr, int64_t code_ptr) {
  char *lang = (char *)lang_ptr;
  char *code = (char *)code_ptr;
  if (!lang || !code)
    return;

  printf("--- Executing Foreign Code [%s] ---\n", lang);
  // In future: spawn process or use FFI
  // For now, just simulate execution via shell for verification
  if (strcmp(lang, "python") == 0) {
    char cmd[1024];
    // Only run if simple?
    snprintf(cmd, 1024, "python3 -c \"%s\"", code);
    printf("Running: %s\n", cmd);
    int res = system(cmd);
    if (res != 0)
      printf("Foreign code failed code: %d\n", res);
  } else {
    printf("%s\n", code);
  }
  printf("--- End Foreign Code ---\n");
}

// ============================================================================
// Table Operations (GUL v4.0)
// ============================================================================

typedef struct {
  char *name;
  double *values; // Assumes numeric for v3.3 strictness
} GulTableRow;

typedef struct {
  int col_count;
  int row_count;
  char **column_names;
  GulTableRow *rows;
} GulTable;

int64_t gul_table_alloc(int64_t col_count, int64_t row_count) {
  GulTable *t = (GulTable *)malloc(sizeof(GulTable));
  if (!t)
    exit(1);
  t->col_count = col_count;
  t->row_count = row_count;
  t->column_names = (char **)malloc(col_count * sizeof(char *));
  t->rows = (GulTableRow *)malloc(row_count * sizeof(GulTableRow));
  if (!t->column_names || !t->rows)
    exit(1);
  return (int64_t)t;
}

void gul_table_set_col_name(int64_t table_ptr, int64_t idx, int64_t name_ptr) {
  GulTable *t = (GulTable *)table_ptr;
  if (idx >= 0 && idx < t->col_count) {
    t->column_names[idx] = strdup((char *)name_ptr);
  }
}

void gul_table_set_row(int64_t table_ptr, int64_t idx, int64_t name_ptr,
                       int64_t values_ptr) {
  GulTable *t = (GulTable *)table_ptr;
  if (idx >= 0 && idx < t->row_count) {
    t->rows[idx].name = strdup((char *)name_ptr);
    t->rows[idx].values = (double *)values_ptr;
  }
}

int64_t gul_table_get_cell(int64_t table_ptr, int64_t row_idx,
                           int64_t col_idx) {
  GulTable *t = (GulTable *)table_ptr;
  if (row_idx >= 0 && row_idx < t->row_count && col_idx >= 0 &&
      col_idx < t->col_count) {
    double val = t->rows[row_idx].values[col_idx];
    // Return as int64 representation of the double bits
    int64_t *ptr = (int64_t *)&val;
    return *ptr;
  }
  return 0;
}

void gul_table_free(int64_t table_ptr) {
  GulTable *t = (GulTable *)table_ptr;
  if (!t)
    return;
  for (int i = 0; i < t->col_count; i++) {
    free(t->column_names[i]);
  }
  free(t->column_names);
  for (int i = 0; i < t->row_count; i++) {
    free(t->rows[i].name);
    free(t->rows[i].values);
  }
  free(t->rows);
  free(t);
}

// ============================================================================
// List Collection (GUL v3.3)
// ============================================================================

typedef struct {
  int64_t *data;
  int64_t len;
  int64_t capacity;
} GulList;

int64_t gul_list_alloc(int64_t initial_capacity) {
  GulList *list = (GulList *)malloc(sizeof(GulList));
  if (!list)
    exit(1);
  list->capacity = initial_capacity > 0 ? initial_capacity : 8;
  list->len = 0;
  list->data = (int64_t *)malloc(list->capacity * sizeof(int64_t));
  if (!list->data)
    exit(1);
  return (int64_t)list;
}

void gul_list_free(int64_t list_ptr) {
  GulList *list = (GulList *)list_ptr;
  if (list) {
    free(list->data);
    free(list);
  }
}

int64_t gul_list_len(int64_t list_ptr) {
  GulList *list = (GulList *)list_ptr;
  return list ? list->len : 0;
}

void gul_list_push(int64_t list_ptr, int64_t value) {
  GulList *list = (GulList *)list_ptr;
  if (!list)
    return;
  if (list->len >= list->capacity) {
    list->capacity *= 2;
    list->data =
        (int64_t *)realloc(list->data, list->capacity * sizeof(int64_t));
    if (!list->data)
      exit(1);
  }
  list->data[list->len++] = value;
}

int64_t gul_list_pop(int64_t list_ptr) {
  GulList *list = (GulList *)list_ptr;
  if (!list || list->len == 0)
    return 0;
  return list->data[--list->len];
}

int64_t gul_list_get(int64_t list_ptr, int64_t idx) {
  GulList *list = (GulList *)list_ptr;
  if (!list || idx < 0 || idx >= list->len)
    return 0;
  return list->data[idx];
}

void gul_list_set(int64_t list_ptr, int64_t idx, int64_t value) {
  GulList *list = (GulList *)list_ptr;
  if (!list || idx < 0 || idx >= list->len)
    return;
  list->data[idx] = value;
}

void gul_list_clear(int64_t list_ptr) {
  GulList *list = (GulList *)list_ptr;
  if (list)
    list->len = 0;
}

int64_t gul_list_contains(int64_t list_ptr, int64_t value) {
  GulList *list = (GulList *)list_ptr;
  if (!list)
    return 0;
  for (int64_t i = 0; i < list->len; i++) {
    if (list->data[i] == value)
      return 1;
  }
  return 0;
}

void gul_list_insert_before(int64_t list_ptr, int64_t idx, int64_t value) {
  GulList *list = (GulList *)list_ptr;
  if (!list || idx < 0 || idx > list->len)
    return;
  if (list->len >= list->capacity) {
    list->capacity *= 2;
    list->data =
        (int64_t *)realloc(list->data, list->capacity * sizeof(int64_t));
  }
  memmove(&list->data[idx + 1], &list->data[idx],
          (list->len - idx) * sizeof(int64_t));
  list->data[idx] = value;
  list->len++;
}

void gul_list_insert_after(int64_t list_ptr, int64_t idx, int64_t value) {
  gul_list_insert_before(list_ptr, idx + 1, value);
}

void gul_list_remove(int64_t list_ptr, int64_t idx) {
  GulList *list = (GulList *)list_ptr;
  if (!list || idx < 0 || idx >= list->len)
    return;
  memmove(&list->data[idx], &list->data[idx + 1],
          (list->len - idx - 1) * sizeof(int64_t));
  list->len--;
}

// ============================================================================
// Dict Collection (GUL v3.3) - Simple hash map
// ============================================================================

typedef struct {
  char *key;
  int64_t value;
  int used;
} GulDictEntry;

typedef struct {
  GulDictEntry *entries;
  int64_t capacity;
  int64_t len;
} GulDict;

static uint64_t dict_hash(const char *key) {
  uint64_t h = 5381;
  while (*key) {
    h = ((h << 5) + h) + *key++;
  }
  return h;
}

int64_t gul_dict_alloc(int64_t capacity) {
  GulDict *dict = (GulDict *)malloc(sizeof(GulDict));
  if (!dict)
    exit(1);
  dict->capacity = capacity > 0 ? capacity : 16;
  dict->len = 0;
  dict->entries = (GulDictEntry *)calloc(dict->capacity, sizeof(GulDictEntry));
  if (!dict->entries)
    exit(1);
  return (int64_t)dict;
}

void gul_dict_free(int64_t dict_ptr) {
  GulDict *dict = (GulDict *)dict_ptr;
  if (dict) {
    for (int64_t i = 0; i < dict->capacity; i++) {
      if (dict->entries[i].used)
        free(dict->entries[i].key);
    }
    free(dict->entries);
    free(dict);
  }
}

int64_t gul_dict_len(int64_t dict_ptr) {
  GulDict *dict = (GulDict *)dict_ptr;
  return dict ? dict->len : 0;
}

void gul_dict_set(int64_t dict_ptr, int64_t key_ptr, int64_t value) {
  GulDict *dict = (GulDict *)dict_ptr;
  if (!dict)
    return;
  char *key = (char *)key_ptr;
  uint64_t h = dict_hash(key) % dict->capacity;

  for (int64_t i = 0; i < dict->capacity; i++) {
    int64_t idx = (h + i) % dict->capacity;
    if (!dict->entries[idx].used) {
      dict->entries[idx].key = strdup(key);
      dict->entries[idx].value = value;
      dict->entries[idx].used = 1;
      dict->len++;
      return;
    }
    if (strcmp(dict->entries[idx].key, key) == 0) {
      dict->entries[idx].value = value;
      return;
    }
  }
}

int64_t gul_dict_get(int64_t dict_ptr, int64_t key_ptr) {
  GulDict *dict = (GulDict *)dict_ptr;
  if (!dict)
    return 0;
  char *key = (char *)key_ptr;
  uint64_t h = dict_hash(key) % dict->capacity;

  for (int64_t i = 0; i < dict->capacity; i++) {
    int64_t idx = (h + i) % dict->capacity;
    if (!dict->entries[idx].used)
      return 0;
    if (strcmp(dict->entries[idx].key, key) == 0) {
      return dict->entries[idx].value;
    }
  }
  return 0;
}

int64_t gul_dict_contains(int64_t dict_ptr, int64_t key_ptr) {
  GulDict *dict = (GulDict *)dict_ptr;
  if (!dict)
    return 0;
  char *key = (char *)key_ptr;
  uint64_t h = dict_hash(key) % dict->capacity;

  for (int64_t i = 0; i < dict->capacity; i++) {
    int64_t idx = (h + i) % dict->capacity;
    if (!dict->entries[idx].used)
      return 0;
    if (strcmp(dict->entries[idx].key, key) == 0)
      return 1;
  }
  return 0;
}

void gul_dict_remove(int64_t dict_ptr, int64_t key_ptr) {
  GulDict *dict = (GulDict *)dict_ptr;
  if (!dict)
    return;
  char *key = (char *)key_ptr;
  uint64_t h = dict_hash(key) % dict->capacity;

  for (int64_t i = 0; i < dict->capacity; i++) {
    int64_t idx = (h + i) % dict->capacity;
    if (!dict->entries[idx].used)
      return;
    if (strcmp(dict->entries[idx].key, key) == 0) {
      free(dict->entries[idx].key);
      dict->entries[idx].used = 0;
      dict->len--;
      return;
    }
  }
}

void gul_dict_clear(int64_t dict_ptr) {
  GulDict *dict = (GulDict *)dict_ptr;
  if (!dict)
    return;
  for (int64_t i = 0; i < dict->capacity; i++) {
    if (dict->entries[i].used) {
      free(dict->entries[i].key);
      dict->entries[i].used = 0;
    }
  }
  dict->len = 0;
}

// ============================================================================
// Set Collection (GUL v3.3) - Simple hash set
// ============================================================================

typedef struct {
  int64_t *values;
  int *used;
  int64_t capacity;
  int64_t len;
} GulSet;

int64_t gul_set_alloc(int64_t capacity) {
  GulSet *set = (GulSet *)malloc(sizeof(GulSet));
  if (!set)
    exit(1);
  set->capacity = capacity > 0 ? capacity : 16;
  set->len = 0;
  set->values = (int64_t *)malloc(set->capacity * sizeof(int64_t));
  set->used = (int *)calloc(set->capacity, sizeof(int));
  if (!set->values || !set->used)
    exit(1);
  return (int64_t)set;
}

void gul_set_free(int64_t set_ptr) {
  GulSet *set = (GulSet *)set_ptr;
  if (set) {
    free(set->values);
    free(set->used);
    free(set);
  }
}

int64_t gul_set_len(int64_t set_ptr) {
  GulSet *set = (GulSet *)set_ptr;
  return set ? set->len : 0;
}

void gul_set_add(int64_t set_ptr, int64_t value) {
  GulSet *set = (GulSet *)set_ptr;
  if (!set)
    return;
  uint64_t h = (uint64_t)value % set->capacity;

  for (int64_t i = 0; i < set->capacity; i++) {
    int64_t idx = (h + i) % set->capacity;
    if (!set->used[idx]) {
      set->values[idx] = value;
      set->used[idx] = 1;
      set->len++;
      return;
    }
    if (set->values[idx] == value)
      return; // Already exists
  }
}

int64_t gul_set_contains(int64_t set_ptr, int64_t value) {
  GulSet *set = (GulSet *)set_ptr;
  if (!set)
    return 0;
  uint64_t h = (uint64_t)value % set->capacity;

  for (int64_t i = 0; i < set->capacity; i++) {
    int64_t idx = (h + i) % set->capacity;
    if (!set->used[idx])
      return 0;
    if (set->values[idx] == value)
      return 1;
  }
  return 0;
}

void gul_set_remove(int64_t set_ptr, int64_t value) {
  GulSet *set = (GulSet *)set_ptr;
  if (!set)
    return;
  uint64_t h = (uint64_t)value % set->capacity;

  for (int64_t i = 0; i < set->capacity; i++) {
    int64_t idx = (h + i) % set->capacity;
    if (!set->used[idx])
      return;
    if (set->values[idx] == value) {
      set->used[idx] = 0;
      set->len--;
      return;
    }
  }
}

void gul_set_clear(int64_t set_ptr) {
  GulSet *set = (GulSet *)set_ptr;
  if (!set)
    return;
  memset(set->used, 0, set->capacity * sizeof(int));
  set->len = 0;
}

// ============================================================================
// Memory Allocation
// ============================================================================

int64_t gul_malloc(int64_t size) { return (int64_t)malloc(size); }
void gul_free(int64_t ptr) { free((void *)ptr); }

// ============================================================================
// Channel Operations (Phase 2: Data-Flow & Concurrency)
// ============================================================================

typedef struct {
  int64_t *buffer;
  int64_t capacity;
  int64_t len;
  int64_t head;
  int64_t tail;
} GulChannel;

int64_t gul_chan_create(int64_t capacity) {
  GulChannel *ch = (GulChannel *)malloc(sizeof(GulChannel));
  if (!ch) {
    fprintf(stderr, "GUL Runtime: Out of memory in chan_create\n");
    exit(1);
  }
  ch->buffer = (int64_t *)malloc(capacity * sizeof(int64_t));
  if (!ch->buffer) {
    fprintf(stderr, "GUL Runtime: Out of memory allocating channel buffer\n");
    exit(1);
  }
  ch->capacity = capacity;
  ch->len = 0;
  ch->head = 0;
  ch->tail = 0;
  return (int64_t)ch;
}

void gul_chan_send(int64_t chan_ptr, int64_t value) {
  GulChannel *ch = (GulChannel *)chan_ptr;
  if (!ch)
    return;

  // Simple blocking behavior: if full, overwrite oldest
  if (ch->len >= ch->capacity) {
    ch->buffer[ch->tail] = value;
    ch->tail = (ch->tail + 1) % ch->capacity;
    ch->head = (ch->head + 1) % ch->capacity;
  } else {
    ch->buffer[ch->tail] = value;
    ch->tail = (ch->tail + 1) % ch->capacity;
    ch->len++;
  }
}

int64_t gul_chan_recv(int64_t chan_ptr) {
  GulChannel *ch = (GulChannel *)chan_ptr;
  if (!ch || ch->len == 0)
    return 0;

  int64_t value = ch->buffer[ch->head];
  ch->head = (ch->head + 1) % ch->capacity;
  ch->len--;
  return value;
}

int64_t gul_chan_len(int64_t chan_ptr) {
  GulChannel *ch = (GulChannel *)chan_ptr;
  return ch ? ch->len : 0;
}

void gul_chan_close(int64_t chan_ptr) {
  GulChannel *ch = (GulChannel *)chan_ptr;
  if (!ch)
    return;
  free(ch->buffer);
  free(ch);
}

// ============================================================================
// DataFrame Operations (Phase 3: Data Analysis)
// ============================================================================

typedef struct {
  int64_t n_rows;
  int64_t n_cols;
  char **column_names;
  int64_t **data; // Array of column arrays
} GulDataFrame;

int64_t gul_frame_create(int64_t n_rows, int64_t n_cols) {
  GulDataFrame *df = (GulDataFrame *)malloc(sizeof(GulDataFrame));
  if (!df) {
    fprintf(stderr, "GUL Runtime: Out of memory in frame_create\n");
    exit(1);
  }

  df->n_rows = n_rows;
  df->n_cols = n_cols;
  df->column_names = (char **)malloc(n_cols * sizeof(char *));
  df->data = (int64_t **)malloc(n_cols * sizeof(int64_t *));

  for (int64_t i = 0; i < n_cols; i++) {
    df->column_names[i] = NULL;
    df->data[i] = (int64_t *)malloc(n_rows * sizeof(int64_t));
    if (!df->data[i]) {
      fprintf(stderr, "GUL Runtime: Out of memory allocating column\n");
      exit(1);
    }
  }

  return (int64_t)df;
}

void gul_frame_set_column_name(int64_t df_ptr, int64_t col_idx,
                               int64_t name_ptr) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df || col_idx < 0 || col_idx >= df->n_cols)
    return;

  if (df->column_names[col_idx]) {
    free(df->column_names[col_idx]);
  }
  df->column_names[col_idx] = strdup((char *)name_ptr);
}

void gul_frame_set_cell(int64_t df_ptr, int64_t row, int64_t col,
                        int64_t value) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df || row < 0 || row >= df->n_rows || col < 0 || col >= df->n_cols)
    return;
  df->data[col][row] = value;
}

int64_t gul_frame_get_cell(int64_t df_ptr, int64_t row, int64_t col) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df || row < 0 || row >= df->n_rows || col < 0 || col >= df->n_cols)
    return 0;
  return df->data[col][row];
}

int64_t gul_frame_get_column(int64_t df_ptr, int64_t col_idx) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df || col_idx < 0 || col_idx >= df->n_cols)
    return 0;
  return (int64_t)df->data[col_idx];
}

int64_t gul_frame_filter(int64_t df_ptr, int64_t (*predicate)(int64_t)) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df)
    return 0;

  // Count matching rows
  int64_t count = 0;
  for (int64_t i = 0; i < df->n_rows; i++) {
    if (predicate(i))
      count++;
  }

  // Create new DataFrame with filtered rows
  int64_t new_df_ptr = gul_frame_create(count, df->n_cols);
  GulDataFrame *new_df = (GulDataFrame *)new_df_ptr;

  // Copy column names
  for (int64_t c = 0; c < df->n_cols; c++) {
    if (df->column_names[c]) {
      gul_frame_set_column_name(new_df_ptr, c, (int64_t)df->column_names[c]);
    }
  }

  // Copy matching rows
  int64_t new_row = 0;
  for (int64_t i = 0; i < df->n_rows; i++) {
    if (predicate(i)) {
      for (int64_t c = 0; c < df->n_cols; c++) {
        new_df->data[c][new_row] = df->data[c][i];
      }
      new_row++;
    }
  }

  return new_df_ptr;
}

void gul_frame_free(int64_t df_ptr) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df)
    return;

  for (int64_t i = 0; i < df->n_cols; i++) {
    if (df->column_names[i])
      free(df->column_names[i]);
    if (df->data[i])
      free(df->data[i]);
  }
  free(df->column_names);
  free(df->data);
  free(df);
}

// ============================================================================
// Tensor Operations (Phase 4: Machine Learning)
// ============================================================================

typedef struct {
  double *data;
  int64_t *shape;
  int64_t ndim;
  int64_t size; // Total elements
  int64_t *strides;
} GulTensor;

int64_t gul_tensor_create(int64_t *shape_ptr, int64_t ndim) {
  GulTensor *t = (GulTensor *)malloc(sizeof(GulTensor));
  if (!t) {
    fprintf(stderr, "GUL Runtime: Out of memory in tensor_create\n");
    exit(1);
  }

  t->ndim = ndim;
  t->shape = (int64_t *)malloc(ndim * sizeof(int64_t));
  t->strides = (int64_t *)malloc(ndim * sizeof(int64_t));

  // Calculate size and strides
  t->size = 1;
  for (int64_t i = 0; i < ndim; i++) {
    t->shape[i] = shape_ptr[i];
    t->size *= shape_ptr[i];
  }

  // Calculate strides (row-major)
  t->strides[ndim - 1] = 1;
  for (int64_t i = ndim - 2; i >= 0; i--) {
    t->strides[i] = t->strides[i + 1] * t->shape[i + 1];
  }

  t->data = (double *)malloc(t->size * sizeof(double));
  if (!t->data) {
    fprintf(stderr, "GUL Runtime: Out of memory allocating tensor data\n");
    exit(1);
  }

  // Initialize to zeros
  for (int64_t i = 0; i < t->size; i++) {
    t->data[i] = 0.0;
  }

  return (int64_t)t;
}

int64_t gul_tensor_zeros(int64_t *shape_ptr, int64_t ndim) {
  return gul_tensor_create(shape_ptr, ndim);
}

int64_t gul_tensor_ones(int64_t *shape_ptr, int64_t ndim) {
  int64_t t_ptr = gul_tensor_create(shape_ptr, ndim);
  GulTensor *t = (GulTensor *)t_ptr;
  for (int64_t i = 0; i < t->size; i++) {
    t->data[i] = 1.0;
  }
  return t_ptr;
}

void gul_tensor_fill(int64_t t_ptr, double value) {
  GulTensor *t = (GulTensor *)t_ptr;
  if (!t)
    return;
  for (int64_t i = 0; i < t->size; i++) {
    t->data[i] = value;
  }
}

double gul_tensor_get(int64_t t_ptr, int64_t *indices) {
  GulTensor *t = (GulTensor *)t_ptr;
  if (!t)
    return 0.0;

  int64_t idx = 0;
  for (int64_t i = 0; i < t->ndim; i++) {
    idx += indices[i] * t->strides[i];
  }
  return t->data[idx];
}

void gul_tensor_set(int64_t t_ptr, int64_t *indices, double value) {
  GulTensor *t = (GulTensor *)t_ptr;
  if (!t)
    return;

  int64_t idx = 0;
  for (int64_t i = 0; i < t->ndim; i++) {
    idx += indices[i] * t->strides[i];
  }
  t->data[idx] = value;
}

// Element-wise tensor addition
int64_t gul_tensor_add(int64_t a_ptr, int64_t b_ptr) {
  GulTensor *a = (GulTensor *)a_ptr;
  GulTensor *b = (GulTensor *)b_ptr;
  if (!a || !b || a->size != b->size)
    return 0;

  int64_t result_ptr = gul_tensor_create(a->shape, a->ndim);
  GulTensor *result = (GulTensor *)result_ptr;

  for (int64_t i = 0; i < a->size; i++) {
    result->data[i] = a->data[i] + b->data[i];
  }

  return result_ptr;
}

// Element-wise tensor multiplication
int64_t gul_tensor_mul(int64_t a_ptr, int64_t b_ptr) {
  GulTensor *a = (GulTensor *)a_ptr;
  GulTensor *b = (GulTensor *)b_ptr;
  if (!a || !b || a->size != b->size)
    return 0;

  int64_t result_ptr = gul_tensor_create(a->shape, a->ndim);
  GulTensor *result = (GulTensor *)result_ptr;

  for (int64_t i = 0; i < a->size; i++) {
    result->data[i] = a->data[i] * b->data[i];
  }

  return result_ptr;
}

// Matrix multiplication for 2D tensors
int64_t gul_tensor_matmul(int64_t a_ptr, int64_t b_ptr) {
  GulTensor *a = (GulTensor *)a_ptr;
  GulTensor *b = (GulTensor *)b_ptr;

  if (!a || !b || a->ndim != 2 || b->ndim != 2)
    return 0;
  if (a->shape[1] != b->shape[0])
    return 0;

  int64_t m = a->shape[0];
  int64_t k = a->shape[1];
  int64_t n = b->shape[1];

  int64_t result_shape[2] = {m, n};
  int64_t result_ptr = gul_tensor_create(result_shape, 2);
  GulTensor *result = (GulTensor *)result_ptr;

  for (int64_t i = 0; i < m; i++) {
    for (int64_t j = 0; j < n; j++) {
      double sum = 0.0;
      for (int64_t l = 0; l < k; l++) {
        sum += a->data[i * k + l] * b->data[l * n + j];
      }
      result->data[i * n + j] = sum;
    }
  }

  return result_ptr;
}

int64_t gul_tensor_reshape(int64_t t_ptr, int64_t *new_shape,
                           int64_t new_ndim) {
  GulTensor *t = (GulTensor *)t_ptr;
  if (!t)
    return 0;

  // Verify total size matches
  int64_t new_size = 1;
  for (int64_t i = 0; i < new_ndim; i++) {
    new_size *= new_shape[i];
  }
  if (new_size != t->size)
    return 0;

  int64_t new_t_ptr = gul_tensor_create(new_shape, new_ndim);
  GulTensor *new_t = (GulTensor *)new_t_ptr;

  // Copy data
  for (int64_t i = 0; i < t->size; i++) {
    new_t->data[i] = t->data[i];
  }

  return new_t_ptr;
}

void gul_tensor_free(int64_t t_ptr) {
  GulTensor *t = (GulTensor *)t_ptr;
  if (!t)
    return;
  free(t->data);
  free(t->shape);
  free(t->strides);
  free(t);
}

// Gradient tracking enhancement for tensors
typedef struct {
  int64_t tensor_ptr;
  double *grad;
  int requires_grad;
} GulGradTensor;

int64_t gul_grad_tensor_create(int64_t tensor_ptr) {
  GulTensor *t = (GulTensor *)tensor_ptr;
  if (!t)
    return 0;

  GulGradTensor *gt = (GulGradTensor *)malloc(sizeof(GulGradTensor));
  if (!gt)
    return 0;

  gt->tensor_ptr = tensor_ptr;
  gt->grad = (double *)calloc(t->size, sizeof(double));
  gt->requires_grad = 1;

  return (int64_t)gt;
}

void gul_grad_tensor_backward(int64_t gt_ptr) {
  GulGradTensor *gt = (GulGradTensor *)gt_ptr;
  if (!gt)
    return;

  GulTensor *t = (GulTensor *)gt->tensor_ptr;
  if (!t)
    return;

  // Initialize gradient of output as 1.0
  gt->grad[0] = 1.0;
  // Full backpropagation would be implemented here
}

int64_t gul_grad_tensor_get_grad(int64_t gt_ptr) {
  GulGradTensor *gt = (GulGradTensor *)gt_ptr;
  if (!gt)
    return 0;
  return (int64_t)gt->grad;
}

void gul_grad_tensor_free(int64_t gt_ptr) {
  GulGradTensor *gt = (GulGradTensor *)gt_ptr;
  if (!gt)
    return;
  free(gt->grad);
  free(gt);
}
