extern crate vecmat;
extern crate approx;
// https://stackoverflow.com/questions/3913832/what-is-a-3d-vector-and-how-does-it-differ-from-a-3d-point
use vecmat::{vec::Vec2, vec::Vec3, vec::Vec4, mat::Mat3, map::Affine3};

mod geometry;
mod bound;
mod linalg;

fn main() {

    let origin = Vec3::<f64>::from(0.0, 0.0, 0.0);
    let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
    let bound = Some(bound::RectangleBound::new(5.0, 10.0));
    let p = geometry::PlaneSurface::new(origin, normal, bound);

    let m = Vec3::<f64>::from(0.0, 2.0, 4.0);

    println!("Is point m:{:#?} on surface P:{:#?} :{}",m, p.affine, p.is_point_on_surface(&m));

    let n = Mat3::<f64>::from(2.0, 0.0, 0.0,
                        0.0, -1.0, 0.0,
                        0.0, 0.0, 3.0);
}
