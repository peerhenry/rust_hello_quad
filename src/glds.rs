// GL Data structures
extern crate gl;
use gl::types::*;
extern crate cgmath;
use cgmath::{Vector3, Vector2, Matrix4};

// with cgmath vectors
pub struct Vertex{
  pub position: Vector3<GLfloat>,
  pub normal: Vector3<GLfloat>,
  pub uv: Vector2<GLfloat>
}

impl Vertex {
    pub fn new(pos: [GLfloat; 3], norm: [GLfloat; 3], tc: [GLfloat; 2]) -> Vertex {
        Vertex {
            position: Vector3::new(pos[0], pos[1], pos[2]),
            normal: Vector3::new(norm[0], norm[1], norm[2]),
            uv: Vector2::new(tc[0], tc[1]),
        }
    }
}

pub struct AttributeHandles {
  pub position: GLuint,
  pub normal: GLuint,
  pub uv: GLuint
}

pub struct UniformHandles {
  pub pvm: GLint,
  pub tex1: GLint
}

pub struct ProgramHandles {
  pub program: GLint,
  pub attributes: AttributeHandles,
  pub uniforms: UniformHandles,
}

pub struct Matrices{
  pub model: Matrix4<GLfloat>,
  pub view: Matrix4<GLfloat>,
  pub projection: Matrix4<GLfloat>
}