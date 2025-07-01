use std::iter::Cycle;

enum Laundrycycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String)
}

fn main() {
    wash_laundry(Laundrycycle::Cold);
    wash_laundry(Laundrycycle::Hot { temperature: 100 });
    wash_laundry(Laundrycycle::Delicate(String::from("Silk")));
}

fn wash_laundry(cycle: Laundrycycle) {
    match cycle {
        Laundrycycle::Cold => {
            println!("Running the laudry with cold temperature");
        },
        Laundrycycle::Hot { temperature } => {
            println!("Running the laudry with atemperature of {temperature}");
        },
        Laundrycycle::Delicate(fabric_type) => {
            println!("Running the laudry with delicate cycle for {fabric_type}");
        }
    }
}