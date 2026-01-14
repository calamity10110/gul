// GUL Runtime - Advanced DataFrame Operations
// Implements group_by, join, and aggregation operations

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Existing GulDataFrame structure (assumed to be defined in stdlib.c)
// typedef struct {
//     int64_t n_rows;
//     int64_t n_cols;
//     char **column_names;
//     int64_t **data;
// } GulDataFrame;

// Forward declarations
extern int64_t gul_frame_create(int64_t n_rows, int64_t n_cols);
extern void gul_frame_set_cell(int64_t df_ptr, int64_t row, int64_t col,
                               int64_t value);
extern int64_t gul_frame_get_cell(int64_t df_ptr, int64_t row, int64_t col);

// ============================================================================
// GROUP BY Operation
// ============================================================================

typedef struct {
  int64_t key;
  int64_t *row_indices;
  int64_t count;
  int64_t capacity;
} GroupInfo;

typedef struct {
  GroupInfo *groups;
  int64_t group_count;
  int64_t capacity;
} GroupByResult;

int64_t gul_frame_group_by(int64_t df_ptr, int64_t col_idx) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df || col_idx < 0 || col_idx >= df->n_cols)
    return 0;

  // Create hash map for groups
  GroupByResult *result = (GroupByResult *)malloc(sizeof(GroupByResult));
  result->capacity = 16;
  result->group_count = 0;
  result->groups = (GroupInfo *)calloc(result->capacity, sizeof(GroupInfo));

  // Group rows by column value
  for (int64_t row = 0; row < df->n_rows; row++) {
    int64_t key = df->data[col_idx][row];

    // Find or create group
    int64_t group_idx = -1;
    for (int64_t g = 0; g < result->group_count; g++) {
      if (result->groups[g].key == key) {
        group_idx = g;
        break;
      }
    }

    if (group_idx == -1) {
      // New group
      group_idx = result->group_count++;
      if (result->group_count > result->capacity) {
        result->capacity *= 2;
        result->groups = (GroupInfo *)realloc(
            result->groups, result->capacity * sizeof(GroupInfo));
      }

      result->groups[group_idx].key = key;
      result->groups[group_idx].capacity = 8;
      result->groups[group_idx].count = 0;
      result->groups[group_idx].row_indices = (int64_t *)malloc(
          result->groups[group_idx].capacity * sizeof(int64_t));
    }

    // Add row to group
    GroupInfo *group = &result->groups[group_idx];
    if (group->count >= group->capacity) {
      group->capacity *= 2;
      group->row_indices = (int64_t *)realloc(
          group->row_indices, group->capacity * sizeof(int64_t));
    }
    group->row_indices[group->count++] = row;
  }

  return (int64_t)result;
}

// ============================================================================
// AGGREGATION Operations
// ============================================================================

int64_t gul_frame_agg_sum(int64_t df_ptr, int64_t col_idx) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df || col_idx < 0 || col_idx >= df->n_cols)
    return 0;

  int64_t sum = 0;
  for (int64_t row = 0; row < df->n_rows; row++) {
    sum += df->data[col_idx][row];
  }
  return sum;
}

double gul_frame_agg_mean(int64_t df_ptr, int64_t col_idx) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df || col_idx < 0 || col_idx >= df->n_cols)
    return 0.0;

  int64_t sum = gul_frame_agg_sum(df_ptr, col_idx);
  return (double)sum / (double)df->n_rows;
}

int64_t gul_frame_agg_min(int64_t df_ptr, int64_t col_idx) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df || col_idx < 0 || col_idx >= df->n_cols || df->n_rows == 0)
    return 0;

  int64_t min_val = df->data[col_idx][0];
  for (int64_t row = 1; row < df->n_rows; row++) {
    int64_t val = df->data[col_idx][row];
    if (val < min_val)
      min_val = val;
  }
  return min_val;
}

int64_t gul_frame_agg_max(int64_t df_ptr, int64_t col_idx) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  if (!df || col_idx < 0 || col_idx >= df->n_cols || df->n_rows == 0)
    return 0;

  int64_t max_val = df->data[col_idx][0];
  for (int64_t row = 1; row < df->n_rows; row++) {
    int64_t val = df->data[col_idx][row];
    if (val > max_val)
      max_val = val;
  }
  return max_val;
}

int64_t gul_frame_agg_count(int64_t df_ptr) {
  GulDataFrame *df = (GulDataFrame *)df_ptr;
  return df ? df->n_rows : 0;
}

// ============================================================================
// JOIN Operation
// ============================================================================

int64_t gul_frame_join(int64_t left_ptr, int64_t right_ptr, int64_t left_col,
                       int64_t right_col) {
  GulDataFrame *left = (GulDataFrame *)left_ptr;
  GulDataFrame *right = (GulDataFrame *)right_ptr;

  if (!left || !right)
    return 0;

  // Inner join implementation
  // Count matching rows
  int64_t match_count = 0;
  for (int64_t i = 0; i < left->n_rows; i++) {
    int64_t left_val = left->data[left_col][i];
    for (int64_t j = 0; j < right->n_rows; j++) {
      if (left_val == right->data[right_col][j]) {
        match_count++;
      }
    }
  }

  // Create result DataFrame
  int64_t result_cols = left->n_cols + right->n_cols;
  int64_t result_ptr = gul_frame_create(match_count, result_cols);
  GulDataFrame *result = (GulDataFrame *)result_ptr;

  // Set column names
  for (int64_t c = 0; c < left->n_cols; c++) {
    if (left->column_names[c]) {
      result->column_names[c] = strdup(left->column_names[c]);
    }
  }
  for (int64_t c = 0; c < right->n_cols; c++) {
    if (right->column_names[c]) {
      result->column_names[left->n_cols + c] = strdup(right->column_names[c]);
    }
  }

  // Fill result with matched rows
  int64_t result_row = 0;
  for (int64_t i = 0; i < left->n_rows; i++) {
    int64_t left_val = left->data[left_col][i];
    for (int64_t j = 0; j < right->n_rows; j++) {
      if (left_val == right->data[right_col][j]) {
        // Copy left columns
        for (int64_t c = 0; c < left->n_cols; c++) {
          result->data[c][result_row] = left->data[c][i];
        }
        // Copy right columns
        for (int64_t c = 0; c < right->n_cols; c++) {
          result->data[left->n_cols + c][result_row] = right->data[c][j];
        }
        result_row++;
      }
    }
  }

  return result_ptr;
}

// ============================================================================
// Free GroupBy Result
// ============================================================================

void gul_frame_free_groupby(int64_t groupby_ptr) {
  GroupByResult *result = (GroupByResult *)groupby_ptr;
  if (!result)
    return;

  for (int64_t g = 0; g < result->group_count; g++) {
    free(result->groups[g].row_indices);
  }
  free(result->groups);
  free(result);
}
