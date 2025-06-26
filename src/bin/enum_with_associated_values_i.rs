#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String),
}

fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("0034-5678"));
    let master_card = PaymentMethodType::DebitCard(String::from("2532-1295"));
    println!("{:?}", visa);
    println!("{:?}", master_card);
}