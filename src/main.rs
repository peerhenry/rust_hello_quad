extern crate gl;
use gl::types::*;
extern crate cgmath;
use cgmath::Vector3;
use cgmath::Vector2;
extern crate glutin;
extern crate libc;
extern crate image;
use image::RgbaImage;
use std::mem;
use std::path::Path;
mod shader_program;
use shader_program::ShaderProgram;
mod mesh;
use mesh::Mesh;
//mod model;
//use model::Model;
mod glds;
use glds::Vertex;

use glutin::GlContext;

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

  // Running the application with "cargo run" will make Path relative to the project directory.
  // Running the built executable by itself, and the path will be relative to the executable's directory.
  // To have access to the 
  let img_result = image::open(&Path::new("resources\\bricks.jpg"));
  let dyn_img = match img_result {
    Ok(img) => img,
    Err(e) => {
      println!("Failed to open image, error: {}", e);
      std::process::exit(0x0100);
    }
  };
  let img = dyn_img.to_rgba();
  let (width, height) = img.dimensions();
  let img_data = &img as &[u8];
  unsafe{
    gl::ActiveTexture(gl::TEXTURE0);
    let mut tid = mem::uninitialized();
    gl::GenTextures(1, &mut tid);
    gl::BindTexture(gl::TEXTURE_2D, tid);
    gl::TexImage2D(gl::TEXTURE_2D, 
      0, 
      gl::RGBA as i32, 
      width as i32, 
      height as i32,
      0, 
      gl::RGBA, 
      gl::UNSIGNED_BYTE, 
      img_data.as_ptr() as *const _
    );
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    // set tex1 sampler uniform to refer to texture unit 0
    let tloc = gl::GetUniformLocation(shader_program.handle, b"Tex1\0".as_ptr() as *const _);
    gl::Uniform1i(tloc, 0); // Tex1 will point to texture unit 0
    if(tloc < 0)
    {
      println!("Uniform variable Tex1 not found!");
    }
  }

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