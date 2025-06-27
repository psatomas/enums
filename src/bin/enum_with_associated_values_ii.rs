#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
}

fn main() {
    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("0034-5678"));

    my_payment_method =
        PaymentMethodType::PayPal(String::from("bob@email.com"), String::from("password"));

    println!("{:?}", my_payment_method);
}