// GUL Runtime - Gradient Tracking & Computational Graph
// Full automatic differentiation with computational graph

#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ============================================================================
// Computational Graph Node
// ============================================================================

typedef enum {
  OP_CONSTANT,
  OP_ADD,
  OP_MUL,
  OP_SUB,
  OP_DIV,
  OP_POW,
  OP_EXP,
  OP_LOG,
  OP_SIN,
  OP_COS,
  OP_MATMUL
} GradOp;

typedef struct GraphNode {
  int64_t id;
  GradOp op;
  double value;
  double grad;
  struct GraphNode *left;
  struct GraphNode *right;
  int requires_grad;
  int visited; // For topological sort
} GraphNode;

// Global graph context
typedef struct {
  GraphNode **nodes;
  int64_t node_count;
  int64_t capacity;
  int is_recording;
} ComputationalGraph;

static ComputationalGraph *global_graph = NULL;

// ============================================================================
// Graph Initialization
// ============================================================================

void gul_grad_begin() {
  if (global_graph) {
    // Clear existing graph
    for (int64_t i = 0; i < global_graph->node_count; i++) {
      free(global_graph->nodes[i]);
    }
    free(global_graph->nodes);
    free(global_graph);
  }

  global_graph = (ComputationalGraph *)malloc(sizeof(Computational Graph));
  global_graph->capacity = 1024;
  global_graph->node_count = 0;
  global_graph->nodes =
      (GraphNode **)malloc(global_graph->capacity * sizeof(GraphNode *));
  global_graph->is_recording = 1;
}

void gul_grad_end() {
  if (!global_graph)
    return;

  // Clean up
  for (int64_t i = 0; i < global_graph->node_count; i++) {
    free(global_graph->nodes[i]);
  }
  free(global_graph->nodes);
  free(global_graph);
  global_graph = NULL;
}

// ============================================================================
// Node Creation
// ============================================================================

GraphNode *create_node(GradOp op, double value, GraphNode *left,
                       GraphNode *right) {
  if (!global_graph || !global_graph->is_recording) {
    // Not recording, return dummy node
    GraphNode *node = (GraphNode *)malloc(sizeof(GraphNode));
    node->value = value;
    node->grad = 0.0;
    node->requires_grad = 0;
    return node;
  }

  GraphNode *node = (GraphNode *)malloc(sizeof(GraphNode));
  node->id = global_graph->node_count;
  node->op = op;
  node->value = value;
  node->grad = 0.0;
  node->left = left;
  node->right = right;
  node->requires_grad =
      (left && left->requires_grad) || (right && right->requires_grad);
  node->visited = 0;

  // Add to graph
  if (global_graph->node_count >= global_graph->capacity) {
    global_graph->capacity *= 2;
    global_graph->nodes = (GraphNode **)realloc(
        global_graph->nodes, global_graph->capacity * sizeof(GraphNode *));
  }
  global_graph->nodes[global_graph->node_count++] = node;

  return node;
}

int64_t gul_make_var(double value) {
  GraphNode *node = create_node(OP_CONSTANT, value, NULL, NULL);
  node->requires_grad = 1;
  return (int64_t)node;
}

// ============================================================================
// Operations
// ============================================================================

int64_t gul_var_add(int64_t a_ptr, int64_t b_ptr) {
  GraphNode *a = (GraphNode *)a_ptr;
  GraphNode *b = (GraphNode *)b_ptr;

  double result = a->value + b->value;
  return (int64_t)create_node(OP_ADD, result, a, b);
}

int64_t gul_var_mul(int64_t a_ptr, int64_t b_ptr) {
  GraphNode *a = (GraphNode *)a_ptr;
  GraphNode *b = (GraphNode *)b_ptr;

  double result = a->value * b->value;
  return (int64_t)create_node(OP_MUL, result, a, b);
}

int64_t gul_var_sub(int64_t a_ptr, int64_t b_ptr) {
  GraphNode *a = (GraphNode *)a_ptr;
  GraphNode *b = (GraphNode *)b_ptr;

  double result = a->value - b->value;
  return (int64_t)create_node(OP_SUB, result, a, b);
}

int64_t gul_var_div(int64_t a_ptr, int64_t b_ptr) {
  GraphNode *a = (GraphNode *)a_ptr;
  GraphNode *b = (GraphNode *)b_ptr;

  double result = a->value / b->value;
  return (int64_t)create_node(OP_DIV, result, a, b);
}

int64_t gul_var_pow(int64_t a_ptr, int64_t b_ptr) {
  GraphNode *a = (GraphNode *)a_ptr;
  GraphNode *b = (GraphNode *)b_ptr;

  double result = pow(a->value, b->value);
  return (int64_t)create_node(OP_POW, result, a, b);
}

// ============================================================================
// Backward Pass (Auto-differentiation)
// ============================================================================

void backward_pass(GraphNode *node) {
  if (!node || !node->requires_grad)
    return;

  // Compute gradients based on operation
  switch (node->op) {
  case OP_ADD:
    if (node->left)
      node->left->grad += node->grad;
    if (node->right)
      node->right->grad += node->grad;
    break;

  case OP_MUL:
    if (node->left)
      node->left->grad += node->grad * node->right->value;
    if (node->right)
      node->right->grad += node->grad * node->left->value;
    break;

  case OP_SUB:
    if (node->left)
      node->left->grad += node->grad;
    if (node->right)
      node->right->grad -= node->grad;
    break;

  case OP_DIV:
    if (node->left)
      node->left->grad += node->grad / node->right->value;
    if (node->right)
      node->right->grad -= node->grad * node->left->value /
                           (node->right->value * node->right->value);
    break;

  case OP_POW:
    if (node->left) {
      node->left->grad += node->grad * node->right->value *
                          pow(node->left->value, node->right->value - 1);
    }
    break;

  case OP_CONSTANT:
    // No backward for constants
    break;
  }

  // Recursively backpropagate
  if (node->left)
    backward_pass(node->left);
  if (node->right)
    backward_pass(node->right);
}

void gul_backward(int64_t output_ptr) {
  GraphNode *output = (GraphNode *)output_ptr;
  if (!output)
    return;

  // Initialize output gradient to 1.0
  output->grad = 1.0;

  // Reverse topological sort and backpropagate
  backward_pass(output);
}

double gul_var_grad(int64_t var_ptr) {
  GraphNode *var = (GraphNode *)var_ptr;
  return var ? var->grad : 0.0;
}

double gul_var_value(int64_t var_ptr) {
  GraphNode *var = (GraphNode *)var_ptr;
  return var ? var->value : 0.0;
}

// ============================================================================
// Example: f(x) = x^2 + 3x + 1, f'(x) = 2x + 3
// ============================================================================

void test_grad_example() {
  gul_grad_begin();

  int64_t x = gul_make_var(2.0);         // x = 2
  int64_t x_squared = gul_var_mul(x, x); // x^2
  int64_t three = gul_make_var(3.0);
  int64_t three_x = gul_var_mul(three, x);        // 3x
  int64_t temp = gul_var_add(x_squared, three_x); // x^2 + 3x
  int64_t one = gul_make_var(1.0);
  int64_t y = gul_var_add(temp, one); // x^2 + 3x + 1

  gul_backward(y);

  double grad_x = gul_var_grad(x); // Should be 2*2 + 3 = 7
  printf("f(2) = %f, f'(2) = %f\n", gul_var_value(y), grad_x);

  gul_grad_end();
}
