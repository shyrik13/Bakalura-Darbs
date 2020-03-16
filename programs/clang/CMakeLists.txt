cmake_minimum_required(VERSION 3.12)
project(clang C)

include_directories(
        "D:/OpenGL/Libraries/glfw-3.3.2/include"
        "D:/OpenGL/Libraries/cglm-0.7.1/cglm/include"
)
link_directories(
        "D:/OpenGL/Libraries/glfw-3.3.2/lib"
)

find_package(OpenGL REQUIRED)
find_package(GLEW REQUIRED STATIC)

add_executable(clang
        main.c
        objects/object.h
        common/read.c common/read.h
        common/node.c common/node.h
        common/shader.c common/shader.h
        common/texture.c common/texture.h
        )

target_link_libraries(clang mingw32 ${OPENGL_LIBRARIES} GLEW::GLEW glfw3)

FILE(COPY res/files/cube.obj DESTINATION "${CMAKE_BINARY_DIR}/res/files")
FILE(COPY res/shaders/vertex_shader.txt DESTINATION "${CMAKE_BINARY_DIR}/res/shaders")
FILE(COPY res/shaders/fragment_shader.txt DESTINATION "${CMAKE_BINARY_DIR}/res/shaders")
FILE(COPY res/shaders/test_vertex_shader.txt DESTINATION "${CMAKE_BINARY_DIR}/res/shaders")
FILE(COPY res/shaders/test_fragment_shader.txt DESTINATION "${CMAKE_BINARY_DIR}/res/shaders")
FILE(COPY res/textures/tuto-14-diffuse.dds DESTINATION "${CMAKE_BINARY_DIR}/res/textures")
FILE(COPY res/textures/tuto-14-normal.dds DESTINATION "${CMAKE_BINARY_DIR}/res/textures")
FILE(COPY res/textures/uvtemplate.dds DESTINATION "${CMAKE_BINARY_DIR}/res/textures")