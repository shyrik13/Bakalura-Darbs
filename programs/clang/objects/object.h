//
// Created by shyri on 3/9/2020.
//

#ifndef CLANG_OBJECT_H
#define CLANG_OBJECT_H

#include <GL/glew.h>

typedef struct Object {
    GLfloat *positions;
    GLfloat *textures;
    GLfloat *normals;
    size_t size_positions;
    size_t size_textures;
    size_t size_normals;
} object;

#endif //CLANG_OBJECT_H
