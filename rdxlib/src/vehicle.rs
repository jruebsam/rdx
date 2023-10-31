use crate::common::*;

#[derive(Debug, Default)]
pub struct Vehicle {
    pub pose: Pose,
}

impl Vehicle {
    pub fn new() -> Self {
        Default::default()
    }
}
