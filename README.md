# Kalman filter challenge

## Dependencies

    rust >= 1.33.0

### crates used:

    vecmat >= 0.4
    approx >= 0.3

## Running the code

### Get the code:

    git clone <url>
    cd kf_challenge/
    cargo build

### To run the code:

    cargo run

### To run the tests:

    cargo test

## Usage

### Points are represented using vectors like:

    let a = Vec2::<f64>::from(1.0, 4.0);
    let b = Vec3::<f64>::from(-2.0, 6.0, -1.0);

### Vectors can be translated or rotated like:

    let c = Vec3::<f64>::from(7.1, -1.5, 3.0);
    let c_translated = linalg::translate(c, b);

    // angle is defined in radians
    let angle = 2.0

    let c_x_rotated = linalg::rotate_about_x_axis(c, angle);
    let c_y_rotated = linalg::rotate_about_y_axis(c, angle);
    let c_z_rotated = linalg::rotate_about_z_axis(c, angle);

### Affine transform can be created like:

    // Define rotation matrix
    let rot_x = Mat3::<f64>::from(1.0, 0.0, 0.0,
                                0.0, angle.cos(), -1.0*angle.sin(),
                                0.0, angle.sin(), angle.cos());

    // Define translation vector
    let translate_v = Vec3::<f64>::from(6.5, 3.9, 4.0);
    let transform = Affine3::from(rotx, origin);

    let c_transformed = transform.map_vec(c);

### Bounds for a surface can be created by:

    let bound = bound::RectangleBound::new(3.5, 7.0);

### Plane surfaces can be created by:

    let origin = Vec3::<f64>::from(2.0, 6.0, 1.0);
    let normal = Vec3::<f64>::from(1.0, 0.0, 0.0);
    let bound = Some(bound::RectangleBound::new(5.0, 10.0));
    let plane = geometry::PlaneSurface::new(origin, normal, bound);

In the context of a plane surface, points can be converted from local coordinates to global coordinates and vice-versa. These points can be defined as 2D points on the plane or 3D points in the basis of the plane surface:

In 2D local coordinates:

    let gpoint = Vec3::<f64>::from(4.0, 7.0, 4.0);
    let lpoint = plane.global_to_local_2d(&gpoint);

    let gpoint_ = p.local_2d_to_global(&lpoint);

In 3D local coordinates:

    let gpoint = Vec3::<f64>::from(4.0, 7.0, 4.0);
    let lpoint = p.global_to_local(&gpoint);

    let gpoint_ = p.local_to_global(&lpoint);
