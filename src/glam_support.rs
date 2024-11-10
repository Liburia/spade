use glam::{DVec2, Vec2};

use crate::{HasPosition, Point2};

impl HasPosition for Vec2 {
    type Scalar = f32;

    fn position(&self) -> Point2<Self::Scalar> {
        Point2 {
            x: self.x,
            y: self.y,
        }
    }
}
impl From<Vec2> for Point2<f32> {
    fn from(value: Vec2) -> Self {
        Point2 {
            x: value.x,
            y: value.y,
        }
    }
}

impl HasPosition for DVec2 {
    type Scalar = f64;

    fn position(&self) -> Point2<Self::Scalar> {
        Point2 {
            x: self.x,
            y: self.y,
        }
    }
}
impl From<DVec2> for Point2<f64> {
    fn from(value: DVec2) -> Self {
        Point2 {
            x: value.x,
            y: value.y,
        }
    }
}
