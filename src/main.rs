#[macro_use]
extern crate glium;

use std::fs::File;
use std::io::Read;
use std::time::{Instant, Duration};
use std::cmp::max;
use noise::{NoiseFn, OpenSimplex};
use glium::{glutin, Surface};
use glium::texture::{RawImage2d, Texture2d};
use glium::Display;
use glium::glutin::dpi::LogicalSize;

fn read_raw(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn get_raw_noise_values(width: u32, height: u32) -> Vec<Vec<f32>> {
    let noise = OpenSimplex::new();
    let mut data = Vec::new();

    for i in 0..height {
        let mut local = Vec::new();
        for j in 0..width {
            local.push(noise.get([i as f64 /  64.0, j as f64 /  64.0]) as f32);
            local.push(noise.get([i as f64 / 128.0, j as f64 / 128.0]) as f32);
            local.push(noise.get([i as f64 / 256.0, j as f64 / 256.0]) as f32);
            local.push(noise.get([i as f64 / 512.0, j as f64 / 512.0]) as f32);
        }
        data.push(local);
    }
    
    data
}

fn get_noise_texture(display: &Display, raw: &Vec<Vec<f32>>, width: u32, height: u32) -> Texture2d {
    let mut data: Vec<f32> = Vec::new();
    
    for i in 0..height {
        let row: &Vec<f32> = raw.get(i as usize).unwrap();
        data.extend(row[0..4*(width as usize)].iter());
    }

    let image = RawImage2d::from_raw_rgba(data, (width, height));
    Texture2d::new(display, image).unwrap()
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let max_dimensions = event_loop.available_monitors()
        .map(|handle| (handle.size().width, handle.size().height))
        .fold((0, 0), |(old_width, old_height), (width, height)| (max(old_width, width), max(old_height, height)));

    let wb = glutin::window::WindowBuilder::new()
        .with_title("allfarbe")
        .with_maximized(true)
        .with_min_inner_size(LogicalSize::new(max_dimensions.0, max_dimensions.1))
        .with_decorations(false);

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2]
    }

    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex { position: [ -1.0, -1.0 ], tex_coords: [ 0.0, 0.0 ] };
    let vertex2 = Vertex { position: [ -1.0,  1.0 ], tex_coords: [ 0.0, 1.0 ] };
    let vertex3 = Vertex { position: [  1.0, -1.0 ], tex_coords: [ 1.0, 0.0 ] };
    let vertex4 = Vertex { position: [  1.0,  1.0 ], tex_coords: [ 1.0, 1.0 ] };
    let shape = vec![ vertex1, vertex2, vertex3, vertex4 ];
    let shape_index = [ 0u16, 1, 2, 1, 2, 3 ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &shape_index).unwrap();

    let vertex_shader_src = read_raw("src/shader.vs");
    let fragment_shader_src = read_raw("src/shader.fs");

    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let start = Instant::now();

    let noise_data = get_raw_noise_values(max_dimensions.0, max_dimensions.1);

    let mut texture = get_noise_texture(&display, &noise_data, max_dimensions.0, max_dimensions.1);
    let mut last_width = 1000;
    let mut last_height = 1000;

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::Resized(size) => {
                    let new_width = size.width;
                    let new_height = size.height;
                    if new_width != last_width || new_height != last_height {
                        texture = get_noise_texture(&display, &noise_data, size.width, size.height);
                        last_width = new_width;
                        last_height = new_height;
                    }
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => {
                    let dimensions = display.get_framebuffer_dimensions();
                    texture = get_noise_texture(&display, &noise_data, dimensions.0, dimensions.1);
                    last_width = dimensions.0;
                    last_height = dimensions.1;
                    ()
                },
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = Instant::now() + Duration::from_nanos(64_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let t = next_frame_time.duration_since(start).as_millis() as f32;
        let uniforms = uniform! {
            matrix: [
                [  t, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            tex: &texture
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
        
    });
}