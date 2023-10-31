use crate::common::*;

pub trait VehicleModel {
    fn do_step(&mut self, dt: f64);
}

#[derive(Debug, Default)]
pub struct SimpleVehicle {
    pub pose: Pose,
}

impl SimpleVehicle {
    pub fn new() -> Self {
        Default::default()
    }
}

impl VehicleModel for SimpleVehicle {
    fn do_step(&mut self, dt: f64) {
        self.pose.position.x += dt;
        self.pose.position.y += dt;
    }
}
