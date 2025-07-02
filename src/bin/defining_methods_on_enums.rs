enum Laundrycycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String)
}

impl Laundrycycle {
    fn wash_laundry(&self) {
        match self {
            Laundrycycle::Cold => {
                println!("Running the laundry with cold temperature");
            },
            Laundrycycle::Hot { temperature } => {
                println!("Running the laundry with temperature of {temperature}");
            },
            Laundrycycle::Delicate(fabric_type) => {
                println!("Running the laundry with delicate cycle for {fabric_type}");
            } 
        }
    }
}

fn main() {
    Laundrycycle::Cold.wash_laundry();
    let hot_cycle = Laundrycycle::Hot { temperature: 100 };
    hot_cycle.wash_laundry();

    let delicate_cycle = Laundrycycle::Delicate(String::from("Silk"));
    delicate_cycle.wash_laundry();
}