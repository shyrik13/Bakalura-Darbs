// Build this example with
// v -live bounce.v
module main

import gl
import gg
import glm
import glfw
import time
import shader
import read
import object
import math

struct Game {
mut:
	gg       &gg.GG
	t        f32
	height   int
	width    int
	main_wnd &glfw.Window
	draw_fn  voidptr
	obj		 &object.Object
	prog 	 int
	vao 	 u32
	vbo_v	 u32
	vbo_t	 u32
	vbo_n	 u32
	persp  	 glm.Mat4
	view   	 glm.Mat4
	model	 glm.Mat4
	
	texture_diff_id u32
	texture_norm_id u32
	u_light 		[]f32
	
	view_id	 u32
	persp_id u32
	model_id u32
	t_dif_id u32
	t_nrm_id u32
	light_id u32
}

fn main() {

	glfw.init_glfw()
	width := 1200
	height := 600
	
	mut pos := glm.f32_calloc(3)
	pos[0] = 4.0
	pos[1] = 3.0
	pos[2] = 3.0
	
	mut direction := glm.f32_calloc(3)
	direction[0] = 0.0
	direction[1] = 0.0
	direction[2] = 0.0
	
	mut up := glm.f32_calloc(3)
	up[0] = 0.0
	up[1] = 1.0
	up[2] = 0.0
	
	mut game := &Game{
		gg: 0
		t: 0
		height: height
		width: width	
		main_wnd: 0
		draw_fn: 0
		obj: 0
		prog: 0
		vao: 0
		vbo_v: 0
		vbo_t: 0
		vbo_n: 0
		persp: glm.perspective(math.radians(45.0), 4.0/3.0, 0.1, 100.0)
		view: glm.lookat(pos, direction, up)
		model: glm.rotation_x_y_z_model(0)
		
		texture_diff_id: 0
		texture_norm_id: 0
		u_light: [0.0, 0.4, 5.7]
		
		view_id : 0
		persp_id: 0
		model_id: 0
		t_dif_id: 0
		t_nrm_id: 0
		light_id: 0
	}
	
	window := glfw.create_window(glfw.WinCfg {
		width: width
		height: height
		borderless: false
		title: 'Cube Rotating'
		ptr: game
		always_on_top: true
	})
	
	v_arr, vt_arr, vn_arr := read.read_into_object(read.file_in_string("cube.obj", 1))
	game.obj = &object.Object {
		vertices: v_arr
		textures: vt_arr
		normals : vn_arr
	}
	
	//window.onkeydown(key_down)
	game.main_wnd = window
	window.make_context_current()
	gg.init_gg()
	game.gg = gg.new_context(gg.Cfg {
		width: width
		height: height
		font_size: 20
		use_ortho: true
		window_user_ptr: 0
	})
	
	println('Starting the game loop...')
	
	vertex_shader := read.file_in_string("vertex_shader.txt" , 0)
	fragment_shader := read.file_in_string("fragment_shader.txt", 0)
	game.prog = shader.new_shader_program(vertex_shader, fragment_shader)

	game.view_id  = gl.uni_location(game.prog, "view")
	game.persp_id = gl.uni_location(game.prog, "perspective")
	game.model_id = gl.uni_location(game.prog, "model")
	game.t_dif_id = gl.uni_location(game.prog, "diffuse_tex")
	game.t_nrm_id = gl.uni_location(game.prog, "normal_tex")
	game.light_id = gl.uni_location(game.prog, "u_light")
	
	game.texture_diff_id = game.gg.create_image('tuto-14-diffuse.jpg', 0)
	game.texture_norm_id = game.gg.create_image('tuto-14-normal.png', 0)
	
	game.vao = gl.gen_vertex_array()
	gl.bind_vao(game.vao)
	game.vbo_v = gl.gen_buffer()
	game.vbo_t = gl.gen_buffer()
	game.vbo_n = gl.gen_buffer()
	
	gl.testing()
	
	go game.run()
	for {
		if window.should_close() {
			break
		}
		gl.clear()
		gl.clear_color(0, 0, 255, 255)
		gl.use_program(game.prog)
		
		gl.set_mat4_with_id(game.view_id, game.view)
		gl.set_mat4_with_id(game.persp_id, game.persp)
		gl.set_mat4_with_id(game.model_id, game.model)
		gl.set_vec3_with_id(game.light_id, &game.u_light[0])
		
		game.draw_object()
		window.swap_buffers()
		glfw.wait_events()
	}
}

fn (game mut Game) update_model() {
	game.t += 0.01
	if (game.t > 360) {
		game.t = 0
	}
	game.model = glm.rotation_x_y_z_model(game.t)
}

fn (game mut Game) run() {
	for {
		game.update_model()		
		glfw.post_empty_event() // Refresh
		time.sleep_ms(1)
	}
}

fn (game &Game) draw_object() {
	
	// TEXTURES
	gl.active_texture(C.GL_TEXTURE0)
	gl.bind_2d_texture(game.texture_diff_id)
	gl.set_int_with_id(game.t_dif_id, 0)
	
	gl.active_texture(C.GL_TEXTURE1)
	gl.bind_2d_texture(game.texture_norm_id)
	gl.set_int_with_id(game.t_nrm_id, 1)
	
	// OBJECT ARRAYS
	gl.set_vbo(game.vbo_v, game.obj.vertices, C.GL_STATIC_DRAW)
	gl.enable_vertex_attrib_array(0)
	gl.vertex_attrib_pointer(0, 3, C.GL_FLOAT, false, 0, 0)
	
	gl.set_vbo(game.vbo_t, game.obj.textures, C.GL_STATIC_DRAW)
	gl.enable_vertex_attrib_array(1)
	gl.vertex_attrib_pointer(1, 2, C.GL_FLOAT, false, 0, 0)
	
	gl.set_vbo(game.vbo_n, game.obj.normals, C.GL_STATIC_DRAW)
	gl.enable_vertex_attrib_array(2)
	gl.vertex_attrib_pointer(2, 3, C.GL_FLOAT, false, 0, 0)
	
	// OBJECT DRAW
	gl.draw_arrays(C.GL_TRIANGLES, 0, game.obj.vertices.len)
	
	// DISABLE ARRAYS
	gl.disable_vertex_attrib_array(0)
	gl.disable_vertex_attrib_array(1)
	gl.disable_vertex_attrib_array(2)
	
}
