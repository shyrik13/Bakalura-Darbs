//
// Created by shyri on 3/9/2020.
//

#include<stdio.h>
#include<stdlib.h>
#include "node.h"

void push(struct Node** head_ref, void *new_data, size_t data_size)
{
    // Allocate memory for node
    struct Node* new_node = (struct Node*)malloc(sizeof(struct Node));

    new_node->data = malloc(data_size);
    new_node->next = (*head_ref);

    // Copy contents of new_data to newly allocated memory.
    // Assumption: char takes 1 byte.
    int i;
    for (i=0; i<data_size; i++)
        *(char *)(new_node->data + i) = *(char *)(new_data + i);

    // Change head pointer as new node is added at the beginning
    (*head_ref) = new_node;

}

size_t getSize(struct Node *node) {
    size_t size = 0;
    while (node != NULL)
    {
        size++;
        node = node->next;
    }
    return size;
}

void printFloat(void *f)
{
    printf(" %f", *(float *)f);
}

void printList(struct Node *node, void (*fptr)(void *))
{
    while (node != NULL)
    {
        (*fptr)(node->data);
        node = node->next;
    }
}

float* castToFloatArray(struct Node *node, size_t size) {
    float* array = malloc(size * sizeof(array));
    int index = (int) (size - 1);
    while (node != NULL) {
        array[index] = *(float *)(node->data);
        node = node->next;
        index--;
    }
    return array;
}

char* castToChartArray(struct Node *node, size_t size) {
    char* array = malloc(size * sizeof(array));
    int index = (int) (size - 1);
    while (node != NULL) {
        array[index] = *(char *)(node->data);
        node = node->next;
        index--;
    }
    return array;
}