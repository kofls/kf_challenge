extern crate vecmat;
extern crate approx;
// https://stackoverflow.com/questions/3913832/what-is-a-3d-vector-and-how-does-it-differ-from-a-3d-point
use vecmat::{vec::Vec2, vec::Vec3, vec::Vec4, mat::Mat3, map::Affine3};
use vecmat::*;
use vecmat::vec::Dot;
use approx::abs_diff_eq;
use std::f64;


// Surface.rs
#[derive(Debug)]
struct PlaneSurface {
    u: Vec3<f64>,
    v: Vec3<f64>,
    t: Vec3<f64>,
    center: Vec3<f64>,
    bounds: Option<Bound>,
    affine: Affine3<f64>,
    affine_inverse: Affine3<f64>,
}

impl PlaneSurface {
    pub fn new(center: Vec3<f64>, normal: Vec3<f64>, bounds: Option<Bound>) -> Self{
        
        let unit_x = Vec3::<f64>::from(1.0, 0.0, 0.0);
        let unit_z = Vec3::<f64>::from(0.0, 0.0, 1.0);

        let t = normal.normalize();
        let u = if unit_z.dot(t) < f64::EPSILON {
                    unit_z.cross(t).normalize()
                } else {
                    unit_x.cross(t).normalize()
                };
        let v = t.cross(u);
        
        let basis = Mat3::from(u[0], v[0], t[0],
                                u[1], v[1], t[1],
                                u[2], v[2], t[2]);

        let affine = Affine3::from(basis, center);
        let affine_inverse = affine.inverse();

        PlaneSurface {u, v, t, center, bounds, affine, affine_inverse}
    }

    pub fn is_point_on_surface(&self, point: &Vec3<f64>) -> bool{
        self.is_point_on_plane(&point) && self.is_point_in_bounds(&point)
    }

    fn is_point_on_plane(&self, point: &Vec3<f64>) -> bool{
        let distance = *point - self.center;
        // https://doc.rust-lang.org/nightly/std/f64/constant.EPSILON.html
        abs_diff_eq!(self.t.dot(distance), 0.0, epsilon = f64::EPSILON)
    }

    fn is_point_in_bounds(&self, point: &Vec3<f64>) -> bool{
        match &self.bounds {
            None => {true},
            Some(Bound::Rectangle(bound)) => {
                let local_point = self.global_to_local_2d(&point);
                (local_point[0].abs() <= bound.x_half_bound + f64::EPSILON) && 
                (local_point[1].abs() <= bound.y_half_bound + f64::EPSILON)
            },
            _ => {unimplemented!();},
        }
    }
    
    pub fn global_to_local_2d(&self, global_point: &Vec3<f64>) -> Vec2<f64> {
        let distance = *global_point - self.center;
        Vec2::from(self.u.dot(distance), self.v.dot(distance))
    }

    pub fn local_2d_to_global(&self, local_point: &Vec2<f64>) -> Vec3<f64> {
        self.center + local_point[0]*self.u + local_point[1]*self.v
    }

    pub fn local_to_global(&self, local_point: &Vec3<f64>) -> Vec3<f64> {
        self.affine.map_vec(*local_point)
    }

    pub fn global_to_local(&self, global_point: &Vec3<f64>) -> Vec3<f64> {
        self.affine_inverse.map_vec(*global_point)
    }

}

//Bound.rs
#[derive(Debug)]
enum Bound {
    Rectangle(RectangleBound),
    Other,
}

#[derive(Debug)]
struct RectangleBound {
    // These are local half bound values from the center of the plane
    x_half_bound: f64,
    y_half_bound: f64,
}

impl RectangleBound {
    pub fn new(x_half_bound: f64, y_half_bound: f64) -> Bound{
        Bound::Rectangle(RectangleBound {x_half_bound, y_half_bound})
    }
}

// LinearAlgebra.rs

// Translations and rotations to be implemented

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


fn main() {

    let origin = Vec3::<f64>::from(0.0, 0.0, 0.0);
    let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
    let bound = Some(RectangleBound::new(5.0, 10.0));
    let p = PlaneSurface::new(origin, normal, bound);


    let m = Vec3::<f64>::from(0.0, 2.0, 4.0);

    println!("Is point m:{:#?} on surface P:{:#?} :{}",m, p.affine, p.is_point_on_plane(&m));
    println!("Is point m:{:#?} on surface P:{:#?} :{}",m, p.affine, p.is_point_in_bounds(&m));
    println!("Is point m:{:#?} on surface P:{:#?} :{}",m, p.affine, p.is_point_on_surface(&m));

    let n = Mat3::<f64>::from(2.0, 0.0, 0.0,
                        0.0, -1.0, 0.0,
                        0.0, 0.0, 3.0);

    

}
