
module object

import glm
import gl
import common

pub struct Object {
pub mut :
	vertices  []f32
	textures  []f32
	normals   []f32
	model	  glm.Mat4
	vao			u32
	vbo_v		u32
	vbo_t		u32
	vbo_n		u32
	x			f32
	y			f32
	z			f32
}

pub fn create_object(vertices, textures, normals []f32) &object.Object {
	res := &object.Object {
		vertices: vertices,
		textures: textures,
		normals: normals,
		model: common.mat4(glm.f32_calloc(16)),
		vao: 0,
		vbo_v: 0,
		vbo_t: 0,
		vbo_n: 0,
		x: 0.0,
		y: 0.0,
		z: 0.0
	}
	return res
}

pub fn (obj mut object.Object) bind_buffers() {
	
	obj.vao = gl.gen_vertex_array()
	gl.bind_vao(obj.vao)
	obj.vbo_v = gl.gen_buffer()
	obj.vbo_t = gl.gen_buffer()
	obj.vbo_n = gl.gen_buffer()
	
}