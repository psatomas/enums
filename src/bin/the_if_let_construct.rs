enum Milk {
    LowFat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;
     
    if let Milk::NonDairy { kind}= my_beverage {
        println!("Your bevarage is {kind} milk");
    } else {
        println!("You have some other milk variant");
    }
} 