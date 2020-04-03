//
// Created by shyri on 4/3/2020.
//

#include "object.h"
#include <GL/glew.h>
#include <stdio.h>

void init_gl_object(struct Object * object)
{
    glGenVertexArrays(1, &(object->vertexArrayID));
    glBindVertexArray(object->vertexArrayID);

    glGenBuffers(1, &(object->vertexbuffer));
    glBindBuffer(GL_ARRAY_BUFFER, object->vertexbuffer);
    glBufferData(GL_ARRAY_BUFFER, object->size_positions * sizeof(float), object->positions, GL_STATIC_DRAW);

    glGenBuffers(1, &(object->uvbuffer));
    glBindBuffer(GL_ARRAY_BUFFER, object->uvbuffer);
    glBufferData(GL_ARRAY_BUFFER, object->size_textures * sizeof(float), object->textures, GL_STATIC_DRAW);

    glGenBuffers(1, &(object->unbuffer));
    glBindBuffer(GL_ARRAY_BUFFER, object->unbuffer);
    glBufferData(GL_ARRAY_BUFFER, object->size_normals * sizeof(float), object->normals, GL_STATIC_DRAW);
}

void init_gl_object_model(struct Object * object, float c, float s, float x, float y, float z)
{
    mat4 model = {
            {powf(c, 2), -c*s,                        s,          0.0},
            {c*(powf(s, 2)+s), powf(c, 2)-powf(s, 3), -c*s,       0.0},
            {s*(s-powf(c, 2)), c*(powf(s, 2)+s),      powf(c, 2), 0.0},
            {x, y,                       z,        1.0}
    };

    glm_mat4_copy(model, object->model);
}




