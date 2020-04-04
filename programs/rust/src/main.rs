#[macro_use]
extern crate glium;
extern crate image;
extern crate rand;

mod common;
mod objects;

use std::io::Cursor;
use rand::Rng;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let mut t: f32 = -0.5;
    let mut c = 0.0;
    let mut s = 0.0;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Object loading from file and shape creation
    let vertex = common::read::read_into_vertex_vector("cube.obj");
    let mut object :objects::Object = objects::Object::new(vertex);
    object.set_x_y_z(-3.4, 0.0, 2.7);
    object.init_gl_object_model(t.cos(), t.sin());

    let shape = glium::vertex::VertexBuffer::new(&display, &(object.vertices)).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut vec_objects: Vec<objects::Object> = Vec::new();
    vec_objects.push(object);

    // Texture loading with normal map
    let image = image::load(Cursor::new(&include_bytes!("texture/tuto-14-diffuse.jpg")[..]),
                            image::ImageFormat::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("texture/tuto-14-normal.png")[..]),
                            image::ImageFormat::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();

    // Program init from shader programs
    let vertex_shader_src = &common::read::read_in_string("vertex_shader.txt");
    let fragment_shader_src = &common::read::read_in_string("fragment_shader.txt");
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut last_time = std::time::Instant::now();
    let mut nb_frames = 0.0;

    let view = objects::camera::view_matrix(&[9.0, 1.0, 1.0], &[-5.0, -2.0, 1.0], &[0.0, 1.0, 0.0]);

    let u_light :[f32; 3] = [3.4, 0.4, -0.7];

    let max :f32 = 5.0;
    let min :f32 = -5.0;

    let max0 :f32 = 18.0;
    let min0 :f32 = -12.0;

    event_loop.run(move |event, _, control_flow| {

        let current_time = std::time::Instant::now();
        nb_frames += 1.0;

        if current_time - last_time >= std::time::Duration::from_secs(1) {
            std::println!("{} ms/frame", (1000.0/nb_frames));
            nb_frames = 0.0;
            last_time = std::time::Instant::now();

            let mut rng = rand::thread_rng();

            let vertex = common::read::read_into_vertex_vector("cube.obj");
            let mut new_object :objects::Object = objects::Object::new(vertex);
            new_object.set_x_y_z(-3.7, rng.gen_range(min, max), rng.gen_range(min0, max0));
            //new_object.set_x_y_z(-4.4, 3.0, 0.7);
            new_object.init_gl_object_model(c, s);

            vec_objects.push(new_object);
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(00_000_100);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Render params
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        };

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
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

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
            //let aspect_ratio = 4.0 / 3.0;

            let degrees: f32 = 90.0;
            let fov: f32 = (degrees).to_radians();
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]

        };

        for object in vec_objects.iter_mut() {

            object.init_gl_object_model(c, s);

            let uniforms = uniform! {
                model: object.model,
                view: view,
                u_light: u_light,
                perspective: perspective,
                diffuse_tex: &diffuse_texture,
                normal_tex: &normal_map
            };
            // Object in scene draw
            target.draw(&shape, &indices, &program, &uniforms,
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