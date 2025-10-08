mod balances;
mod system;

#[derive(Debug)]
struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
}

impl Runtime {
    pub fn new() -> Self {
        Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
    }
}

fn main() {
    let alice = "Alice".to_string();
    let bob = "Bob".to_string();
    let charlie = "Charlie".to_string();

    let mut runtime = Runtime::new();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();

    runtime.system.inc_nonce(&alice);

    runtime.balances.transfer(&alice, &bob, 30).map_err(|err| println!("Transfer error: {:?}", err)).ok();

    runtime.system.inc_nonce(&alice);

    runtime.balances.transfer(&alice, &charlie, 50).map_err(|err| println!("Transfer error: {:?}", err)).ok();

    println!("runtime is {:#?}", runtime);
}
