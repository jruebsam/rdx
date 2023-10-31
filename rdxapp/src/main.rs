use rdxlib;

fn main() {
    let veh = rdxlib::SimpleVehicle::new();
    let mut sim = rdxlib::Simulation::new(0.01, veh);

    for _ in 1..100 {
        sim.do_step();
        print!("\r\n");
        print!("\r\n{:#?}", sim);
    }
}
