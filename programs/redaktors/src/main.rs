#[macro_use]
extern crate glium;
extern crate image;
extern crate rand;
extern crate chrono;
extern crate cgmath;

mod common;
mod entity;

use crate::cgmath::*;
use std::collections::HashMap;

const OBJECTS_NAMES: &'static [&'static str] = &["plane", "cube", "barrel", "boulder"];
const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let mut t: f32 = -0.5;
    let mut c = 0.0;
    let mut s = 0.0;

    let event_loop = glutin::event_loop::EventLoop::new();
    let mut wb = glutin::window::WindowBuilder::new().with_inner_size(
        glutin::dpi::PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    );
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut vec_objects: Vec<entity::Object> = Vec::new();

    let mut shape_map = HashMap::new();
    let mut vertex_map = HashMap::new();
    let mut diffuse_texture_map = HashMap::new();
    let mut normal_texture_map = HashMap::new();

    for object_name in OBJECTS_NAMES {
        let vertex = common::loader::load_into_vertex_vector(&format!("{}{}", object_name, ".obj"));
        vertex_map.insert(object_name.to_string(), vertex.clone());
        shape_map.insert(object_name.to_string(), glium::vertex::VertexBuffer::new(&display, &vertex).unwrap());
        diffuse_texture_map.insert(object_name.to_string(), glium::texture::SrgbTexture2d::new(&display, common::loader::load_diffuse_texture(object_name)).unwrap());
        normal_texture_map.insert(object_name.to_string(), glium::texture::Texture2d::new(&display, common::loader::load_normal_texture(object_name)).unwrap());
    }

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let indices_vertex = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    // Program init from shader programs
    let vertex_shader_src = &common::loader::load_string("vertex_shader.txt");
    let fragment_shader_src = &common::loader::load_string("fragment_shader.txt");
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let vertex_shader_src = &common::loader::load_string("vertex_shader_vector.txt");
    let fragment_shader_src = &common::loader::load_string("fragment_shader_vector.txt");
    let program_vector = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut camera = entity::camera::Camera::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]);

    let mut mouse_ray = entity::mouse_picker_test::MouseRay::new(&camera, [SCREEN_WIDTH, SCREEN_HEIGHT]);

    let u_light :[f32; 3] = [3.4, 0.4, -0.7];

    let mut can = true;

    event_loop.run(move |event, _, control_flow| {

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(00_000_100);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Render params
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,

                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == glutin::event::ElementState::Pressed {

                        if input.scancode >= 2 && input.scancode < 11 {
                            let index = input.scancode - 2;
                            if (index as usize) < OBJECTS_NAMES.len() {
                                camera.choosen_object_num = index;
                                println!("Selected : {}", OBJECTS_NAMES[camera.choosen_object_num as usize]);
                            }
                        }

                        match input.virtual_keycode {
                            Some(glutin::event::VirtualKeyCode::W) => {
                                camera.change_position(0.0, 0.0, 0.1);
                            },
                            Some(glutin::event::VirtualKeyCode::A) => {
                                camera.change_position(-0.1, 0.0, 0.0);
                            },
                            Some(glutin::event::VirtualKeyCode::S) => {
                                camera.change_position(0.0, 0.0, -0.1);
                            },
                            Some(glutin::event::VirtualKeyCode::D) => {
                                camera.change_position(0.1, 0.0, 0.0);
                            },
                            Some(glutin::event::VirtualKeyCode::Space) => {
                                let mut object :entity::Object = entity::Object::new(OBJECTS_NAMES[camera.choosen_object_num as usize]);
                                object.set_x_y_z(camera.position[0], camera.position[1], camera.position[2]);
                                //object.init_gl_object_model(0.0, 1.0);
                                object.id = vec_objects.len() as i32;
                                vec_objects.push(object);
                            }
                            _ => return,
                        }
                    }
                    return;
                },
                glutin::event::WindowEvent::MouseInput { button, state,  .. } => {
                    match button {
                        glutin::event::MouseButton::Right => {
                            match state {
                                glutin::event::ElementState::Pressed => {
                                    camera.can_rotate = true;
                                },
                                glutin::event::ElementState::Released => {
                                    camera.can_rotate = false;
                                }
                            }
                        },
                        _ => return,
                    }
                },
                glutin::event::WindowEvent::MouseWheel { delta, .. } => {
                    match delta {
                        glutin::event::MouseScrollDelta::LineDelta(0.0, 1.0) => {
                            camera.change_position(0.0, -0.1, 0.0);
                        },
                        glutin::event::MouseScrollDelta::LineDelta(0.0, -1.0) => {
                            camera.change_position(0.0, 0.1, 0.0);
                        },
                        _ => return,
                    }
                },
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    if camera.can_rotate {
                        camera.change_direction(position.x as f32, position.y as f32);
                    } else {
                        camera.last_mouse_position = [position.x as f32, position.y as f32];
                    }

                    mouse_ray = entity::mouse_picker_test::MouseRay::new(&camera, [SCREEN_WIDTH, SCREEN_HEIGHT]);

                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let (width, height) = target.get_dimensions();
        camera.calculate_projection_matrix(width as f32, height as f32);

        let mut min_distance = std::f32::MAX;

        let mut closest_object_id = -1;

        for object in vec_objects.iter_mut() {

            object.init_gl_object_model(c, s);

            let mut model_view: Matrix4<f32> = Matrix4::from(object.model);

            let mut model_view_3d: Matrix3<f32> = Matrix3 {
                x: Vector3 {
                    x: model_view.x.x,
                    y: model_view.x.y,
                    z: model_view.x.z
                },
                y: Vector3 {
                    x: model_view.y.x,
                    y: model_view.y.y,
                    z: model_view.y.z
                },
                z: Vector3 {
                    x: model_view.z.x,
                    y: model_view.z.y,
                    z: model_view.z.z
                }
            };


            model_view_3d.invert();
            model_view_3d.transpose();

            let mouse_ray: Vector3<f32> = Vector3::from(entity::mouse_picker::calculate_mouse_ray(&camera, [SCREEN_WIDTH, SCREEN_HEIGHT]));
            let object_position = Vector3 {
                x: object.x,
                y: object.y,
                z: object.z
            };

            let mut t = std::f32::MAX;

            let mut max_radius: f32 = 0.0;

            let view_matrix_3d: Matrix3<f32> = Matrix3 {
                x: Vector3 {
                    x: camera.view_matrix.x.x,
                    y: camera.view_matrix.x.y,
                    z: camera.view_matrix.x.z
                },
                y: Vector3 {
                    x: camera.view_matrix.y.x,
                    y: camera.view_matrix.y.y,
                    z: camera.view_matrix.y.z
                },
                z: Vector3 {
                    x: camera.view_matrix.z.x,
                    y: camera.view_matrix.z.y,
                    z: camera.view_matrix.z.z
                }
            };

            view_matrix_3d.invert();
            view_matrix_3d.transpose();

            let div = camera.position - object_position;

            for vertex in &vertex_map[object.name] {

                let world_pos_3d: Vector3<f32> = view_matrix_3d * Vector3::from([vertex.position[0] ,vertex.position[1], vertex.position[2]]);

                world_pos_3d.normalize();
                let radius: f32 = (world_pos_3d.x.powf(2.0) + world_pos_3d.y.powf(2.0) + world_pos_3d.z.powf(2.0)).sqrt();

                if radius > max_radius {
                    max_radius = radius;
                }

            }

            let b = mouse_ray.dot(div);
            let c = div.dot(div) - max_radius.powf(2.0);

            if  b.powf(2.0) >= c {

                let distance = (div.x.powf(2.0) + div.y.powf(2.0) + div.z.powf(2.0)).sqrt();

                if distance < min_distance {
                    closest_object_id = object.id;
                    min_distance = distance;
                }
            }

        }

        for object in vec_objects.iter_mut() {

            let view_matrix: [[f32; 4]; 4] = camera.view_matrix.into();
            let projection_matrix: [[f32; 4]; 4] = camera.projection_matrix.into();
            let closest: bool = object.id == closest_object_id;

            let uniforms = uniform! {
                model: object.model,
                view: view_matrix,
                u_light: u_light,
                perspective: projection_matrix,
                diffuse_tex: &diffuse_texture_map[object.name],
                normal_tex: &normal_texture_map[object.name],
                closest: closest
            };
            // Object in scene draw
            target.draw(&shape_map[object.name], &indices, &program, &uniforms,
                        &params).unwrap();
        }

        target.finish().unwrap();

        t += 0.001;
        if t > 360.0 {
            t = 0.0;
        }

        c = t.cos();
        s = t.sin();

    });
}