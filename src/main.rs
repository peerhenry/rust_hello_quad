extern crate gl;
use gl::types::*;
extern crate cgmath;
use cgmath::{ Matrix, Matrix4, One, PerspectiveFov, Point3, Vector2, Vector3 };
extern crate glutin;
use glutin::{GlContext, ControlFlow, Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput};
use std::mem;
mod camera;
mod shader_program;
use shader_program::ShaderProgram;
mod mesh;
use mesh::Mesh;
mod glds;
use glds::Vertex;
mod w_event_handler;
use w_event_handler::WindowEventHandler;

fn main() {
  // Setup window
  let mut events_loop = glutin::EventsLoop::new();
  let window_builder = glutin::WindowBuilder::new()
    .with_title("Hello, quad!")
    .with_dimensions(1600, 900);
  let context = glutin::ContextBuilder::new()
    .with_vsync(true);
  let gl_window = glutin::GlWindow::new(window_builder, context, &events_loop).unwrap();

  unsafe {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
  }

  let mut shader_program = ShaderProgram::new(&gl_window);

  // Load texture & send texture to opengl
  shader_program.load_texture(String::from("resources\\bricks.jpg"));

  // Define data for mesh and create Mesh
  let vert = vec![
    Vertex::new(
      [-0.5, -0.5, 0.0],  // bottom left
      [0.0, 0.0, -1.0],
      [0.0, 1.0]
    ),
    Vertex::new(
      [-0.5, 0.5, 0.0],  // top left
      [0.0, 0.0, -1.0],
      [0.0, 0.0]
    ),
    Vertex::new(
      [0.5, -0.5, 0.0],  // bottom right
      [0.0, 0.0, -1.0],
      [1.0, 1.0]
    ),
    Vertex::new(
      [0.5, 0.5, 0.0],  // top right
      [0.0, 0.0, -1.0],
      [1.0, 0.0]
    )
  ];
  let ind = vec![3,1,2,2,1,0];
  //let ind = vec![0,1,2,2,1,3];
  //let ind = vec![0,1,2,3,1,2]; // DEBUG for face cull
  let mesh = Mesh::new(vert, ind, &shader_program);

  let mut event_handler = WindowEventHandler::new(&mut shader_program, &gl_window);

  let mut running = true;
  while running {
    events_loop.poll_events(|event| {
      //println!("{:?}", event);
      match event {
        Event::WindowEvent{ event, .. } => {
          match event {
            WindowEvent::Closed => { running = false; },
            _ => { event_handler.handle(event); }
          }
        },
        _ => ()
      }
    });

    unsafe{
      gl::Clear(gl::DEPTH_BUFFER_BIT);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    mesh.draw();

    gl_window.swap_buffers().unwrap();
  }
}

