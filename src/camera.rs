extern crate gl;
use gl::types::GLfloat;
extern crate cgmath;
use cgmath::{Vector3, Point3};

pub struct Camera{
  pub theta: f64,
  pub phi: f64,
  pub direction: Vector3<GLfloat>,
  pub location: Point3<GLfloat>
}

fn calculate_dir(theta: f64, phi: f64) -> Vector3<GLfloat> {
  let x = -phi.cos()*theta.sin();
  let y = phi.sin();
  let z = phi.cos()*theta.cos();
  Vector3::new(x as GLfloat, y as GLfloat, z as GLfloat)
}

impl Camera{
  pub fn new(theta: f64, phi: f64, location: Point3<GLfloat>) -> Camera {
    Camera{
      theta: theta,
      phi: phi,
      direction: calculate_dir(theta, phi),
      location: location
    }
  }

  pub fn set_orientation(&mut self, theta: f64, phi: f64){
    self.theta = theta;
    self.phi = phi;
    self.direction = calculate_dir(theta, phi);
  }

  pub fn set_location(&mut self, location: Point3<GLfloat>){
    self.location = location;
  }
}