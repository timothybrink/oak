#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "parse.h"
#include "utils.h"
#include "readall.h"

int main(int argc, char *argv[]) {
  if (argc < 2) {
    printf("missing filename!\n");
    return 1;
  }

  FILE *fptr;
  // printf("opening %s...\n", argv[1]);
  fptr = fopen(argv[1], "r");
  if (fptr == NULL)
    error("file not found!\n");

  char *filecontents;
  size_t chars_read;

  int res = readall(fptr, &filecontents, &chars_read);
  if (res) {
    printf("err: %d\n", res);
    exit(1);
  }
    
  struct list final_list;
  final_list = get_list(&filecontents);
  print_list(final_list);
  free_list(final_list);

  printf("\n");

  return 0;
}

struct list get_list(char **cur) {
  struct list l;

  // printf("%s\n", *cur);

  // check if this list is a list proper
  if (*cur[0] == '(') {
    // consume opening parenthesis and get items
    (*cur)++;
    int i = 0;
    char done_list = 0;
    struct list fl[LIST_MAX_LENGTH];
    while(*cur[0] && i < LIST_MAX_LENGTH) {
      // printf("c: %c", *cur[0]);
      switch (*cur[0]) {
        // discard whitespace
        case ' ':
        case '\n':
          (*cur)++;
          break;
        // end list
        case ')':
          (*cur)++;
          done_list = 1;
          break;
        // it's some kind of sublist
        default:
          fl[i] = get_list(cur);
          i++;          
      }

      if (done_list) break;
    }
    
    // check for errors
    if (i == LIST_MAX_LENGTH - 1)
      error("list reached max length!");

    // no errors, create and return list
    l.type = LIST;
    l.length = i;
    l.contents.list = (struct list *) malloc(sizeof(struct list) * l.length);
    for (i = 0; i < l.length; i++) {
      l.contents.list[i] = fl[i];
    }
  }
  
  // check if it is a token (there is a valid character left)
  else if (*cur[0]) {
    l.type = TOKEN;
    int tok_size = get_token_length(*cur);
    l.contents.token = malloc(tok_size * sizeof(char));
    strncpy(l.contents.token, *cur, tok_size);
    *cur += tok_size;
    // printf("tok: '%s'\n", l.contents.token);
  }
  
  // no characters (null terminator), error
  else
    error("ran out of characters!");

  // return list that was made
  return l;
}

int get_token_length(char *cur) {
  int i = 0;
  while (i < TOKEN_MAX_LENGTH) {
    if (ends_tok(cur[i]))
      return i;
    i++;
  }
  warning("max token length reached");
  return i;
}

int ends_tok(char c) {
  return c == ' ' || c == '\n' || c == ')' || !c;
}