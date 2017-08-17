extern crate gl;
use gl::types::*;
extern crate cgmath;
use cgmath::Vector3;
use cgmath::Vector2;
extern crate glutin;
use glutin::GlContext;
//extern crate libc;
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
  let window = glutin::WindowBuilder::new()
    .with_title("Hello, world!")
    .with_dimensions(1024, 768);
  let context = glutin::ContextBuilder::new()
    .with_vsync(true);
  let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

  unsafe {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.0, 154.0/255.0, 206.0/255.0, 235.0/255.0);
  }

  let shader_program = ShaderProgram::new();

  // Load texture & send texture to opengl
  shader_program.load_texture(String::from("resources\\bricks.jpg"));

  // Define data for mesh
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
  let ind = vec![0,1,2,2,1,3];
  // create mesh
  let mesh = Mesh::new(vert, ind, &shader_program);

  // create model
  //let model = Model::new(&mesh, &shader_program);

  unsafe{
    gl::FrontFace(gl::CW); // clockwise is front
    gl::Enable(gl::CULL_FACE);  // enable back face culling
  }

  let mut running = true;
  while running {
    events_loop.poll_events(|event| {
      match event {
        glutin::Event::WindowEvent{ event, .. } => match event {
          glutin::WindowEvent::Closed => running = false,
          glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
          _ => ()
        },
        _ => ()
      }
    });

    unsafe{
      gl::Clear(gl::COLOR_BUFFER_BIT); 
    }
    mesh.draw();

    gl_window.swap_buffers().unwrap();
  }
}