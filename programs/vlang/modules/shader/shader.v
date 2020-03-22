module shader

import gl

pub fn new_shader_program(vertex_shader_src string, frament_shader_src string) int {

	// VERTEX SHADER
	vertex_shader := gl.create_shader(C.GL_VERTEX_SHADER)
	gl.shader_source(vertex_shader, 1, vertex_shader_src, 0)
	gl.compile_shader(vertex_shader)
	if gl.shader_compile_status(vertex_shader) == 0 {
		log := gl.shader_info_log(vertex_shader)
		println('shader $vertex_shader compilation failed')
		println('shader source = $vertex_shader_src')
		println('shader failed to compile')
		exit(1)
	}

	// FRAGMENT SHADER
	fragment_shader := gl.create_shader(C.GL_FRAGMENT_SHADER)
	gl.shader_source(fragment_shader, 1, frament_shader_src, 0)
	gl.compile_shader(fragment_shader)
	if gl.shader_compile_status(fragment_shader) == 0 {
		println('fragment $fragment_shader shader compilation failed')
		println('shader failed to compile')
		exit(1)
	}
	// link shaders
	shader_program := gl.create_program()
	gl.attach_shader(shader_program, vertex_shader)
	gl.attach_shader(shader_program, fragment_shader)
	gl.link_program(shader_program)
	// check for linking errors
	success := gl.get_program_link_status(shader_program)
	if success == 0 {
		println('shader compilation failed')
		println('vertex source = $vertex_shader_src')
		println('fragment source = $frament_shader_src')
		println('shader failed to compile')
		exit(1)
	}

	return shader_program
}
