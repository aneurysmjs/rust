mod balances;

fn main() {
    let mut balances = balances::Pallet::new();

    balances.set_balance("Alice".into(), 100);

    let alice_balance = balances.get_balance(&"Alice".into());

    println!("Alice's balance is {}", alice_balance);
}
