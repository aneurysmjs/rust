use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn get_balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(&mut self, from: &String, to: &String, amount: u128) -> Result<(), &str> {
        let from_balance = self.get_balance(from);
        let to_balance = self.get_balance(to);

        let checked_from_balance = from_balance.checked_sub(amount).ok_or("Insufficient balance")?;
        let checked_to_balance = to_balance.checked_add(amount).ok_or("Overflow when adding balance")?;

        self.set_balance(from, checked_from_balance);
        self.set_balance(to, checked_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balances() {
        let mut balances = Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), 200);

        assert_eq!(balances.get_balance(&"Alice".to_string()), 100);
        assert_eq!(balances.get_balance(&"Bob".to_string()), 200);
        assert_eq!(balances.get_balance(&"Charlie".to_string()), 0);
    }

    #[test]
    fn test_transfer() {
        let alice = "Alice".to_string();
        let bob = "Bob".to_string();

        let mut balances = Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), 200);

        let _ = balances.transfer(&alice, &bob, 50);

        assert_eq!(balances.get_balance(&alice), 50);
        assert_eq!(balances.get_balance(&bob), 250);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let alice = "Alice".to_string();
        let bob = "Bob".to_string();

        let mut balances = Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), 200);

        let result = balances.transfer(&alice, &bob, 500);

        assert_eq!(result, Err("Insufficient balance"));
        assert_eq!(balances.get_balance(&alice), 100);
        assert_eq!(balances.get_balance(&bob), 200);
    }

    #[test]
    fn test_transfer_overflow() {
        let alice = "Alice".to_string();
        let bob = "Bob".to_string();

        let mut balances = Pallet::new();

        balances.set_balance(&"Alice".to_string(), 100);
        balances.set_balance(&"Bob".to_string(), u128::MAX);

        let result = balances.transfer(&alice, &bob, 10);

        assert_eq!(result, Err("Overflow when adding balance"));
        assert_eq!(balances.get_balance(&alice), 100);
        assert_eq!(balances.get_balance(&bob), u128::MAX);
    }
}
