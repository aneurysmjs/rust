use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self, who: String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn get_balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balances() {
        let mut balances = Pallet::new();

        balances.set_balance("Alice".into(), 100);
        balances.set_balance("Bob".into(), 200);

        assert_eq!(balances.get_balance(&"Alice".into()), 100);
        assert_eq!(balances.get_balance(&"Bob".into()), 200);
        assert_eq!(balances.get_balance(&"Charlie".into()), 0);
    }
}
