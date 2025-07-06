enum Milk {
    LowFat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::NonDairy {
        kind: String::from("Oat"),
    };

    let Milk::NonDairy{ kind} = my_beverage else {
        println!("You do not have the nondairy milk");
        return;
    };

    println!("{kind} milk is available here");
} 