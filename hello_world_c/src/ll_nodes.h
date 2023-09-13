// ll_nodes.h

#ifndef MYFUNCTIONS_H
#define MYFUNCTIONS_H

char *create_heap_string(char *sourceString);

struct LLNode *create_llnode(char *data);

struct LLNode *append_llnode(struct LLNode **head, char *data);

struct LLNode *append_llnode(struct LLNode **head, char *data);

struct LLNode *create_insert_llnode(struct LLNode **head, char *data, int index);

int delete_llnode(struct LLNode *head, int index);

struct LLNode *find_node(struct LLNode *head, char *data);

#endif