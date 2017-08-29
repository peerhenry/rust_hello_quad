extern crate gl;
use gl::types::*;
use std::ptr;
use std::mem;
use glds::{Vertex, AttributeHandles};
use shader_program::ShaderProgram;

pub struct Mesh{
  pub vertices: Vec<Vertex>,
  pub indices: Vec<GLuint>,
  vao: GLuint,
  vbo: GLuint,
  ibo: GLuint
}

impl Mesh{
  // todo: give it the model uniform
  pub fn new(vertices: Vec<Vertex>, indices: Vec<GLuint>, attribute_handles: &AttributeHandles)->Mesh{

    unsafe{
      let mut vao = mem::uninitialized();
      let mut vb = mem::uninitialized();
      let mut ib = mem::uninitialized();

      // Generate vao and buffers
      gl::GenVertexArrays(1, &mut vao);
      gl::GenBuffers(1, &mut vb);
      gl::GenBuffers(1, &mut ib);

      gl::BindVertexArray(vao);

      // Send vertex data to opengl
      gl::BindBuffer(gl::ARRAY_BUFFER, vb);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * 8 * mem::size_of::<f32>()) as GLsizeiptr,   // nr of vertices * floats per vertex * size of float
        vertices.as_ptr() as *const _, 
        gl::STATIC_DRAW
      );

      // Send indices to opengl
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ib);
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER, 
        (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr, 
        indices.as_ptr() as *const _, 
        gl::STATIC_DRAW
      );
      
      let pos_attrib = attribute_handles.position;
      let normal_attrib = attribute_handles.normal;
      let uv_attrib = attribute_handles.uv;
      let stride = 8 * mem::size_of::<f32>() as GLsizei; // A vertex consists of 8 floats

      // vertex positions
      gl::VertexAttribPointer(
        pos_attrib, // attribute handle
        3,          // attribute has size 3
        gl::FLOAT,  // type is float
        gl::FALSE as GLboolean, // normalized? false
        stride,     // stride (size of a vertex)
        ptr::null() // no offset
      );

      // normals
      gl::VertexAttribPointer(
        normal_attrib, 
        3, 
        gl::FLOAT, 
        gl::FALSE as GLboolean, 
        stride,
        (3 * mem::size_of::<f32>()) as *const () as *const _  // skip the first thee; they are position
      );

      // texture coords
      gl::VertexAttribPointer(
        uv_attrib, 
        2, 
        gl::FLOAT,
        gl::FALSE as GLboolean, 
        stride,
        (6 * mem::size_of::<f32>()) as *const () as *const _  // skip the first six
      );

      gl::EnableVertexAttribArray(pos_attrib as GLuint);
      gl::EnableVertexAttribArray(normal_attrib as GLuint);
      gl::EnableVertexAttribArray(uv_attrib as GLuint);

      gl::BindVertexArray(0);

      Mesh{
        vertices: vertices,
        indices: indices,
        vao: vao,
        vbo: vb,
        ibo: ib
      }
    }
  }

  pub fn draw(&self){
    unsafe{
      gl::Enable(gl::PROGRAM_POINT_SIZE);
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindVertexArray(self.vao);
      gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
      //gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, self.indices[0] as *const _);
      //gl::DrawArrays(gl::TRIANGLES, 0, 3);
     	//gl::DrawArrays(gl::POINTS, 0, 4);

      gl::BindVertexArray(0);
    }
  }
}