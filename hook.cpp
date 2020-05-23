#include <cstdio>

extern int addXXX(int a, int b);

int addXXX_hook(int a, int b) {
  printf("hook function called!\n");
  return addXXX(a,b)+123;
}
