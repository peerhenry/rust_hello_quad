// std
use std::ptr;
use std::process;
use std::ffi::CStr;
use std::mem;
use std::str;
use std::path::Path;
use std::rc::Rc;
// external crates
extern crate gl;
use gl::types::*;
extern crate image;
extern crate cgmath;
use cgmath::{ Matrix, SquareMatrix, Matrix3, Matrix4, One, PerspectiveFov, Point3, Vector3};
extern crate glutin;
use glutin::Window;
// custom external crates
extern crate uniforms;
use uniforms::uniform::{Uniform, UniformObserver, GlSendBehavior};
use uniforms::gl_sender::GlSender;

// internal modules
use glds::{AttributeHandles, UniformHandles, ProgramHandles, Matrices};
use camera::Camera;

pub struct ShaderProgram {
    pub handles: ProgramHandles,
    pub matrices: Matrices,
    pub camera: Camera,
}

fn calculate_view(cam_location: Point3<GLfloat>, target: Point3<GLfloat>) -> Matrix4<GLfloat>
{
  Matrix4::look_at(
    cam_location,            // camera location
    target,                       // target look at
    Vector3::new(0.0, 1.0, 0.0)   // up direction
  )
}

fn calculate_projection(fovy: f32, ratio: f32, near: f32, far: f32) -> Matrix4<GLfloat> {
  Matrix4::from(PerspectiveFov {
    fovy: cgmath::Rad::from(cgmath::Deg(fovy)),
    aspect: ratio,
    near: near,
    far: far
  })
}

fn truncate_mat4(mat4: Matrix4<GLfloat>) -> Matrix3<GLfloat>{
  Matrix3::from_cols(mat4.x.truncate(), mat4.y.truncate(), mat4.z.truncate())
}

