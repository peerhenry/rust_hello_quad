extern crate glutin;
use glutin::{GlContext, ControlFlow, Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput};
extern crate cgmath;
use cgmath::{ Matrix, Matrix4, One, PerspectiveFov, Point3, Vector2, Vector3 };
use shader_program::ShaderProgram;

pub struct WindowEventHandler<'a>{
  shader_program: &'a mut ShaderProgram,
  gl_window: &'a glutin::GlWindow,
  prev_x: f64,
  prev_y: f64,
}

impl<'a> WindowEventHandler<'a>{
  pub fn new(shader_program: &'a mut ShaderProgram, gl_window: &'a glutin::GlWindow) -> WindowEventHandler<'a> {
    WindowEventHandler{
      shader_program: shader_program,
      gl_window: gl_window,
      prev_x: 0.0,
      prev_y: 0.0
    }
  }

  pub fn handle(&mut self, event: glutin::WindowEvent){
    match event {
      WindowEvent::Resized(w, h) => {
        self.gl_window.resize(w, h);
        // todo: calculate aspectratio, update PVM
      },
      WindowEvent::KeyboardInput  { input, .. }  => {
        match input.state{
          ElementState::Pressed => {
            self.handle_pressed(input);
          },
          ElementState::Released => {
            self.handle_released(input);
          }
        }
      },
      WindowEvent::MouseMoved { position, .. } => {
        let mouse_x = position.0;
        let mouse_y = position.1;
        let dx = mouse_x - self.prev_x;
        let dy = mouse_y - self.prev_y;
        // set new view angles in shader program
        let dtheta = dx*0.005;
        let dphi = dy*0.005;
        self.shader_program.incr_view_angles(dtheta, dphi);
        self.prev_x = mouse_x;
        self.prev_y = mouse_y;
      },
      WindowEvent::MouseInput { state, button, .. } => {
        // device_id, state, button
      },
      _ => ()
    }
  }

  fn handle_pressed(&self, input: KeyboardInput){
    if let Some(keycode) = input.virtual_keycode
    {
      //println!("keycode: {:?}", keycode);
      match keycode {
        VirtualKeyCode::W => {
          // make a call to shader program
          /*shader_program.matrices.view = Matrix4::look_at(
            Point3::new(0.5, 1.0, -2.0),  // camera location
            Point3::new(0.0, 0.0, 0.0),   // target look at
            Vector3::new(0.0, 1.0, 0.0)   // up direction
          );
          let pvm_matrix = shader_program.matrices.projection * shader_program.matrices.view * shader_program.matrices.model;
          let pvm_handle = shader_program.uniforms.pvm;
          unsafe{ gl::UniformMatrix4fv(pvm_handle, 1, gl::FALSE, pvm_matrix.as_ptr()); }*/
        },
        VirtualKeyCode::A => println!("left"),
        VirtualKeyCode::S => println!("backward"),
        VirtualKeyCode::D => println!("right"),
        _ => ()
      }
    }
  }

  fn handle_released(&self, input: KeyboardInput){

  }
}