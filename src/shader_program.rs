use std::ptr;
extern crate gl;
use gl::types::*;
use std::process;
use std::ffi::CStr;
use std::mem;
use std::str;
use std::iter::Iterator;

/*mod gl {
    pub use self::Gles2 as Gl;
    //include!(concat!(env!("OUT_DIR"), "/test_gl_bindings.rs"));
}*/

pub struct ShaderProgram {
    //gl: gl::Gl,
    //model: Model
    pub handle: GLuint,
    pub pos_attrib: GLuint,
    pub normal_attrib: GLuint,
    pub uv_attrib: GLuint
}

impl ShaderProgram {
  pub fn new() -> ShaderProgram {
    //let gl = gl::Gl::load_with(|ptr| gl_window.get_proc_address(ptr) as *const _);

    let version = unsafe {
      let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
      String::from_utf8(data).unwrap()
    };
    println!("OpenGL version {}", version);

    unsafe {
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

      
      let handle = gl::CreateProgram();
      gl::AttachShader(handle, v_shader);
      gl::AttachShader(handle, f_shader);
      gl::LinkProgram(handle);

      gl::UseProgram(handle);

      let pos_attrib = gl::GetAttribLocation(handle, b"VertexPosition\0".as_ptr() as *const _) as GLuint;
      let normal_attrib = gl::GetAttribLocation(handle, b"VertexNormal\0".as_ptr() as *const _) as GLuint;
      let uv_attrib = gl::GetAttribLocation(handle, b"VertexTexCoord\0".as_ptr() as *const _) as GLuint;
      
      ShaderProgram {
        handle: handle,
        pos_attrib: pos_attrib,
        normal_attrib: normal_attrib,
        uv_attrib: uv_attrib
      }
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