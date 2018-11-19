#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
  TK_NUM = 256,
  TK_EOF,
} TokenType;

typedef struct {
  int type;
  long int value;
  char *input;
} Token;

Token tokens[100];

void tokenize(char *p) {
  int index = 0;

  while (*p) {
    if (isspace(*p)) {
      p++;
      continue;
    }

    if (*p == '+' || *p == '-') {
      tokens[index].type = *p;
      tokens[index].input = p;
      index++;
      p++;
      continue;
    }

    if (isdigit(*p)) {
      tokens[index].type = TK_NUM;
      tokens[index].input = p;
      tokens[index].value = strtol(p, &p, 10);
      index++;

      if (errno == ERANGE) {
        fprintf(stderr, "long型の範囲を超えています: %s", p);
        exit(1);
      }

      continue;
    }

    fprintf(stderr, "字句解析に失敗しました: %s", p);
    exit(1);
  }

  tokens[index].type = TK_EOF;
}

void error(int index) {
  fprintf(stderr, "予期せぬトークンです: %s\n", tokens[index].input);
  exit(1);
}

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "引数の個数が正しくありません\n");
    return 1;
  }

  tokenize(argv[1]);

  // アセンブリの前半部分を出力
  printf(".intel_syntax noprefix\n");
  printf(".global main\n");
  printf("main:\n");

  if (tokens[0].type != TK_NUM) {
    error(0);
  }

  printf("  mov rax, %ld\n", tokens[0].value);

  int i = 1;
  while (tokens[i].type != TK_EOF) {
    // printf("tokens[%d].type %d\n", i, tokens[i].type);

    if (tokens[i].type == '+') {
      i++;
      if (tokens[i].type != TK_NUM) {
        error(i);
      }

      printf("  add rax, %ld\n", tokens[i].value);
      i++;
      continue;
    }

    if (tokens[i].type == '-') {
      i++;
      if (tokens[i].type != TK_NUM) {
        error(i);
      }

      printf("  sub rax, %ld\n", tokens[i].value);
      i++;
      continue;
    }

    error(i);
  }

  printf("  ret\n");
  return 0;
}

// vim: set et ts=2 sw=2 :
