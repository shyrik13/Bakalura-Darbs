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
#include "common/node.h"
#include <time.h>

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

    cube.x = -15.0f;
    cube.y = 0.0f;
    cube.z = -15.0f;

    struct Node *object_head = NULL;
    push(&object_head, &cube, sizeof(cube));

    mat4 perspective;
    glm_mat4_identity(perspective);
    glm_perspective(glm_rad(90.0f), 4.0f / 3.0f, 0.1f, 100.0f, perspective);

    mat4 view;
    glm_mat4_identity(view);
    vec3 pos = {1.0f, 0.0f, 1.0f};
    vec3 direct = {0.0f, 0.0f, 0.0f};
    vec3 up = {0.0f, 1.0f, 0.0f};
    glm_lookat(pos, direct, up, view);

    vec3 u_light = {0.0, 0.4, (float) 5.7};
    float t = 0.0;
    float c = cosf(t);
    float s = sinf(t);

    GLuint view_matrixID = (GLuint) glGetUniformLocation(programID, "view");
    GLuint perspective_matrixID = (GLuint) glGetUniformLocation(programID, "perspective");
    GLuint model_matrixID = (GLuint) glGetUniformLocation(programID, "model");
    GLuint u_lightID = (GLuint) glGetUniformLocation(programID, "u_light");
    GLuint texture_diffID = (GLuint) glGetUniformLocation(programID, "diffuse_tex");
    GLuint texture_normID = (GLuint) glGetUniformLocation(programID, "normal_tex");

    init_gl_object((struct Object *)(object_head->data));

    double lastTime = glfwGetTime();
    int nbFrames = 0;

    float max = 15.0f;
    float min = -15.0f;

    float max0 = 0.0f;
    float min0 = -30.0f;

    float count = 0.0f;
    srand((unsigned int)time(NULL));

    do {

        double currentTime = glfwGetTime();
        nbFrames++;
        if ( currentTime - lastTime >= 1.0 ){
            printf("%f ms/frame\n", 1000.0/((double)nbFrames));
            nbFrames = 0;
            lastTime += 1.0;

            struct Object cube = create_object_from_file("cube.obj");
            cube.x = (((float)(rand()) / (float)(RAND_MAX)) * (max0 - min0)) + min0;
            cube.y = (((float)(rand()) / (float)(RAND_MAX)) * (max - min)) + min;
            //cube.z = (((float)(rand()) / (float)(RAND_MAX)) * (max0 - min)) + min;
            cube.z = -25.0f;
            init_gl_object(&cube);
            push(&object_head, &cube, sizeof(cube));

        }

        glClearColor(0.0f, 0.0f, 0.4f, 0.0f);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);


        //struct ObjectList *node = head;
        struct Node *node = object_head;

        count = 0.0f;
        while (node != NULL)
        {
            struct Object object = *(struct Object *)(node->data);

            init_gl_object_model(&object, c, s, object.x, object.y, object.z);

            glUseProgram(programID);

            glBindVertexArray(object.vertexArrayID);

            glUniformMatrix4fv(view_matrixID, 1, GL_FALSE, &view[0][0]);
            glUniformMatrix4fv(perspective_matrixID, 1, GL_FALSE, &perspective[0][0]);
            glUniformMatrix4fv(model_matrixID, 1, GL_FALSE, &(object.model[0][0]));
            glUniform3fv(u_lightID, 1, u_light);

            // Bind our texture in Texture Unit 0
            glActiveTexture(GL_TEXTURE0 + 0);
            glBindTexture(GL_TEXTURE_2D, texture_diff);
            glUniform1i(texture_diffID, 0);

            glActiveTexture(GL_TEXTURE0 + 2);
            glBindTexture(GL_TEXTURE_2D, texture_norm);
            glUniform1i(texture_normID, 2);

            glEnableVertexAttribArray(0);
            glBindBuffer(GL_ARRAY_BUFFER, object.vertexbuffer);
            glVertexAttribPointer(
                    0,
                    3,
                    GL_FLOAT,
                    GL_FALSE,
                    0,
                    (void*)0
            );

            glEnableVertexAttribArray(1);
            glBindBuffer(GL_ARRAY_BUFFER, object.uvbuffer);
            glVertexAttribPointer(
                    1,
                    2,
                    GL_FLOAT,
                    GL_FALSE,
                    0,
                    (void*)0
            );

            glEnableVertexAttribArray(2);
            glBindBuffer(GL_ARRAY_BUFFER, object.unbuffer);
            glVertexAttribPointer(
                    2,                                // attribute. No particular reason for 2, but must match the layout in the shader.
                    3,                                // size
                    GL_FLOAT,                         // type
                    GL_TRUE,                          // normalized?
                    0,                                // stride
                    (void*)0                          // array buffer offset
            );

            glDrawArrays(GL_TRIANGLES, 0, (GLsizei) object.size_positions);

            glDisableVertexAttribArray(0);
            glDisableVertexAttribArray(1);
            glDisableVertexAttribArray(2);

            node = node->next;
            count += 5.0f;
        }



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

    struct Node *node = object_head;
    while (node != NULL) {

        struct Object object = *(struct Object *)(node->data);
        glDeleteBuffers(1, &object.vertexbuffer);
        glDeleteBuffers(1, &object.uvbuffer);
        glDeleteBuffers(1, &object.unbuffer);
        glDeleteVertexArrays(1, &object.vertexArrayID);

        node = node->next;
    }

    glDeleteProgram(programID);
    glDeleteTextures(1, &texture_diff);
    glDeleteTextures(1, &texture_norm);

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