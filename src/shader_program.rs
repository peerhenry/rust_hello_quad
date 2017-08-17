use std::ptr;
use std::process;
use std::ffi::CStr;
use std::mem;
use std::str;
use std::path::Path;
extern crate gl;
use gl::types::*;
extern crate image;

pub struct Attributes {
  pub position: GLuint,
  pub normal: GLuint,
  pub uv: GLuint
}

pub struct Uniforms{
  pub tex1: GLint
}

pub struct ShaderProgram {
    pub handle: GLuint,
    pub attribs: Attributes,
    pub uniforms: Uniforms,
}

impl ShaderProgram {
  pub fn new() -> ShaderProgram {

    // Print version
    let version = unsafe {
      let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
      String::from_utf8(data).unwrap()
    };
    println!("OpenGL version {}", version);

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
      if(result == (gl::FALSE as i32))
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
      if(result == (gl::FALSE as i32))
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
      let attribs = Attributes{
        position: pos_attrib,
        normal: normal_attrib,
        uv: uv_attrib,
      };

      // Get uniform handles
      let tloc = gl::GetUniformLocation(handle, b"Tex1\0".as_ptr() as *const _);
      let uniforms = Uniforms{
        tex1: tloc
      };

      // Check if handles were found
      if tloc < 0 {
        panic!("Uniform variable Tex1 not found!");
      }

      // Return a new ShaderProgram
      ShaderProgram {
        handle: handle,
        attribs: attribs,
        uniforms: uniforms,
      }
    }
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
      gl::Uniform1i(self.uniforms.tex1, 0);
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

//uniform mat4 ModelViewMatrix;
//uniform mat3 NormalMatrix;
//uniform mat4 Projection;
//uniform mat4 MVP;
void main()
{
  gl_PointSize = 100.0;
  TexCoord = VertexTexCoord;
  Normal = VertexNormal;
  Position = VertexPosition;
  gl_Position = vec4(VertexPosition, 1.0);
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