impl ShaderProgram {
  pub fn new(window: &Window) -> ShaderProgram {

    // Print version
    let version = unsafe {
      let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
      String::from_utf8(data).unwrap()
    };
    println!("OpenGL version {}", version);

    // Create Camera
    let camera = Camera::new(0.0, 0.0, Point3::new(0.0, 1.0, -3.0));

    // Create PVM Matrices
    let aspect = {
      if let Some((width, height)) = window.get_inner_size_pixels() {
        width as f32 / height as f32
      } else {
        4.0 / 3.0
      }
    };
    let projection = calculate_projection(90.0, aspect, 0.1, 128.0);
    let target = camera.location + camera.direction;
    let view = calculate_view(camera.location, target);
    let mut matrices = Matrices{
      model: Matrix4::one(),
      view: view,
      projection: projection
    };

    // Calculate PVM
    let pvm_matrix = matrices.projection * matrices.view * matrices.model;

    // Do the unsafe OpenGL stuff
    unsafe {
      // Compile shader source code
      let v_shader = gl::CreateShader(gl::VERTEX_SHADER);
      gl::ShaderSource(v_shader, 1, [VERTEX_SHADER_CODE.as_ptr() as *const _].as_ptr(), ptr::null());
      gl::CompileShader(v_shader);

      let f_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
      gl::ShaderSource(f_shader, 1, [FRAGMENT_SHADER_CODE.as_ptr() as *const _].as_ptr(), ptr::null());
      gl::CompileShader(f_shader);

      // Check shader compilation status
      let mut result = mem::uninitialized();
      gl::GetShaderiv(v_shader, gl::COMPILE_STATUS, &mut result);
      if result == (gl::FALSE as i32)
      {
        println!("Vertex shader compilation failed!");
        let mut infolog: [GLchar; 200] = [0; 200];
        let mut il = mem::uninitialized();
        gl::GetShaderInfoLog(v_shader, 1024, &mut il, &mut infolog[0]);
        let til: [u8; 200] = mem::transmute(infolog);
        let gl_error_str = str::from_utf8(&til).unwrap();
        println!("{}", gl_error_str); // print the gl error
        process::exit(0x0100);
      }

      gl::GetShaderiv(f_shader, gl::COMPILE_STATUS, &mut result);
      if result == (gl::FALSE as i32)
      {
        println!("fragment shader compilation failed!");
        let mut infolog: [GLchar; 200] = [0; 200];
        let mut il = mem::uninitialized();
        gl::GetShaderInfoLog(v_shader, 1024, &mut il, &mut infolog[0]);
        let til: [u8; 200] = mem::transmute(infolog);
        let gl_error_str = str::from_utf8(&til).unwrap();
        println!("{}", gl_error_str); // print the gl error
        process::exit(0x0100);
      }

      // Build the shader program
      let handle = gl::CreateProgram();
      gl::AttachShader(handle, v_shader);
      gl::AttachShader(handle, f_shader);
      gl::LinkProgram(handle);
      gl::UseProgram(handle);

      // Get attribute handles
      let pos_attrib = gl::GetAttribLocation(handle, b"VertexPosition\0".as_ptr() as *const _) as GLuint;
      let normal_attrib = gl::GetAttribLocation(handle, b"VertexNormal\0".as_ptr() as *const _) as GLuint;
      let uv_attrib = gl::GetAttribLocation(handle, b"VertexTexCoord\0".as_ptr() as *const _) as GLuint;
      let attribs = AttributeHandles{
        position: pos_attrib,
        normal: normal_attrib,
        uv: uv_attrib,
      };

      // Get uniform handles
      let pvm_handle = gl::GetUniformLocation(handle, b"PVM\0".as_ptr() as *const _);
      let projection_handle = gl::GetUniformLocation(handle, b"Projection\0".as_ptr() as *const _);
      let view_model_handle = gl::GetUniformLocation(handle, b"ViewModel\0".as_ptr() as *const _);
      let normal_matrix_handle = gl::GetUniformLocation(handle, b"NormalMatrix\0".as_ptr() as *const _);
      let tloc = gl::GetUniformLocation(handle, b"Tex1\0".as_ptr() as *const _);
      let uniform_handles = UniformHandles{
        tex1: tloc,
        pvm: pvm_handle
      };

      // Check if handles were found
      if tloc < 0 {
        panic!("Uniform variable Tex1 not found!");
      }
      if pvm_handle < 0 {
        panic!("Uniform variable PVM not found!");
      }
      // Other opengl settings...
      gl::FrontFace(gl::CW); // clockwise is front
      gl::Enable(gl::CULL_FACE);  // enable back face culling
      // Enable depth test
      gl::Enable(gl::DEPTH_TEST);
      // Accept fragment if it closer to the camera than the former one
      gl::DepthFunc(gl::LESS);
      
      // -- Create Uniforms --

      // todo: 
      // finish this function
      // then make it generic
      // then use it for all the uniforms
      // then write shader with the uniforms
      fn create_uniform<T>(val: T, handle: GLint, observers: Vec<Rc<UniformObserver>>, calculation: Box<Fn()->T>, send_behavior: Option<Box<GlSendBehavior<T>>>) -> Rc<Uniform<T>> where T: Copy {
        let u = Uniform::new(val, observers, calculation, send_behavior);
        Rc::new(u)
      }

      fn create_mat3_uniform(handle: GLint, observers: Vec<Rc<UniformObserver>>) -> Rc<Uniform<Matrix3<GLfloat>>> {
        let gl_sender = GlSender::<Matrix3<GLfloat>>::new(handle);
        let boxed_sender = Box::new(gl_sender) as Box<GlSendBehavior<Matrix3<GLfloat>>>;
        let wrapped_sender = Some(boxed_sender);
        create_uniform::<Matrix3<GLfloat>>( Matrix3::from_value(1.0), handle, observers, Box::new(|| Matrix3::from_value(1.0)), wrapped_sender )
      }

      fn create_mat4_uniform(handle: GLint, observers: Vec<Rc<UniformObserver>>) -> Rc<Uniform<Matrix4<GLfloat>>> {
        let gl_sender = GlSender::<Matrix4<GLfloat>>::new(handle);
        let boxed_sender = Box::new(gl_sender) as Box<GlSendBehavior<Matrix4<GLfloat>>>;
        let wrapped_sender = Some(boxed_sender);
        create_uniform::<Matrix4<GLfloat>>( Matrix4::from_value(1.0), handle, observers, Box::new(|| Matrix4::from_value(1.0)), wrapped_sender )
      }

      fn create_partial_uniform<T>(val: T, observers: Vec<Rc<UniformObserver>>, calculation: Box<Fn()->T>) -> Rc<Uniform<T>> where T: Copy {
        let u = Uniform::new(val, observers, calculation, None);
        Rc::new(u)
      }

      fn create_mat4_partial_uniform(observers: Vec<Rc<UniformObserver>>) -> Rc<Uniform<Matrix4<GLfloat>>> {
        create_partial_uniform::<Matrix4<GLfloat>>( Matrix4::from_value(1.0), observers, Box::new(|| Matrix4::from_value(1.0)) )
      }

      // uniforms
      let u_pvm = create_mat4_uniform(pvm_handle, vec![]);
      let u_normal_matrix = create_mat3_uniform(normal_matrix_handle, vec![]);
      let u_view_model = {
        let observer1 = u_pvm.clone() as Rc<UniformObserver>;
        let observer2 = u_normal_matrix.clone() as Rc<UniformObserver>;
        create_mat4_uniform(view_model_handle, vec![observer1, observer2])
      };
      let u_projection = create_mat4_uniform(projection_handle, vec![u_pvm.clone()]);
      // partial uniforms
      let u_view = create_mat4_partial_uniform(vec![u_view_model.clone()]);
      let u_model = create_mat4_partial_uniform(vec![u_view_model.clone()]);

      // -- Set Calculations --

      // set VM calculation
      {
        let c_view = u_projection.clone();
        let c_model = u_view_model.clone();
        u_pvm.set_calculation(Box::new(move || c_view.value.get() * c_model.value.get()));
      };

      // Set NormalMatrix calculation
      {
        let u_view_model = u_view_model.clone();
        u_normal_matrix.set_calculation(Box::new( move || truncate_mat4(u_view_model.value.get()) ));
      };

      { // set PVM calculation
        let c_projection = u_projection.clone();
        let u_view_model = u_view_model.clone();
        u_pvm.set_calculation(Box::new( move || c_projection.value.get() * u_view_model.value.get() ));
      };

      // -- Send Uniforms --
      //gl::UniformMatrix4fv(pvm, 1, gl::FALSE, pvm_matrix.as_ptr());

      // Return a new ShaderProgram
      ShaderProgram {
        handles: ProgramHandles{
          program: handle as GLint,
          attributes: attribs,
          uniforms: uniform_handles,
        },
        matrices: matrices,
        camera: camera
      }
    }
  }

