#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}
#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}
#[derive(Debug)]
enum RestaurantItem {
    Burrito{meat: Meat, beans: Beans},
    bowl{meat: Meat, beans: Beans},
    vaganPlate,
}

fn main() {
    let lunch =RestaurantItem::Burrito{
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let dinner = RestaurantItem::bowl{
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let abandoned_meal = RestaurantItem::vaganPlate;
    println!("Lunch was {lunch:?} and dinner was {dinner:?}.");
    println!("nobody ate {abandoned_meal:?}.");
} 