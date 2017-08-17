/*extern crate gl;
use gl::types::*;
use std::mem;
use std::ptr;

use mesh::Mesh;
use shader_program::ShaderProgram;

pub struct Model {
  //mesh: &Mesh,
  //vbo: GLuint,
  vao: GLuint
}

impl Model{
  pub fn new(mesh: &Mesh, program: &ShaderProgram) -> Model {
    let vertices = mesh.get_vertices();
    let vertex_slice = vertices.as_slice();

    // convert the vector to an array

    unsafe{
      let mut vao = mem::uninitialized();
      let mut vb = mem::uninitialized();
      let mut ib = mem::uninitialized();

      gl::GenVertexArrays(1, &mut vao);
      gl::BindVertexArray(vao);

      gl::GenBuffers(1, &mut vb);
      gl::BindBuffer(gl::ARRAY_BUFFER, vb);
      gl::BufferData(gl::ARRAY_BUFFER,
                          (vertex_slice.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                          vertex_slice.as_ptr() as *const _, gl::STATIC_DRAW);

      gl::GenBuffers(1, &mut ib);
      gl::BindBuffer(gl::ARRAY_BUFFER, ib);
      

      let pos_attrib = program.pos_attrib;
      let color_attrib = program.color_attrib;
      gl::VertexAttribPointer(pos_attrib as gl::types::GLuint, 2, gl::FLOAT, 0,
                                  5 * mem::size_of::<f32>() as gl::types::GLsizei,
                                  ptr::null());
      gl::VertexAttribPointer(color_attrib as gl::types::GLuint, 3, gl::FLOAT, 0,
                                  5 * mem::size_of::<f32>() as gl::types::GLsizei,
                                  (2 * mem::size_of::<f32>()) as *const () as *const _);
      gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
      gl::EnableVertexAttribArray(color_attrib as gl::types::GLuint);

      Model{
        //mesh: mesh,
        //vbo: vb,
        vao: vao
      }
    }
  }

  pub fn draw(&self) {
    unsafe{
    //gl::EnableClientState(gl::VERTEX_ARRAY);  // Enable the vertex array
    gl::BindVertexArray( self.vao ); //Tell opengl where the vertices are
    gl::DrawArrays(gl::TRIANGLES, 0, 3);
    //gl::DisableClientState(gl::VERTEX_ARRAY);
    }
  }

}*/