#[derive(Debug)]
pub enum Bound {
    Rectangle(RectangleBound),
    Other, // Can be extended to use other types of bounds
}

#[derive(Debug)]
pub struct RectangleBound {
    // These are local half bound values from the center of the plane
    pub x_half_bound: f64,
    pub y_half_bound: f64,
}

impl RectangleBound {
    pub fn new(x_half_bound: f64, y_half_bound: f64) -> Bound{
        Bound::Rectangle(RectangleBound {x_half_bound, y_half_bound})
    }
}