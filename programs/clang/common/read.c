//
// Created by shyri on 3/9/2020.
//

#include "../objects/object.h"
#include "node.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <io.h>
#include "read.h"

static const char FILE_PATH[] = "res/files/";

struct Object create_object_from_file(char *file_name) {

    char* file_path = concat(FILE_PATH, file_name);

    struct Object new_object;

    struct Node *start_v = NULL;
    struct Node *start_vt = NULL;
    struct Node *start_vn = NULL;

    float *arr_v = NULL;
    float *arr_vt = NULL;
    float *arr_vn = NULL;

    size_t size_v = 0;
    size_t size_vt = 0;
    size_t size_vn = 0;

    struct Node *start_pos = NULL;
    struct Node *start_tex = NULL;
    struct Node *start_norm = NULL;

    unsigned float_size = sizeof(GLfloat);
    char* pend;
    char* u_pend;
    if( access( file_path, F_OK ) != -1 ) {
        FILE* file = fopen(file_path, "r"); /* should check the result */
        char line[256];
        char * pch;
        while (fgets(line, sizeof(line), file)) {
            pch = strtok(line," ");
            while (pch != NULL) {

                if (strcmp(pch, "v") == 0) {
                    float first = strtof(strtok(NULL, " "), &pend);
                    float second = strtof(strtok(NULL, " "), &pend);
                    float third = strtof(strtok(NULL, " "), &pend);
                    push(&start_v, &first, float_size);
                    push(&start_v, &second, float_size);
                    push(&start_v, &third, float_size);
                } else if (strcmp(pch, "vt") == 0) {
                    if (arr_v == NULL) {
                        size_v = getSize(start_v);
                        arr_v = castToFloatArray(start_v, size_v);
                        free(start_v);
                    }
                    float first = strtof(strtok(NULL, " "), &pend);
                    float second = strtof(strtok(NULL, " "), &pend);
                    push(&start_vt, &first, float_size);
                    push(&start_vt, &second, float_size);
                } else if (strcmp(pch, "vn") == 0) {
                    if (arr_vt == NULL) {
                        size_vt = getSize(start_vt);
                        arr_vt = castToFloatArray(start_vt, size_vt);
                        free(start_vt);
                    }
                    float first = strtof(strtok(NULL, " "), &pend);
                    float second = strtof(strtok(NULL, " "), &pend);
                    float third = strtof(strtok(NULL, " "), &pend);
                    push(&start_vn, &first, float_size);
                    push(&start_vn, &second, float_size);
                    push(&start_vn, &third, float_size);
                } else if (strcmp(pch, "f") == 0) {
                    if (arr_vn == NULL) {
                        size_vn = getSize(start_vn);
                        arr_vn = castToFloatArray(start_vn, size_vn);
                        free(start_vn);
                    }

                    char * first = strtok(NULL, " ");
                    char * second = strtok(NULL, " ");
                    char * third = strtok(NULL, " ");

                    u_pend = strtok(first, "/");
                    int counter = 0;
                    int idx;
                    float f1 = 0;
                    float f2 = 0;
                    float f3 = 0;
                    while (counter < 3) {
                        idx = atoi(u_pend);
                        switch (counter) {
                            case 0: {
                                idx = (idx-1)*3;
                                f1 = arr_v[idx];
                                f2 = arr_v[idx+1];
                                f3 = arr_v[idx+2];
                                push(&start_pos, &f1, float_size);
                                push(&start_pos, &f2, float_size);
                                push(&start_pos, &f3, float_size);
                            } break;
                            case 1: {
                                idx = (idx-1)*2;
                                f1 = arr_vt[idx];
                                f2 = arr_vt[idx+1];
                                push(&start_tex, &f1, float_size);
                                push(&start_tex, &f2, float_size);
                            } break;
                            case 2: {
                                idx = (idx-1)*3;
                                f1 = arr_vn[idx];
                                f2 = arr_vn[idx+1];
                                f3 = arr_vn[idx+2];
                                push(&start_norm, &f1, float_size);
                                push(&start_norm, &f2, float_size);
                                push(&start_norm, &f3, float_size);
                            } break;
                        }
                        u_pend = strtok(NULL, "/");
                        counter++;
                    }

                    u_pend = strtok(second, "/");
                    counter = 0;
                    while (counter < 3) {
                        idx = atoi(u_pend);
                        switch (counter) {
                            case 0: {
                                idx = (idx-1)*3;
                                f1 = arr_v[idx];
                                f2 = arr_v[idx+1];
                                f3 = arr_v[idx+2];
                                push(&start_pos, &f1, float_size);
                                push(&start_pos, &f2, float_size);
                                push(&start_pos, &f3, float_size);
                            } break;
                            case 1: {
                                idx = (idx-1)*2;
                                f1 = arr_vt[idx];
                                f2 = arr_vt[idx+1];
                                push(&start_tex, &f1, float_size);
                                push(&start_tex, &f2, float_size);
                            } break;
                            case 2: {
                                idx = (idx-1)*3;
                                f1 = arr_vn[idx];
                                f2 = arr_vn[idx+1];
                                f3 = arr_vn[idx+2];
                                push(&start_norm, &f1, float_size);
                                push(&start_norm, &f2, float_size);
                                push(&start_norm, &f3, float_size);
                            } break;
                        }
                        u_pend = strtok(NULL, "/");
                        counter++;
                    }

                    u_pend = strtok(third, "/");
                    counter = 0;
                    while (counter < 3) {
                        idx = atoi(u_pend);
                        switch (counter) {
                            case 0: {
                                idx = (idx-1)*3;
                                f1 = arr_v[idx];
                                f2 = arr_v[idx+1];
                                f3 = arr_v[idx+2];
                                push(&start_pos, &f1, float_size);
                                push(&start_pos, &f2, float_size);
                                push(&start_pos, &f3, float_size);
                            } break;
                            case 1: {
                                idx = (idx-1)*2;
                                f1 = arr_vt[idx];
                                f2 = arr_vt[idx+1];
                                push(&start_tex, &f1, float_size);
                                push(&start_tex, &f2, float_size);
                            } break;
                            case 2: {
                                idx = (idx-1)*3;
                                f1 = arr_vn[idx];
                                f2 = arr_vn[idx+1];
                                f3 = arr_vn[idx+2];
                                push(&start_norm, &f1, float_size);
                                push(&start_norm, &f2, float_size);
                                push(&start_norm, &f3, float_size);
                            } break;
                        }
                        u_pend = strtok(NULL, "/");
                        counter++;
                    }
                }
                pch = strtok(NULL, " ");
            }
        }
        fclose(file);

        free(arr_v);
        free(arr_vt);
        free(arr_vn);

        new_object.size_positions = getSize(start_pos);
        new_object.positions = castToFloatArray(start_pos, new_object.size_positions);
        free(start_pos);
        new_object.size_textures = getSize(start_tex);
        new_object.textures = castToFloatArray(start_tex, new_object.size_textures);
        free(start_tex);
        new_object.size_normals = getSize(start_norm);
        new_object.normals = castToFloatArray(start_norm, new_object.size_normals);
        free(start_norm);
    } else {
        printf("File not exist in cmake-build/res folder");
        exit(-1);
    }
    free(file_path);

    return new_object;
}

char* concat(const char *s1, const char *s2)
{
    const size_t len1 = strlen(s1);
    const size_t len2 = strlen(s2);
    char *result = malloc(len1 + len2 + 1);
    memcpy(result, s1, len1);
    memcpy(result + len1, s2, len2 + 1);
    return result;
}