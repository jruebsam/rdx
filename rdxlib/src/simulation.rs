use crate::VehicleModel;

#[derive(Debug)]
pub struct Simulation<T: VehicleModel> {
    time: f64,
    dt: f64,
    vehicle: T,
}

impl<T: VehicleModel> Simulation<T> {
    pub fn new(dt: f64, vehicle: T) -> Self {
        Self {
            time: 0.0,
            dt,
            vehicle,
        }
    }

    pub fn do_step(&mut self) {
        self.time += self.dt;
        self.vehicle.do_step(self.dt);
    }
}
