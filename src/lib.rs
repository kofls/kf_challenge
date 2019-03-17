use vecmat::{vec::Vec2, vec::Vec3};
use approx::abs_diff_eq;
use std::f64;

mod geometry;
mod bound;
mod linalg;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        let a = Vec3::<f64>::from(1.0, 2.0, 4.0);
        let b = Vec3::<f64>::from(-2.0, 6.0, -1.0);
        let c = Vec3::<f64>::from(-1.0, 8.0, 3.0);

        assert_eq!(linalg::translate(a, b), c);
    }

    #[test]
    fn test_rotate_about_x_axis() {
        let a = Vec3::<f64>::from(1.0, 2.0, 4.0);
        let angle = f64::consts::PI;
        let b = linalg::rotate_about_x_axis(a, angle);
        let c = Vec3::<f64>::from(1.0, -2.0, -4.0);
        
        // tests for approximately equal since floating point
        // arithmetic is slightly imprecise
        abs_diff_eq!(b[0], c[0], epsilon = f64::EPSILON);
        abs_diff_eq!(b[1], c[1], epsilon = f64::EPSILON);
        abs_diff_eq!(b[2], c[2], epsilon = f64::EPSILON);
    }

    #[test]
    fn test_rotate_about_y_axis() {
        let a = Vec3::<f64>::from(1.0, 2.0, 4.0);
        let angle = f64::consts::PI;
        let b = linalg::rotate_about_y_axis(a, angle);
        let c = Vec3::<f64>::from(-1.0, 2.0, -4.0);
        
        abs_diff_eq!(b[0], c[0], epsilon = f64::EPSILON);
        abs_diff_eq!(b[1], c[1], epsilon = f64::EPSILON);
        abs_diff_eq!(b[2], c[2], epsilon = f64::EPSILON);
    }

    #[test]
    fn test_rotate_about_z_axis() {
        let a = Vec3::<f64>::from(1.0, 2.0, 4.0);
        let angle = f64::consts::PI;
        let b = linalg::rotate_about_z_axis(a, angle);
        let c = Vec3::<f64>::from(-1.0, -2.0, 4.0);
        
        abs_diff_eq!(b[0], c[0], epsilon = f64::EPSILON);
        abs_diff_eq!(b[1], c[1], epsilon = f64::EPSILON);
        abs_diff_eq!(b[2], c[2], epsilon = f64::EPSILON);
    }

    #[test]
    fn test_is_point_on_surface_true() {
        let origin = Vec3::<f64>::from(0.0, 0.0, 0.0);
        let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
        let bound = Some(bound::RectangleBound::new(5.0, 10.0));
        let p = geometry::PlaneSurface::new(origin, normal, bound);
        let m = Vec3::<f64>::from(0.0, 2.0, 4.0);

        assert!(p.is_point_on_surface(&m));
    }

    #[test]
    fn test_is_point_on_surface_false() {
        let origin = Vec3::<f64>::from(2.0, 6.0, 1.0);
        let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
        let bound = Some(bound::RectangleBound::new(1.0, 3.0));
        let p = geometry::PlaneSurface::new(origin, normal, bound);
        let m = Vec3::<f64>::from(0.0, 2.0, 4.0);

        assert!(!p.is_point_on_surface(&m));
    }
    
    #[test]
    fn test_global_to_local_2d() {
        let origin = Vec3::<f64>::from(2.0, 6.0, 1.0);
        let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
        let bound = Some(bound::RectangleBound::new(1.0, 3.0));
        let p = geometry::PlaneSurface::new(origin, normal, bound);

        let gpoint = Vec3::<f64>::from(2.0, 7.0, 4.0);
        let lpoint = p.global_to_local_2d(&gpoint);
        let lpoint_expected = Vec2::<f64>::from(1.0, 3.0);

        assert_eq!(lpoint, lpoint_expected);
    }

    #[test]
    fn test_local_2d_to_global() {
        let origin = Vec3::<f64>::from(2.0, 6.0, 1.0);
        let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
        let bound = Some(bound::RectangleBound::new(1.0, 3.0));
        let p = geometry::PlaneSurface::new(origin, normal, bound);

        let lpoint = Vec2::<f64>::from(1.0, 3.0);
        let gpoint_expected = Vec3::<f64>::from(2.0, 7.0, 4.0);
        let gpoint = p.local_2d_to_global(&lpoint);

        assert_eq!(gpoint, gpoint_expected);
    }

    #[test]
    fn test_global_to_local() {
        let origin = Vec3::<f64>::from(2.0, 6.0, 1.0);
        let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
        let bound = Some(bound::RectangleBound::new(1.0, 3.0));
        let p = geometry::PlaneSurface::new(origin, normal, bound);

        let gpoint = Vec3::<f64>::from(5.0, 7.0, 4.0);
        let lpoint = p.global_to_local(&gpoint);
        let lpoint_expected = Vec3::<f64>::from(3.0, 1.0, 3.0);

        assert_eq!(lpoint, lpoint_expected);
    }

    #[test]
    fn test_local_to_global() {
        let origin = Vec3::<f64>::from(2.0, 6.0, 1.0);
        let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
        let bound = Some(bound::RectangleBound::new(1.0, 3.0));
        let p = geometry::PlaneSurface::new(origin, normal, bound);

        let lpoint = Vec3::<f64>::from(3.0, 1.0, 3.0);
        let gpoint_expected = Vec3::<f64>::from(5.0, 7.0, 4.0);
        let gpoint = p.local_to_global(&lpoint);

        assert_eq!(gpoint, gpoint_expected);
    }

}