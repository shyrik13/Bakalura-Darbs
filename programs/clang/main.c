#include <stdio.h>
#include <stdlib.h>
#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <stdbool.h>
#include "common/read.h"
#include "objects/object.h"
#include "common/shader.h"
#include "common/texture.h"

#include "cglm/mat4.h"
#include "cglm/cam.h"

const int WINDOW_HEIGHT = 620;
const int WINDOW_WIDTH = 1240;

void controls(GLFWwindow* window, int key, int scancode, int action, int mods)
{
    if(action == GLFW_PRESS)
        if(key == GLFW_KEY_ESCAPE)
            glfwSetWindowShouldClose(window, GL_TRUE);
}

GLFWwindow* initWindow(const int resX, const int resY)
{

    if(!glfwInit())
    {
        fprintf(stderr, "Failed to initialize GLFW\n");
        return NULL;
    }
    glfwWindowHint(GLFW_SAMPLES, 4); // 4x antialiasing

    // Open a window and create its OpenGL context
    GLFWwindow* window = glfwCreateWindow(resX, resY, "TEST", NULL, NULL);

    if(window == NULL)
    {
        fprintf(stderr, "Failed to open GLFW window.\n");
        glfwTerminate();
        return NULL;
    }

    glfwMakeContextCurrent(window);
    glfwSetKeyCallback(window, controls);

    // Get info of GPU and supported OpenGL version
    printf("Renderer: %s\n", glGetString(GL_RENDERER));
    printf("OpenGL version supported %s\n", glGetString(GL_VERSION));

    glEnable(GL_DEPTH_TEST); // Depth Testing
    glDepthFunc(GL_LEQUAL);
    glDisable(GL_CULL_FACE);
    glCullFace(GL_BACK);
    return window;
}

