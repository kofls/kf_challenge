// 'vecmat' library is used for the vector and matrix functions.
// This library implements vector dot/cross product and Affine transforms.
// 'approx' library is used for checking equality of approximate floating point values.
use vecmat::{vec::Vec2, vec::Vec3, mat::Mat3, map::Affine3};
use vecmat::vec::Dot;
use crate::bound::Bound;
use approx::abs_diff_eq;
use std::f64;

// A plane surface is defined by its basis and the center of the surface.
// t is the normal to the surface and u, v are orthogonal vectors along the surface.
// bounds is an optional field. If bounds is 'None', the plane surface is assumed to be boundless/infinite.
// affine and affine_inverse are initially calculated and stored for better performance.
#[derive(Debug)]
pub struct PlaneSurface {
    pub t: Vec3<f64>,
    pub u: Vec3<f64>,
    pub v: Vec3<f64>,
    pub center: Vec3<f64>,
    pub bounds: Option<Bound>,
    affine: Affine3<f64>,
    affine_inverse: Affine3<f64>,
}

impl PlaneSurface {
    // center is any point on the plane surface. It is considered as the origin for
    // the local coordinate system.
    // The t, u, v coordinate system used is from
    // https://gitlab.cern.ch/acts/acts-core/tree/master/Core/include/Acts/Surfaces/PlaneSurface.cpp
    //      "the right-handed coordinate system is defined as
    //      T = normal
    //      U = Z x T if T not parallel to Z otherwise U = X x T
    //      V = T x U"
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

    // If point is not on plane, is_point_in_bounds is not called.
    pub fn is_point_on_surface(&self, point: &Vec3<f64>) -> bool{
        self.is_point_on_plane(&point) && self.is_point_in_bounds(&point)
    }

    fn is_point_on_plane(&self, point: &Vec3<f64>) -> bool{
        let distance = *point - self.center;
        // approximate check is done since floating point operations are slightly imprecise
        abs_diff_eq!(self.t.dot(distance), 0.0, epsilon = f64::EPSILON)
    }

    fn is_point_in_bounds(&self, point: &Vec3<f64>) -> bool{
        // If plane surface is unbounded, returns true. Therefore, the 'is_point_on_plane' check must be made beforehand.
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
    
    // The local 2D coordinates are meaningful only if the point lies on the plane.
    // If not, use 'local_to_global' instead.
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
