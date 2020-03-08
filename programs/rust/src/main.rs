#[macro_use]
extern crate glium;
extern crate image;

mod common;
mod objects;

use std::io::Cursor;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Object loading from file and shape creation
    let object = common::read::read_into_vertex_vector("cube.obj");
    let shape = glium::vertex::VertexBuffer::new(&display, &object).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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

    let mut t: f32 = -0.5;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
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

        t += 0.01;
        if t > 360.0 {
            t = 0.0;
        }

        let mut target = display.draw();

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
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

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let c = t.cos();
        let s = t.sin();
        let uniforms = uniform! {
            model: [
                // object rotation matrix around x, y, z
                [c.powi(2), -c*s, s, 0.0],
                [c*(s.powi(2)+s), c.powi(2)-s.powi(3), -c*s, 0.0],
                [s*(s-c.powi(2)), c*(s.powi(2)+s), c.powi(2), 0.0],
                [-3.4, 0.0, 0.7f32, 1.0f32] // position in space (x = -3.4, y = 0.0, z = 0.7)
            ],
            view: objects::camera::view_matrix(&[8.0, 1.0, 1.0], &[-5.0, -2.0, 1.0], &[0.0, 1.0, 0.0]),
            u_light: [3.4, 0.4, -0.7f32],
            perspective: perspective,
            diffuse_tex: &diffuse_texture,
            normal_tex: &normal_map
        };

        // Object in scene draw
        target.draw(&shape, &indices, &program, &uniforms,
                    &params).unwrap();
        target.finish().unwrap();
    });
}