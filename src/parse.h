#define LINE_MAX_LENGTH   200
#define LIST_MAX_LENGTH   10
#define TOKEN_MAX_LENGTH  50

struct list get_list(char **);
int get_token_length(char *);
int ends_tok(char);

enum types {LIST, TOKEN};

union list_contents {
  struct list *list;
  char *token;
};

struct list {
  union list_contents contents;
  enum types type;
  unsigned int length;
};