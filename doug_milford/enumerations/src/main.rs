enum Payment {
    Cash(f32),
    CreditCard(String, f32),
    DebitCard(DebitData),
    Crypto{account_id: String, amount: f32},
}

struct DebitData {
    pub card_number: String,
    pub amount: f32,
}

fn main() {
    println!("Hello, world!");

    let some_payment = Payment::Cash(100.);
    process_payment(some_payment);

    let cc_payment = Payment::CreditCard("ccnum".to_string(),123.);
    process_payment(cc_payment);

    let debit_payment = Payment::DebitCard(DebitData {
        card_number: "dnum".to_string(),
        amount: 400.,
    });
    process_payment(debit_payment);

    let crypto_payment = Payment::Crypto{account_id: "a123".to_string(), amount: 5.};
    process_payment(crypto_payment);

    

    }

fn process_payment(some_payment: Payment) {
    match some_payment {
        Payment::Cash(amt) => {
            println!("Paying with cash: {}", amt);
        }
        Payment::CreditCard(some_string, some_f32) => {
            println!("Using credit card s {} f {}", some_string, some_f32);
        }
        Payment::DebitCard(data) => {
            println!("Using debit card {} {}", data.card_number, data.amount);
        }
        _ => (),
    }

}
