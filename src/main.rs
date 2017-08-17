extern crate gl;
use gl::types::*;
extern crate cgmath;
use cgmath::{ Matrix, Matrix4, One, PerspectiveFov, Point3, Vector2, Vector3 };
extern crate glutin;
use glutin::{GlContext, ControlFlow, Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput};
use std::mem;
mod shader_program;
use shader_program::ShaderProgram;
mod mesh;
use mesh::Mesh;
mod glds;
use glds::Vertex;

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

  let mut running = true;
  while running {
    events_loop.poll_events(|event| {
      //println!("{:?}", event);
      match event {
       Event::WindowEvent{ event, .. } => match event {
          WindowEvent::Closed => running = false,
          WindowEvent::Resized(w, h) => gl_window.resize(w, h),
          WindowEvent::KeyboardInput  { input, .. }  => {
            match input.state{
              ElementState::Pressed => {
                handle_pressed(input, &mut shader_program);
              },
              ElementState::Released => {
                handle_released(input);
              }
            }
          },
          _ => ()
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

fn handle_pressed(input: KeyboardInput, shader_program: &mut ShaderProgram){
  if let Some(keycode) = input.virtual_keycode
  {
    //println!("keycode: {:?}", keycode);
    match keycode {
      VirtualKeyCode::W => {
        shader_program.matrices.view = Matrix4::look_at(
          Point3::new(0.5, 1.0, -2.0),  // camera location
          Point3::new(0.0, 0.0, 0.0),   // target look at
          Vector3::new(0.0, 1.0, 0.0)   // up direction
        );
        let pvm_matrix = shader_program.matrices.projection * shader_program.matrices.view * shader_program.matrices.model;
        let pvm_handle = shader_program.uniforms.pvm;
        unsafe{ gl::UniformMatrix4fv(pvm_handle, 1, gl::FALSE, pvm_matrix.as_ptr()); }
      },
      VirtualKeyCode::A => println!("left"),
      VirtualKeyCode::S => println!("backward"),
      VirtualKeyCode::D => println!("right"),
      _ => ()
    }
  }
}

fn handle_released(input: KeyboardInput){

}