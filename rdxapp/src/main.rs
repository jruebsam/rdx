use rdxlib;

fn main() {
    let veh = rdxlib::Vehicle {
        pose: rdxlib::Pose::new(),
    };
    println!("Vehicle: {:?}", veh);
}
