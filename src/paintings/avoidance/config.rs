use std::f64::consts::PI;

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
            size: 1500.0,
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

    pub fn move_candidates(&self, scan_distance: &f64, scan_angle: &f64) -> Vec<(Point, f64)> {
        let mut points: Vec<(Point, f64)> = vec![];

        let mut angle = self.direction - scan_angle;

        while angle <= self.direction + scan_angle {
            angle += scan_angle / 5.0;

            let point = Point(
                self.position.0 + angle.cos() * PI * scan_distance,
                self.position.1 + angle.sin() * PI * scan_distance,
            );

            points.push((point, angle));
        }

        points
    }
}
