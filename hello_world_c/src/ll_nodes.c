#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Define a structure for a doubly linked list node
struct LLNode {
  char *data;
  struct LLNode *next;
  struct LLNode *prev;
};

char *create_heap_string(char *sourceString) {
  // Allocate memory for the string (+1 for the null terminator)
  // (char*): This is a typecast. It's used to explicitly cast the pointer
  // returned by malloc to a char* type. The return type of malloc is void*, and
  // this typecast informs the compiler that we intend to treat the allocated
  // memory as a character array (string).
  char *heapString = (char *)malloc(strlen(sourceString) + 1);

  if (heapString == NULL) {
    fprintf(stderr, "Memory allocation failed\n");
    return NULL;
  }

  // Copy the source string to the heap-allocated memory
  strcpy(heapString, sourceString);

  return heapString;
}

struct LLNode *create_llnode(char *data) {
  struct LLNode *newNode = (struct LLNode *)malloc(sizeof(struct LLNode));

  if (newNode == NULL) {
    fprintf(stderr, "Memory allocation failed\n");
    exit(1);
  }

  newNode->data = create_heap_string(data);
  newNode->next = NULL;
  newNode->prev = NULL;

  return newNode;
}

struct LLNode *append_llnode(struct LLNode **head, char *data) {
  struct LLNode *current = *head;
  while (current->next != NULL) {
    current = current->next;
  }
  current->next = create_llnode(data);

  return current;
}

struct LLNode *prepend_llnode(struct LLNode **head, char *data) {
  struct LLNode *newHead = create_llnode(data);

  newHead->next = *head;

  if (*head != NULL) {
    (*head)->prev = newHead;  // Update the previous head's prev pointer
  }

  *head = newHead;  // Update the head pointer to point to newHead

  return newHead;
}

/**
 * create_insert_llnode
 *
 * create and insert a node at the given index
 */
struct LLNode *create_insert_llnode(struct LLNode **head, char *data,
                                    int index) {
  int count = 0;

  // three cases:
  // index is 0
  if (index == 0) {
    // prepend node
    return prepend_llnode(head, data);
  }

  // index is < length of list
  struct LLNode *current = *head;

  while (current != NULL) {
    if (count == index - 1) {
      // make new node
      struct LLNode *newNode = create_llnode(data);

      // insert new node
      newNode->prev = current;
      newNode->next = current->next;

      if (current->next != NULL) {
        current->next->prev = newNode;
      }

      return newNode;
    }
    current = current->next;
    count++;
  }

  // index is > length of list
  if (count > index) {
    // append node
    return append_llnode(head, data);
  }

  return NULL;
}

/**
 * delete_llnode
 *
 * delete a node by index
 */
int delete_llnode(struct LLNode *head, int index) {
  int count = 0;
  struct LLNode *current = head;

  if (head != NULL) {
    while (current != NULL) {
      if (count == index - 1) {
        // rewire the prev and next nodes
        struct LLNode *prevNode = current->prev;
        struct LLNode *nextNode = current->next;

        if (prevNode != NULL) {
          prevNode->next = nextNode;
        }
        if (nextNode != NULL) {
          nextNode->prev = prevNode;
        }

        // free the current node
        free(current->data);
        free(current);

        // success
        return 1;
      }
      current = current->next;
      count++;
    }
  }
  // failure
  return 0;
}

struct LLNode *find_node(struct LLNode *head, char *data) {
  struct LLNode *current = head;

  if (head != NULL) {
    while (current->next != NULL) {
      int match = strcmp(data, current->data);

      if (match == 0) {
        printf("Match found\n");
        return current;
      }

      current = current->next;
    }
  }
  return NULL;
}