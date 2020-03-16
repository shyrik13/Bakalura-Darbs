//
// Created by shyri on 3/9/2020.
//

#ifndef CLANG_NODE_H
#define CLANG_NODE_H
#include<stdio.h>

typedef struct Node
{
    void  *data;
    struct Node *next;
} node;

void push(struct Node** head_ref, void *new_data, size_t data_size);
size_t getSize(struct Node *node);
void printFloat(void *f);
void printList(struct Node *node, void (*fptr)(void *));
float* castToFloatArray(struct Node *node, size_t size);
char* castToChartArray(struct Node *node, size_t size);
#endif //CLANG_NODE_H
