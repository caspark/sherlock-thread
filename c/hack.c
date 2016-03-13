#include <stdio.h>

int puts (const char *s)
{
  return printf("Hijacked puts: %s\n", s);
}