void display( GLFWwindow* window )
{

    GLuint programID = load_shaders("vertex_shader.txt", "fragment_shader.txt");
    GLuint texture_diff = load_DDS("tuto-14-diffuse.dds");
    GLuint texture_norm = load_DDS("tuto-14-normal.dds");

    struct Object cube = create_object_from_file("cube.obj");

    mat4 perspective;
    glm_mat4_identity(perspective);
    glm_perspective(glm_rad(45.0f), 4.0f / 3.0f, 0.1f, 100.0f, perspective);

    mat4 view;
    glm_mat4_identity(view);
    vec3 pos = {4.0f, 3.0f, 3.0f};
    vec3 direct = {0.0f, 0.0f, 0.0f};
    vec3 up = {0.0f, 1.0f, 0.0f};
    glm_lookat(pos, direct, up, view);

    vec3 u_light = {0.0, 0.4, (float) 5.7};
    float t = 0.0;
    float c = cosf(t);
    float s = sinf(t);

    GLuint VertexArrayID;
    glGenVertexArrays(1, &VertexArrayID);
    glBindVertexArray(VertexArrayID);

    GLuint view_matrixID = (GLuint) glGetUniformLocation(programID, "view");
    GLuint perspective_matrixID = (GLuint) glGetUniformLocation(programID, "perspective");
    GLuint model_matrixID = (GLuint) glGetUniformLocation(programID, "model");
    GLuint u_lightID = (GLuint) glGetUniformLocation(programID, "u_light");
    GLuint texture_diffID = (GLuint) glGetUniformLocation(programID, "diffuse_tex");
    GLuint texture_normID = (GLuint) glGetUniformLocation(programID, "normal_tex");

    GLuint vertexbuffer;
    glGenBuffers(1, &vertexbuffer);
    glBindBuffer(GL_ARRAY_BUFFER, vertexbuffer);
    glBufferData(GL_ARRAY_BUFFER, cube.size_positions * sizeof(float), cube.positions, GL_STATIC_DRAW);

    GLuint uvbuffer;
    glGenBuffers(1, &uvbuffer);
    glBindBuffer(GL_ARRAY_BUFFER, uvbuffer);
    glBufferData(GL_ARRAY_BUFFER, cube.size_textures * sizeof(float), cube.textures, GL_STATIC_DRAW);

    GLuint unbuffer;
    glGenBuffers(1, &unbuffer);
    glBindBuffer(GL_ARRAY_BUFFER, unbuffer);
    glBufferData(GL_ARRAY_BUFFER, cube.size_normals * sizeof(float), cube.normals, GL_STATIC_DRAW);

    do {

        mat4 model = {
                {powf(c, 2), -c*s,                        s,          0.0},
                {c*(powf(s, 2)+s), powf(c, 2)-powf(s, 3), -c*s,       0.0},
                {s*(s-powf(c, 2)), c*(powf(s, 2)+s),      powf(c, 2), 0.0},
                {-3.0f, 0.0f,                       0.0f,        1.0}
        };

        glClearColor(0.0f, 0.0f, 0.4f, 0.0f);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        glUseProgram(programID);

        glUniformMatrix4fv(view_matrixID, 1, GL_FALSE, &view[0][0]);
        glUniformMatrix4fv(perspective_matrixID, 1, GL_FALSE, &perspective[0][0]);
        glUniformMatrix4fv(model_matrixID, 1, GL_FALSE, &model[0][0]);
        glUniform3fv(u_lightID, 1, u_light);

        // Bind our texture in Texture Unit 0
        glActiveTexture(GL_TEXTURE0 + 0);
        glBindTexture(GL_TEXTURE_2D, texture_diff);
        glUniform1i(texture_diffID, 0);

        glActiveTexture(GL_TEXTURE0 + 2);
        glBindTexture(GL_TEXTURE_2D, texture_norm);
        glUniform1i(texture_normID, 2);

        glEnableVertexAttribArray(0);
        glBindBuffer(GL_ARRAY_BUFFER, vertexbuffer);
        glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                GL_FALSE,
                0,
                (void*)0
        );

        glEnableVertexAttribArray(1);
        glBindBuffer(GL_ARRAY_BUFFER, uvbuffer);
        glVertexAttribPointer(
                1,
                2,
                GL_FLOAT,
                GL_FALSE,
                0,
                (void*)0
        );

        glEnableVertexAttribArray(2);
        glBindBuffer(GL_ARRAY_BUFFER, unbuffer);
        glVertexAttribPointer(
                2,                                // attribute. No particular reason for 2, but must match the layout in the shader.
                3,                                // size
                GL_FLOAT,                         // type
                GL_TRUE,                          // normalized?
                0,                                // stride
                (void*)0                          // array buffer offset
        );

        glDrawArrays(GL_TRIANGLES, 0, (GLsizei) cube.size_positions);

        glDisableVertexAttribArray(0);
        glDisableVertexAttribArray(1);
        glDisableVertexAttribArray(2);

        // Swap buffers
        glfwSwapBuffers(window);
        glfwPollEvents();

        t += 0.001;

        if (t > 360) {
            t = 0;
        }

        c = cosf(t);
        s = sinf(t);

    } while (glfwGetKey(window, GLFW_KEY_ESCAPE ) != GLFW_PRESS &&
             glfwWindowShouldClose(window) == 0);

    glDeleteBuffers(1, &vertexbuffer);
    glDeleteBuffers(1, &uvbuffer);
    glDeleteBuffers(1, &unbuffer);
    glDeleteProgram(programID);
    glDeleteTextures(1, &texture_diff);
    glDeleteTextures(1, &texture_norm);
    glDeleteVertexArrays(1, &VertexArrayID);

}

int main(int argc, char** argv)
{
    GLFWwindow* window = initWindow(WINDOW_WIDTH, WINDOW_HEIGHT);
    if( NULL != window )
    {
        // Initialize GLEW
        glewExperimental = true;
        if (glewInit() != GLEW_OK) {
            fprintf(stderr, "Failed to initialize GLEW\n");
            return -1;
        }
        display( window );
    }
    glfwDestroyWindow(window);
    glfwTerminate();
    return 0;
}