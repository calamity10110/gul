#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// GUL Runtime Library

// String Concatenation
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

// Integer to String
int64_t gul_int_to_string(int64_t n) {
  char *res = malloc(32);
  if (!res) {
    fprintf(stderr, "GUL Runtime: Out of memory in int_to_string\n");
    exit(1);
  }
  sprintf(res, "%ld", n);
  return (int64_t)res;
}
