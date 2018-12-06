#include <ctype.h>
#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/**
# 1 + 2 * 4
# 
#  __+__
# 1    _*_
#     2   4
#
# expr: 
# addsub: muldiv + addsub | muldiv - addsub | muldiv
# muldiv: number * muldiv | number - muldiv | number
# number: [0-9]+
*/

typedef enum {
  TK_NUM = 256,
  TK_EOF,
} TokenTypeEnum;

typedef int TokenType;

typedef struct {
  int type;
  long int value;
  char *input;
} Token;

typedef enum {
  ND_NUM = 256,
} NodeTypeEnum;

typedef int NodeType;

typedef struct Node {
  int type;
  struct Node *lhs;
  struct Node *rhs;
  int value;
} Node;

// パーサが読み込んでいるトークンの位置
int pos = 0;

Token tokens[100];

void tokenize(char *p) {
  int index = 0;

  while (*p) {
    if (isspace(*p)) {
      p++;
      continue;
    }

    switch (*p) {
      case '+':
      case '-':
      case '*':
      case '/':
      case '(':
      case ')':
        tokens[index].type = *p;
        tokens[index].input = p;
        index++;
        p++;
        continue;
      default:
        break;
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

void error_message(const char* str, const char* input) {
  fprintf(stderr, str, input);
}

void error(const int index) {
  fprintf(stderr, "予期せぬトークンです: %s\n", tokens[index].input);
  exit(1);
}

Node* Node_new(const int type, Node* lhs, Node* rhs) {
  Node *node = (Node *)malloc(sizeof(Node));
  node->type = type;
  node->lhs = lhs;
  node->rhs = rhs;

  return node;
}

Node* Node_number(const int value) {
  Node* node = Node_new(ND_NUM, NULL, NULL);
  node->value = value;

  return node;
}

void Node_free(Node* node) {
  if (node->lhs) {
    free(node->lhs);
  }

  if (node->rhs) {
    free(node->rhs);
  }

  free(node);
}

Node* Node_mul();

Node* Node_expr() {
  Node* lhs = Node_mul();

  TokenType type = tokens[pos].type;

  if (type == '+') {
    pos++;
    return Node_new('+', lhs, Node_expr());
  }

  if (type == '-') {
    pos++;
    return Node_new('-', lhs, Node_expr());
  }

  return lhs;
}

Node* Node_term() {
  const TokenType type = tokens[pos].type;

  if (type == TK_NUM) {
    return Node_number(tokens[pos++].value);
  }

  if (type == '(') {
    pos++;
    
    Node* node = Node_expr();

    if (tokens[pos].type != ')') {
      error_message("閉じカッコがありません: %s", tokens[pos].input);
    }

    pos++;

    return node;
  }

  error_message("数値でも開きカッコでもないトークンです: %s", tokens[pos].input);

  return NULL;
}

Node* Node_mul() {
  Node *lhs = Node_term();

  const TokenType type = tokens[pos].type;

  if (type == '*') {
    pos++;
    return Node_new('*', lhs, Node_mul());
  }
 
  if (type == '/') {
    pos++;
    return Node_new('/', lhs, Node_mul());
  }

  return lhs;
}

void compile(const Node* node) {
  if (node == NULL) {
    printf("コないはず\n");
    exit(2);
  }

  switch (node->type) {
    case '+':
      compile(node->lhs);
      compile(node->rhs);
      printf("  pop rdi\n");
      printf("  pop rax\n");
      printf("  add rax, rdi\n");
      printf("  push rax\n");
      break;
    case '-':
      compile(node->lhs);
      compile(node->rhs);
      printf("  pop rdi\n");
      printf("  pop rax\n");
      printf("  sub rax, rdi\n");
      printf("  push rax\n");
      break;
    case '/':
      compile(node->lhs);
      compile(node->rhs);
      printf("  pop rdi\n");    // 除数
      printf("  pop rax\n");    // 被除数の下位64ビット
      printf("  mov rdx, 0\n"); // 被除数の上位64ビット
      printf("  div rdi\n");    // 「(RDX << 64 + RAX)」 / RDI https://www.mztn.org/lxasm64/amd04.html
      printf("  push rax\n");
      break;
    case '*':
      compile(node->lhs);
      compile(node->rhs);
      printf("  pop rdi\n"); // 乗数
      printf("  pop rax\n"); // 被乗数の下位64ビット
      printf("  mul rdi\n"); // 「(RDX << 64 + RAX)」 * RDI  https://www.mztn.org/lxasm64/amd04.html
      printf("  push rax\n");
      break;
    case ND_NUM:
      printf("  push %d\n", node->value);
      break;
    default:
      fprintf(stderr, "未知のノードに遭遇しました");
      exit(1);
  }
}

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "引数の個数が正しくありません\n");
    return 1;
  }

  tokenize(argv[1]);

  Node* root = Node_expr();

  // アセンブリの前半部分を出力
  printf(".intel_syntax noprefix\n");
  printf(".global main\n");
  printf("main:\n");

  compile(root);

  // 最後にスタックにある値を取り出す
  printf("  pop rax\n");
  printf("  ret\n");

  Node_free(root);

  return 0;
}

// vim: set et ts=2 sw=2 :
