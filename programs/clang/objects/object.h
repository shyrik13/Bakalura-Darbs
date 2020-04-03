//
// Created by shyri on 3/9/2020.
//

#ifndef CLANG_OBJECT_H
#define CLANG_OBJECT_H

#include <GL/glew.h>
#include "cglm/mat4.h"
#include "cglm/cam.h"

typedef struct Object {
    GLfloat *positions;
    GLfloat *textures;
    GLfloat *normals;
    size_t size_positions;
    size_t size_textures;
    size_t size_normals;
    mat4   model;
    GLuint vertexArrayID;
    GLuint vertexbuffer;
    GLuint uvbuffer;
    GLuint unbuffer;
    float  x, y, z;
} object;

void init_gl_object(struct Object * object);
void init_gl_object_model(struct Object * object, float c, float s, float x, float y, float z);
#endif //CLANG_OBJECT_H
