extern crate glutin;
use glutin::{GlContext, ControlFlow, Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput};
extern crate cgmath;
use cgmath::{ Matrix, Matrix4, One, PerspectiveFov, Point3, Vector2, Vector3 };
use shader_program::ShaderProgram;

const one_over_sqrt_two: f64 = 0.7071067811865475244;

pub struct ControlState{
  event_happened: bool,
  forward_is_down: bool,
  backward_is_down: bool,
  left_is_down: bool,
  right_is_down: bool,
}

pub fn new_control_state()->ControlState{
  ControlState{
    event_happened: false,
    forward_is_down: false,
    backward_is_down: false,
    left_is_down: false,
    right_is_down: false
  }
}

pub enum CamDir{
  Forward,
  Backward,
  Left,
  Right
}

pub struct WindowEventHandler<'a>{
  shader_program: &'a mut ShaderProgram,
  gl_window: &'a glutin::GlWindow,
  prev_x: f64,
  prev_y: f64,
  dx: f64,
  dy: f64,
  control_state: ControlState,
  move_dir: (f64, f64),
  pub quit: bool
}

impl<'a> WindowEventHandler<'a>{
  pub fn new(shader_program: &'a mut ShaderProgram, gl_window: &'a glutin::GlWindow) -> WindowEventHandler<'a> {
    WindowEventHandler{
      shader_program: shader_program,
      gl_window: gl_window,
      prev_x: 0.0,
      prev_y: 0.0,
      dx: 0.0,
      dy: 0.0,
      control_state: new_control_state(),
      move_dir: (0.0,0.0),
      quit: false
    }
  }

  pub fn handle(&mut self, event: glutin::WindowEvent){

    let mut mouse_has_moved = false;
    self.control_state.event_happened = false;

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
        mouse_has_moved = true;
        let mouse_x = position.0;
        let mouse_y = position.1;
        self.dx = mouse_x - self.prev_x;
        self.dy = mouse_y - self.prev_y;
        self.prev_x = mouse_x;
        self.prev_y = mouse_y;
      },
      WindowEvent::MouseInput { state, button, .. } => {
        // device_id, state, button
      },
      _ => ()
    }

    if self.control_state.event_happened
    {
      let mut new_dir: (f64, f64) = (0.0,0.0);
      let mut move_para: bool = false;
      let mut move_ortho: bool = false;
      // recalculate move_dir
      if self.control_state.forward_is_down
      {
        new_dir.0 -= 1.0;
        move_para = true;
      }
      if self.control_state.backward_is_down
      {
        new_dir.0 += 1.0;
        if(new_dir.0 > 0.0) { move_para = true; }
        else { move_para = false; }
      }
      if self.control_state.left_is_down
      {
        new_dir.1 -= 1.0;
        move_ortho = true;
      }
      if self.control_state.right_is_down
      {
        new_dir.1 += 1.0;
        if(new_dir.1 > 0.0) { move_ortho = true; }
        else {move_ortho = false;}
      }
      // normalize
      if new_dir.0 != 0.0 && new_dir.1 != 0.0
      {
        new_dir.0 = new_dir.0 * one_over_sqrt_two;
        new_dir.1 = new_dir.1 * one_over_sqrt_two;
      }
      // save new dir
      self.move_dir = new_dir;
    }
    
    let m_fac = 0.02f64;
    let d_para = m_fac*self.move_dir.0;
    let d_ortho = m_fac*self.move_dir.1;
    self.shader_program.move_camera(d_para, d_ortho);

    if mouse_has_moved
    {
      // set new view angles in shader program
      let dtheta = self.dx*0.005;
      let dphi = self.dy*0.005;
      self.shader_program.incr_view_angles(dtheta, dphi);
    }
  }

  fn handle_pressed(&mut self, input: KeyboardInput){
    if let Some(keycode) = input.virtual_keycode
    {
      match keycode {
        VirtualKeyCode::Escape => self.quit = true,
        VirtualKeyCode::W => self.set_movement(CamDir::Forward, true),
        VirtualKeyCode::A => self.set_movement(CamDir::Left, true),
        VirtualKeyCode::S => self.set_movement(CamDir::Backward, true),
        VirtualKeyCode::D => self.set_movement(CamDir::Right, true),
        _ => ()
      }
    }
  }

  fn handle_released(&mut self, input: KeyboardInput){
    if let Some(keycode) = input.virtual_keycode
    {
      match keycode {
        VirtualKeyCode::W => self.set_movement(CamDir::Forward, false),
        VirtualKeyCode::A => self.set_movement(CamDir::Left, false),
        VirtualKeyCode::S => self.set_movement(CamDir::Backward, false),
        VirtualKeyCode::D => self.set_movement(CamDir::Right, false),
        _ => ()
      }
    }
  }

  fn set_movement(&mut self, direction: CamDir, value: bool){
    self.control_state.event_happened = true;
    match direction{
      CamDir::Forward => self.control_state.forward_is_down = value,
      CamDir::Backward => self.control_state.backward_is_down = value,
      CamDir::Left => self.control_state.left_is_down = value,
      CamDir::Right => self.control_state.right_is_down = value,
    }
  }
}