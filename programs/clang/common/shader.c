//
// Created by shyri on 3/9/2020.
//

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <GL/glew.h>
#include <io.h>
#include <stdbool.h>

#include "shader.h"
#include "read.h"
#include "node.h"

static const char FILE_PATH[] = "res/shaders/";

char* file_into_string(const char * file_path) {
    char *buffer = NULL;
    size_t size = 0;
    if( access( file_path, F_OK ) != -1 ) {
        FILE *fp = fopen(file_path, "r");
        fseek(fp, 0, SEEK_END); /* Go to end of file */
        size = ftell(fp); /* How many bytes did we pass ? */
        rewind(fp);
        buffer = malloc((size + 1) * sizeof(*buffer));
        fread(buffer, size, 1, fp);
        buffer[size] = '\0';
        fclose(fp);
    } else {
        printf("File not exist in cmake-build/res folder");
    }
    return buffer;
}

char* file_into_string2(const char * file_path) {
    FILE    *infile;
    char    *buffer;
    long    numbytes;

    infile = fopen(file_path, "r");

    if(infile == NULL)
        return NULL;

    fseek(infile, 0L, SEEK_END);
    numbytes = ftell(infile);

    fseek(infile, 0L, SEEK_SET);

    buffer = (char*)calloc(numbytes, sizeof(char));

    if(buffer == NULL)
        return NULL;

    fread(buffer, sizeof(char), numbytes, infile);
    fclose(infile);

    return buffer;
}

GLuint load_shaders(const char * vertex_file_name, const char * fragment_file_name){

    // Create the shaders
    GLuint VertexShaderID = glCreateShader(GL_VERTEX_SHADER);
    GLuint FragmentShaderID = glCreateShader(GL_FRAGMENT_SHADER);

    char* vertex_file_path = concat(FILE_PATH, vertex_file_name);
    char* fragment_file_path = concat(FILE_PATH, fragment_file_name);

    char* vertex_shader_code = file_into_string2(vertex_file_path);
    char* fragment_shader_code = file_into_string2(fragment_file_path);

    GLint Result = GL_FALSE;
    int InfoLogLength;

    // Compile Vertex Shader
    printf("Compiling shader : %s\n", vertex_file_path);
    char const * vertex_source_pointer = vertex_shader_code;
    glShaderSource(VertexShaderID, 1, &vertex_source_pointer , NULL);
    glCompileShader(VertexShaderID);

    free(vertex_shader_code);
    // Check Vertex Shader
    glGetShaderiv(VertexShaderID, GL_COMPILE_STATUS, &Result);
    glGetShaderiv(VertexShaderID, GL_INFO_LOG_LENGTH, &InfoLogLength);
    if ( InfoLogLength > 0 ){
        char vertex_shader_error_message[InfoLogLength+1];
        glGetShaderInfoLog(VertexShaderID, InfoLogLength, NULL, &vertex_shader_error_message[0]);
        printf("%s\n", vertex_shader_error_message);
    }

    // Compile Fragment Shader
    printf("Compiling shader : %s\n", fragment_file_path);
    char const * fragment_source_pointer = fragment_shader_code;
    glShaderSource(FragmentShaderID, 1, &fragment_source_pointer , NULL);
    glCompileShader(FragmentShaderID);

    free(fragment_shader_code);
    // Check Fragment Shader
    glGetShaderiv(FragmentShaderID, GL_COMPILE_STATUS, &Result);
    glGetShaderiv(FragmentShaderID, GL_INFO_LOG_LENGTH, &InfoLogLength);
    if ( InfoLogLength > 0 ){
        char fragment_shader_error_message[InfoLogLength+1];
        glGetShaderInfoLog(FragmentShaderID, InfoLogLength, NULL, &fragment_shader_error_message[0]);
        printf("%s\n", fragment_shader_error_message);
    }

    // Link the program
    printf("Linking program\n");
    GLuint ProgramID = glCreateProgram();
    glAttachShader(ProgramID, VertexShaderID);
    glAttachShader(ProgramID, FragmentShaderID);
    glLinkProgram(ProgramID);

    // Check the program
    glGetProgramiv(ProgramID, GL_LINK_STATUS, &Result);
    glGetProgramiv(ProgramID, GL_INFO_LOG_LENGTH, &InfoLogLength);
    if ( InfoLogLength > 0 ){
        char program_error_message[InfoLogLength+1];
        glGetProgramInfoLog(ProgramID, InfoLogLength, NULL, &program_error_message[0]);
        printf("%s\n", program_error_message);
    }

    glDetachShader(ProgramID, VertexShaderID);
    glDetachShader(ProgramID, FragmentShaderID);

    glDeleteShader(VertexShaderID);
    glDeleteShader(FragmentShaderID);

    return ProgramID;
}