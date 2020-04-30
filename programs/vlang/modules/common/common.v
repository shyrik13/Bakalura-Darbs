
module common

import glm
import math
import rand


pub fn testing() {
	C.glEnable(C.GL_DEPTH_TEST) // Depth Testing
    C.glDepthFunc(C.GL_LEQUAL)
    C.glDisable(C.GL_CULL_FACE)
    C.glCullFace(C.GL_BACK)
}

pub fn set_mat4_with_id(mat4_id u32, m glm.Mat4) {
	C.glUniformMatrix4fv(mat4_id, 1, false, m.data)
}

pub fn set_vec3_with_id(vec3_id u32, m &f32) {
	C.glUniform3fv(vec3_id, 1, m)
}

pub fn uni_location(program_id int, key string) int {
	return C.glGetUniformLocation(program_id, key.str)
}

pub fn rotation_x_y_z_model(t, x, y, z f32) glm.Mat4 {
	c := math.cosf(t)
	s := math.sinf(t)
	mut out := glm.f32_calloc(16)
	out[0] = math.powf(c, 2)
	out[1] = -c*s
	out[2] = s
	out[3] = 0.0
	out[4] = c*(math.powf(s, 2)+s)
	out[5] = math.powf(c, 2)-math.powf(s, 3)
	out[6] = -c*s
	out[7] = 0.0
	out[8] = s*(s-math.powf(c, 2))
	out[9] = c*(math.powf(s, 2)+s)
	out[10] = math.powf(c, 2)
	out[11] = 0.0
	out[12] = x
	out[13] = y
	out[14] = z
	out[15] = 1.0
	
	return mat4(out)
}

pub fn perspective(fovy, aspect, nearVal, farVal f32) glm.Mat4 {

	f  := 1.0 / math.tanf(fovy * 0.5)
	f2 := 1.0 / (nearVal - farVal)
	
	mut out := glm.f32_calloc(16)
	out[0] = f / aspect
	out[1] = 0.0
	out[2] = 0.0
	out[3] = 0.0
	out[4] = 0.0
	out[5] = f
	out[6] = 0.0
	out[7] = 0.0
	out[8] = 0.0
	out[9] = 0.0
	out[10] = (nearVal + farVal) * f2
	out[11] = -1.0
	out[12] = 0.0
	out[13] = 0.0
	out[14] = 2.0 * nearVal * farVal * f2
	out[15] = 0.0
	
	return mat4(out)
}

pub fn lookat(position, direction, up &f32) glm.Mat4 {
	
	f := glm_vec3_normalize(glm_vec3_sub(direction, position))
	s := glm_vec3_crossn(f, up)
	u := glm_vec3_cross(s, f)
	
	mut out := glm.f32_calloc(16)
	out[0] = s[0]
	out[1] = u[0]
	out[2] =-f[0]
	out[3] = 0.0
	out[4] = s[1]
	out[5] = u[1]
	out[6] =-f[1]
	out[7] = 0.0
	out[8] = s[2]
	out[9] = u[2]
	out[10] =-f[2]
	out[11] = 0.0
	out[12] =-glm_vec3_dot(s, position)
	out[13] =-glm_vec3_dot(u, position)
	out[14] = glm_vec3_dot(f, position)
	out[15] = 1.0
	
	return mat4(out)
}

fn glm_vec3_normalize(v &f32) &f32 {
	
	norm := glm_vec3_norm(v)
	
	if (norm == 0.0) {
		mut out := glm.f32_calloc(3)
		out[1] = 0.0
		out[2] = 0.0
		out[3] = 0.0
		return out
	}
	
	return glm_vec3_scale(v, 1.0 / norm)
}

fn glm_vec3_dot(a, b &f32) f32 {
	return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn glm_vec3_norm2(v &f32) f32 {
	return glm_vec3_dot(v, v)
}

fn glm_vec3_norm(v &f32) f32 {
	return math.sqrtf(glm_vec3_norm2(v))
}

fn glm_vec3_crossn(a, b &f32) &f32 {
  return glm_vec3_normalize(glm_vec3_cross(a, b))
}

fn glm_vec3_cross(a, b &f32) &f32 {
  mut out := glm.f32_calloc(3)
  out[0] = a[1] * b[2] - a[2] * b[1]
  out[1] = a[2] * b[0] - a[0] * b[2]
  out[2] = a[0] * b[1] - a[1] * b[0]
  return out
}

fn glm_vec3_scale(v &f32, s f32) &f32 {
	mut out := glm.f32_calloc(3)
	out[0] = v[0] * s
	out[1] = v[1] * s
	out[2] = v[2] * s
	return out
}

fn glm_vec3_sub(a, b &f32) &f32 {
	mut out := glm.f32_calloc(3)
	out[0] = a[0] - b[0]
	out[1] = a[1] - b[1]
	out[2] = a[2] - b[2]
	return out
}

pub fn view_matrix(position, direction, up &f32) glm.Mat4 {
	
	mut len := direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2]
	len = math.sqrtf(len)
	f := [direction[0] / len, direction[1] / len, direction[2] / len]

    s := [
			up[1] * f[2] - up[2] * f[1],
			up[2] * f[0] - up[0] * f[2],
			up[0] * f[1] - up[1] * f[0]
		]

	len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2]
	len = math.sqrtf(len)
	s_norm := [s[0] / len, s[1] / len, s[2] / len]
	
    u := [
			f[1] * s_norm[2] - f[2] * s_norm[1],
			f[2] * s_norm[0] - f[0] * s_norm[2],
			f[0] * s_norm[1] - f[1] * s_norm[0]
		]
	
    p := [
			-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
			-position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
			-position[0] * f[0] - position[1] * f[1] - position[2] * f[2]
		]
		
	mut out := glm.f32_calloc(16)
	out[0]  = s_norm[0] 
	out[1]  = u[0] 
	out[2]  = f[0] 
	out[3]  = 0.0
	out[4]  = s_norm[1] 
	out[5]  = u[1] 
	out[6]  = f[1] 
	out[7]  = 0.0
	out[8]  = s_norm[2] 
	out[9]  = u[2] 
	out[10] = f[2] 
	out[11] = 0.0
	out[12] = p[0]      
	out[13] = p[1] 
	out[14] = p[2] 
	out[15] = 1.0
	//out := [
	//	s_norm[0], u[0], f[0], 0.0,
	//	s_norm[1], u[1], f[1], 0.0,
	//	s_norm[2], u[2], f[2], 0.0,
	//	p[0], p[1], p[2], 1.0
	//]
    return mat4(out)
}

pub fn mat4(f &f32) glm.Mat4 {
	res := glm.Mat4 {
		data: f
	}
	return res
}


pub fn glfw_get_time() f32 {
	return C.glfwGetTime()
}

pub fn rand_srand_null() {
	s := int(C.time(C.NULL))
	rand.seed(s)
}

pub fn rand_float_between_max_min(max, min f32) f32 {
	return (f32(C.rand())/f32(C.RAND_MAX) * (max - min)) + min
}

pub fn delete_buffer(vbo u32) {
	C.glDeleteBuffers(1, &vbo)
}

pub fn delete_vertex_array(vao u32) {
	C.glDeleteVertexArrays(1, &vao)
}

pub fn gen_buffer(vbo &u32) {
	//vbo = u32(0)
	C.glGenBuffers(1, vbo)
}

pub fn gen_vertex_array(vao &u32) {
	//vao = u32(0)
	C.glGenVertexArrays(1, vao)
}
