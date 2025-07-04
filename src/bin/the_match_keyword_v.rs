enum Milk {
    LowFat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::LowFat(2) => {
                println!("delicious, 2% milk is my favorite!");
            }
            Milk::LowFat(percent) => {
                println!("you've got the lowfat {percent} percent version!");
            }
            Milk::Whole => {
                println!("You've got whole milk!");
            }
        }
    }
}
fn main() {
    Milk::LowFat(1).drink();
    Milk::LowFat(2).drink();
    Milk::Whole.drink();
}