extern crate vecmat;
extern crate approx;
// https://stackoverflow.com/questions/3913832/what-is-a-3d-vector-and-how-does-it-differ-from-a-3d-point
use vecmat::{vec::Vec2, vec::Vec3, vec::Vec4, mat::Mat3, map::Affine3};

mod geometry;
mod bound;
mod linalg;

fn main() {

    let a = Vec3::<f64>::from(1.0, 2.0, 4.0);
    let b = Vec3::<f64>::from(-2.0, 6.0, -1.0);

    println!("Vector a is {:?}", a);
    println!("Vector b is {:?}", b);
    println!("Vector a translated by b is {:?}\n\n", linalg::translate(a, b));

    let a = Vec3::<f64>::from(1.0, 2.0, 4.0);
    let angle = std::f64::consts::PI;
    let b = linalg::rotate_about_x_axis(a, angle);

    println!("Vector a is {:?}", a);
    println!("Vector a rotated by Pi radians about x-axis is {:?}\n\n", linalg::rotate_about_y_axis(a, angle));

    let origin = Vec3::<f64>::from(2.0, 6.0, 1.0);
    let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
    let bound = Some(bound::RectangleBound::new(5.0, 10.0));
    let p = geometry::PlaneSurface::new(origin, normal, bound);
    let m = Vec3::<f64>::from(0.0, 2.0, 4.0);

    println!("Point m is {:?}", m);
    println!("Surface p is {:#?}", p);
    println!("Is point m on surface p: {}\n", p.is_point_on_surface(&m));

    let gpoint = Vec3::<f64>::from(2.0, 7.0, 4.0);
    let lpoint = p.global_to_local_2d(&gpoint);
    println!("Global point g is {:?}", gpoint);
    println!("In local coordinates of surface p, the same point is {:?}", lpoint);

    let gpoint_ = p.local_2d_to_global(&lpoint);
    println!("This point converted to global coordinates again is {:?}", gpoint_);

    let n = Mat3::<f64>::from(2.0, 0.0, 0.0,
                        0.0, -1.0, 0.0,
                        0.0, 0.0, 3.0);
}
