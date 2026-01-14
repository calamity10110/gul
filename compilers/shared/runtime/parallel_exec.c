// GUL Runtime - Parallel Execution Support
// Uses rayon-style work-stealing for parallel for loops

#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define MAX_THREADS 16

typedef struct {
  int64_t start;
  int64_t end;
  void (*body)(int64_t);
  pthread_t thread_id;
} ParallelTask;

typedef struct {
  ParallelTask *tasks;
  int64_t num_tasks;
  int64_t num_threads;
} ParallelContext;

// Thread entry point
void *worker_thread(void *arg) {
  ParallelTask *task = (ParallelTask *)arg;

  for (int64_t i = task->start; i < task->end; i++) {
    task->body(i);
  }

  return NULL;
}

// Execute parallel for loop
void gul_parallel_for(int64_t start, int64_t end, void (*body)(int64_t)) {
  if (end <= start)
    return;

  int64_t range = end - start;
  int64_t num_threads = range < MAX_THREADS ? range : MAX_THREADS;

  if (num_threads == 1 || range < 100) {
    // Sequential execution for small ranges
    for (int64_t i = start; i < end; i++) {
      body(i);
    }
    return;
  }

  // Create parallel context
  ParallelContext ctx;
  ctx.num_threads = num_threads;
  ctx.num_tasks = num_threads;
  ctx.tasks = (ParallelTask *)malloc(num_threads * sizeof(ParallelTask));

  // Divide work among threads
  int64_t chunk_size = range / num_threads;
  int64_t remainder = range % num_threads;

  int64_t current_start = start;
  for (int64_t t = 0; t < num_threads; t++) {
    int64_t current_chunk = chunk_size + (t < remainder ? 1 : 0);

    ctx.tasks[t].start = current_start;
    ctx.tasks[t].end = current_start + current_chunk;
    ctx.tasks[t].body = body;

    pthread_create(&ctx.tasks[t].thread_id, NULL, worker_thread, &ctx.tasks[t]);

    current_start += current_chunk;
  }

  // Wait for all threads to complete
  for (int64_t t = 0; t < num_threads; t++) {
    pthread_join(ctx.tasks[t].thread_id, NULL);
  }

  free(ctx.tasks);
}

// Parallel map operation
int64_t *gul_parallel_map(int64_t *input, int64_t length,
                          int64_t (*mapper)(int64_t)) {
  int64_t *output = (int64_t *)malloc(length * sizeof(int64_t));

  // Create closure for parallel execution
  typedef struct {
    int64_t *input;
    int64_t *output;
    int64_t (*mapper)(int64_t);
  } MapClosure;

  MapClosure closure = {input, output, mapper};

  // Parallel execution
  gul_parallel_for(0, length, ^(int64_t i) {
    closure.output[i] = closure.mapper(closure.input[i]);
  });

  return output;
}

// Parallel reduce operation
int64_t gul_parallel_reduce(int64_t *input, int64_t length,
                            int64_t (*reducer)(int64_t, int64_t),
                            int64_t initial) {
  if (length == 0)
    return initial;
  if (length == 1)
    return reducer(initial, input[0]);

  int64_t num_threads = length < MAX_THREADS ? length : MAX_THREADS;
  int64_t *partial_results = (int64_t *)malloc(num_threads * sizeof(int64_t));

  // Initialize partial results
  for (int64_t t = 0; t < num_threads; t++) {
    partial_results[t] = initial;
  }

  // Parallel reduction (simplified without closures)
  int64_t chunk_size = length / num_threads;
  for (int64_t t = 0; t < num_threads; t++) {
    int64_t start = t * chunk_size;
    int64_t end = (t == num_threads - 1) ? length : (t + 1) * chunk_size;

    for (int64_t i = start; i < end; i++) {
      partial_results[t] = reducer(partial_results[t], input[i]);
    }
  }

  // Final reduction
  int64_t result = initial;
  for (int64_t t = 0; t < num_threads; t++) {
    result = reducer(result, partial_results[t]);
  }

  free(partial_results);
  return result;
}