  pub fn set_new_aspect(&self, ratio: f32){
    let projection = calculate_projection(90.0, ratio, 0.1, 128.0);
    self.update_pvm();
  }

  // Increment view angles and update view
  pub fn incr_view_angles(&mut self, dtheta: f64, dphi: f64)
  {
    let new_theta = self.camera.theta + dtheta;
    let new_phi = self.camera.phi + dphi;
    self.camera.set_orientation(new_theta, new_phi);
    self.update_view();
  }

  // Move cam location forward and update view
  pub fn move_camera(&mut self, d_par: f64, d_orth: f64)
  {
    let sin_theta = self.camera.theta.sin();
    let cos_theta = self.camera.theta.cos();
    let dz: GLfloat = (cos_theta*d_par + sin_theta*d_orth) as GLfloat;
    let dx: GLfloat = (-sin_theta*d_par + cos_theta*d_orth) as GLfloat;
    let new_cam_loc = Point3::new(self.camera.location.x + dx , self.camera.location.y, self.camera.location.z + dz);
    self.camera.location = new_cam_loc;
    self.update_view();
  }



  // [REGION] uniform calculation and setting

    // Calculate PVM and send to OpenGL
  fn update_pvm(&self)
  {
    let pvm_handle = self.handles.uniforms.pvm;
    let pvm_matrix = self.matrices.projection * self.matrices.view * self.matrices.model;
    unsafe{ gl::UniformMatrix4fv(pvm_handle, 1, gl::FALSE, pvm_matrix.as_ptr()); }
  }

  // Calculates new view matrix, stores it, and updates PVM
  fn update_view(&mut self){
    let target = self.camera.location + self.camera.direction;
    let new_view = calculate_view(self.camera.location, target);
    self.matrices.view = new_view;
    self.update_pvm();
  }

  pub fn load_texture(&self, rel_path: String){
    // with "caro run", path is relative to project root, otherwise relative to executable.
    let img_result = image::open(&Path::new(&rel_path));
    let dyn_img = match img_result {
      Ok(img) => img,
      Err(e) => {
        println!("Failed to open image, error: {}", e);
        process::exit(0x0100);
      }
    };
    let img = dyn_img.to_rgba();
    let (width, height) = img.dimensions();
    let img_data = &img as &[u8];
    // Now that we have the image as a byte array, it's time for unsafe gl calls.
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
      // Set tex1 sampler uniform to refer to texture unit 0
      gl::Uniform1i(self.handles.uniforms.tex1, 0);
    }
  }
}

const VERTEX_SHADER_CODE: &'static [u8] = b"
#version 400
precision mediump float;

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec3 VertexNormal;
layout (location = 2) in vec2 VertexTexCoord;

out vec3 Position;
out vec3 Normal;
out vec2 TexCoord;

uniform mat4 ViewModelMatrix;
uniform mat3 NormalMatrix;
uniform mat4 Projection;
uniform mat4 PVM;
void main()
{
  gl_PointSize = 100.0;
  TexCoord = VertexTexCoord;
  Normal = VertexNormal;
  Position = vec3(PVM*vec4(VertexPosition, 1.0));
  gl_Position = PVM*vec4(VertexPosition, 1.0);
}
\0";
// normalize(NormalMatrix * VertexNormal);
// Position = vec3(ModelViewMatrix * vec4(VertexPosition, 1.0));

const FRAGMENT_SHADER_CODE: &'static [u8] = b"
#version 400

precision mediump float;

in vec3 Position;
in vec3 Normal;
in vec2 TexCoord;

uniform sampler2D Tex1;
layout (location = 0) out vec4 FragColor;
void main()
{
  vec4 texColor = texture(Tex1, TexCoord);
  FragColor = texColor;
}
\0";