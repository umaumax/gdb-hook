#include <cstdio>

int addXXX(int a, int b) {
  return a + b;
}

int addXXX_hook(int a, int b) {
  printf("hook function called!\n");
  return addXXX(a,b)+123;
}

int main(int argc, char* argv[]) {
  int ret = addXXX(1, 2);
  printf("ret=%d\n", ret);
  return 0;
}
