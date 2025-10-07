mod balances;

fn main() {
    let mut balances = balances::Pallet::new();

    balances.set_balance(&"Alice".to_string(), 100);

    let alice_balance = balances.get_balance(&"Alice".to_string());

    balances.transfer(&"Alice".to_string(), &"Bob".to_string(), 50).unwrap();

    println!("Alice's balance is {}", alice_balance);
}
