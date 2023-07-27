use std::convert::From;

#[derive(Debug, Copy, Clone)]
pub struct Area {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl Area {
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Area {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
    pub fn middle_x(self) -> f64 {
        (self.x_min + self.x_max) / 2.0
    }
    pub fn middle_y(self) -> f64 {
        (self.y_min + self.y_max) / 2.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub area: Area,
    pub upper_left: Option<usize>,
    pub upper_right: Option<usize>,
    pub lower_left: Option<usize>,
    pub lower_right: Option<usize>,
}

impl From<Area> for Node {
    fn from(area: Area) -> Self {
        Node {
            area,
            upper_left: None,
            upper_right: None,
            lower_left: None,
            lower_right: None,
        }
    }
}
