use crate::shapes::point::Point;

#[derive(Debug)]
pub struct AvoidanceConfig {
    /// How far ahead a trail is allowed to see
    pub scan_distance: f64,

    /// How far to to the left/right a trail is allowed to scan
    pub scan_angle: f64,

    /// Size of the canvas
    pub size: f64,
}

impl Default for AvoidanceConfig {
    fn default() -> Self {
        AvoidanceConfig {
            scan_distance: 20.0,
            scan_angle: 0.5,
            size: 1000.0,
        }
    }
}

#[derive(Debug)]
pub struct Trail {
    /// Size of the trail
    pub radius: f64,

    /// Position of the head
    pub position: Point,

    /// orientation of the head
    pub direction: f64,
}

impl Trail {
    pub fn new(radius: f64, position: Point, direction: f64) -> Self {
        Trail {
            radius,
            position,
            direction,
        }
    }
}
