// This module is an abstraction over the vector operations and
// affine transform methods provided by the vecmat library.
use vecmat::{vec::Vec3, mat::Mat3, map::Affine3};

pub fn translate(x: Vec3<f64>, trans: Vec3<f64>) -> Vec3<f64>{
    x + trans
}

pub fn rotate_about_x_axis(v: Vec3<f64>, angle: f64) -> Vec3<f64>{
    let rotx = Mat3::<f64>::from(1.0, 0.0, 0.0,
                                0.0, angle.cos(), -1.0*angle.sin(),
                                0.0, angle.sin(), angle.cos());
    let origin = Vec3::<f64>::zero();
    let transform = Affine3::from(rotx, origin);
    transform.map_vec(v)
}

pub fn rotate_about_y_axis(v: Vec3<f64>, angle: f64) -> Vec3<f64>{
    let rotx = Mat3::<f64>::from(angle.cos(), 0.0, angle.sin(),
                                0.0, 1.0, 0.0,
                                -1.0*angle.sin(), 0.0, angle.cos());
    let origin = Vec3::<f64>::zero();
    let transform = Affine3::from(rotx, origin);
    transform.map_vec(v)
}

pub fn rotate_about_z_axis(v: Vec3<f64>, angle: f64) -> Vec3<f64>{
    let rotx = Mat3::<f64>::from(angle.cos(), -1.0*angle.sin(), 0.0,
                                angle.sin(), angle.cos(), 0.0,
                                0.0, 0.0, 1.0);
    let origin = Vec3::<f64>::zero();
    let transform = Affine3::from(rotx, origin);
    transform.map_vec(v)
}
