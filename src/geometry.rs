use vecmat::{vec::Vec2, vec::Vec3, vec::Vec4, mat::Mat3, map::Affine3};
use vecmat::*;
use vecmat::vec::Dot;
use crate::bound::Bound;
use approx::abs_diff_eq;
use std::f64;


#[derive(Debug)]
pub struct PlaneSurface {
    pub u: Vec3<f64>,
    pub v: Vec3<f64>,
    pub t: Vec3<f64>,
    pub center: Vec3<f64>,
    pub bounds: Option<Bound>,
    pub affine: Affine3<f64>,
    pub affine_inverse: Affine3<f64>,
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
        
        let basis = Mat3::from(t[0], u[0], v[0],
                                t[1], u[1], v[1],
                                t[2], u[2], v[2]);

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
