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
import common

struct Game {
mut:
	gg       &gg.GG
	t        f32
	height   int
	width    int
	main_wnd &glfw.Window
	draw_fn  voidptr
	objects  []&object.Object
	prog 	 int
	persp  	 glm.Mat4
	view   	 glm.Mat4
	
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
	pos[0] = 1.0
	pos[1] = 0.0
	pos[2] = 1.0
	
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
		objects: []
		prog: 0
		persp: common.perspective(math.radians(90.0), 4.0/3.0, 0.1, 100.0)
		view: common.lookat(pos, direction, up)
		
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
	
	mut first_obj := object.create_object(v_arr, vt_arr, vn_arr)
	
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

	game.view_id  = common.uni_location(game.prog, "view")
	game.persp_id = common.uni_location(game.prog, "perspective")
	game.model_id = common.uni_location(game.prog, "model")
	game.t_dif_id = common.uni_location(game.prog, "diffuse_tex")
	game.t_nrm_id = common.uni_location(game.prog, "normal_tex")
	game.light_id = common.uni_location(game.prog, "u_light")
	
	game.texture_diff_id = game.gg.create_image('tuto-14-diffuse.jpg', 0)
	game.texture_norm_id = game.gg.create_image('tuto-14-normal.png', 0)
	
	first_obj.bind_buffers()
	
	first_obj.x = -15.0
	first_obj.y = 0.0
	first_obj.z = -15.0
	
	game.objects << first_obj
	
	common.testing()
	
	go game.run()
	
	mut last_time := common.glfw_get_time()
	mut nb_frames := 0
	
	max := 15.0
	min := -15.0
	
	max0 := 0.0
	min0 := -30.0
	
	common.rand_srand_null()
	
	for {
		if window.should_close() {
			break
		}
		
		current_time := common.glfw_get_time()
        nb_frames++
        if ( current_time - last_time >= 1.0 ){
            printf("%f ms/frame\n", 1000.0/(f32(nb_frames)))
            nb_frames = 0
			last_time += 1.0
			
			mut new_obj := object.create_object(v_arr, vt_arr, vn_arr)
			new_obj.x = common.rand_float_between_max_min(max0, min0)
			new_obj.y = common.rand_float_between_max_min(max, min)
			new_obj.z = -25.0
			new_obj.bind_buffers()
			game.objects << new_obj
			
        }
		
		gl.clear()
		gl.clear_color(0, 0, 255, 255)
		
		for obj in game.objects {
			gl.use_program(game.prog)
			
			common.set_mat4_with_id(game.view_id, game.view)
			common.set_mat4_with_id(game.persp_id, game.persp)
			common.set_mat4_with_id(game.model_id, obj.model)
			common.set_vec3_with_id(game.light_id, &game.u_light[0])
			
			game.draw_object(obj)
			
		}
		
		window.swap_buffers()
		glfw.wait_events()
		glfw.post_empty_event()
	}
}

fn (game Game) update_model(obj mut object.Object) {
	obj.model = common.rotation_x_y_z_model(game.t, obj.x, obj.y, obj.z)
}

fn (game mut Game) run() {
	for {	
		
		for obj in game.objects {
			game.update_model(mut obj)
		}

		time.sleep_ms(1)
		
		game.t += 0.01
		if (game.t > 360) {
			game.t = 0
		}
	}
}

fn (game &Game) draw_object(obj object.Object) {
	
	// TEXTURES
	gl.active_texture(C.GL_TEXTURE0)
	gl.bind_2d_texture(game.texture_diff_id)
	gl.set_int_with_id(game.t_dif_id, 0)
	
	gl.active_texture(C.GL_TEXTURE1)
	gl.bind_2d_texture(game.texture_norm_id)
	gl.set_int_with_id(game.t_nrm_id, 1)
	
	// OBJECT ARRAYS
	gl.set_vbo(obj.vbo_v, obj.vertices, C.GL_STATIC_DRAW)
	gl.enable_vertex_attrib_array(0)
	gl.vertex_attrib_pointer(0, 3, C.GL_FLOAT, false, 0, 0)
	
	gl.set_vbo(obj.vbo_t, obj.textures, C.GL_STATIC_DRAW)
	gl.enable_vertex_attrib_array(1)
	gl.vertex_attrib_pointer(1, 2, C.GL_FLOAT, false, 0, 0)
	
	gl.set_vbo(obj.vbo_n, obj.normals, C.GL_STATIC_DRAW)
	gl.enable_vertex_attrib_array(2)
	gl.vertex_attrib_pointer(2, 3, C.GL_FLOAT, false, 0, 0)
	
	// OBJECT DRAW
	gl.draw_arrays(C.GL_TRIANGLES, 0, obj.vertices.len)
	
	// DISABLE ARRAYS
	gl.disable_vertex_attrib_array(0)
	gl.disable_vertex_attrib_array(1)
	gl.disable_vertex_attrib_array(2)
	
}
