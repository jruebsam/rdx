#[derive(Debug, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct Orientation {
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
}

impl Orientation {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct Pose {
    pub position: Position,
    pub orientation: Orientation,
}

impl Pose {
    pub fn new() -> Self {
        Default::default()
    }
}
