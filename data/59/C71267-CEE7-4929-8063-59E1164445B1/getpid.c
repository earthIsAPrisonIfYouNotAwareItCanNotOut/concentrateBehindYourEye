#include "apue.h"

int
main(void)
{
  printf("hello world from process ID %ld\n", (long)getpid());  /* getpid returns a pid_t data type, We don’t know its size */
  /* so have to cast the value to the largest data type and guarantee a long promotes portability */
    exit(0);
}
