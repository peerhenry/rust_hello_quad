extern crate gl;
use gl::types::*;
extern crate cgmath;
use cgmath::Vector3;
use cgmath::Vector2;

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