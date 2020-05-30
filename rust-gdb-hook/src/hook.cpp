#include <cstdio>

extern "C" {
int add(int a, int b);
}

int add_hook(int a, int b) {
  printf("c++ lib hook called!\n");
  return add(a, b) + 123;
}
