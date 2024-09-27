#include <stdio.h>
#include <stdlib.h>

#include "parse.h"
#include "utils.h"

void print_list(struct list l) {
  if (l.type == LIST) {
    printf("(");
    int i = 0;
    while (i < l.length) {
      // printf("%d,", i);
      print_list(l.contents.list[i]);
      if (i != l.length - 1) {
        printf(" ");
      }
      i++;
    }
    printf(")");
  }
  
  else if (l.type == TOKEN)
    printf("%s", l.contents.token);
}

void free_list(struct list l) {
  if (l.type == LIST) {
    int i;
    for (i = 0; i < l.length; i++) {
      free_list(l.contents.list[i]);
    }
    free(l.contents.list);
  }
  
  else if (l.type == TOKEN)
    free(l.contents.token);
}

// prints the given message
void warning(char *msg) {
  printf("warning: %s\n", msg);
}

// Prints the given message and exits with non-zero exit code
void error(char *msg) {
  printf("error: %s\n", msg);
  exit(1);
}