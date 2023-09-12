#include <stdio.h>
#include <stdlib.h>

// Define a structure for a doubly linked list node
struct LLNode {
    int data;
    struct LLNode* next;
    struct LLNode* prev;
};

struct LLNode* create_llnode(int data){
    struct LLNode* newNode = (struct LLNode*)malloc(sizeof(struct LLNode));

    if (newNode == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        exit(1);
    }

    newNode->data = data;
    newNode->next = NULL;
    newNode->prev = NULL;

    return newNode;
}

struct LLNode* append_llnode(struct LLNode** head, int data){
    struct LLNode* current = *head;
    while (current->next != NULL) {
        current = current->next;
    }
    current->next = create_llnode(data);

    return current;
}

struct LLNode* prepend_llnode(struct LLNode** head, int data){
    struct LLNode* newHead = create_llnode(data);

    newHead->next = *head;

    if (*head != NULL) {
        (*head)->prev = newHead; // Update the previous head's prev pointer
    }

    *head = newHead; // Update the head pointer to point to newHead

    return newHead;
}

struct LLNode* insert_llnode(struct LLNode** head, int data, int index){
    int count = 0;

    // three cases:
    // index is 0
    if (index == 0){
        // prepend node
        return prepend_llnode(*head, data);
    }

    // index is < length of list
    struct LLNode* current = *head;

    while (current != NULL){
        if (count == index - 1){
            // make new node
            struct LLNode* newNode = create_llnode(data);

            // insert new node
            newNode->prev = current;
            newNode->next = current->next;

            if(current->next != NULL){
                current->next->prev = newNode;
            }

            return newNode;
        }
        current = current->next;
        count++;
    }

    // index is > length of list
    if (count > index){
        // append node
        return append_llnode(*head, data);
    }
    


}