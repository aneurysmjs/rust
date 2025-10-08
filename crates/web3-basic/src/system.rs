use std::collections::BTreeMap;

pub struct Pallet {
    block_number: u32,

    /**
     * Nonce is the amount of transactions that every user did on the blockchain.
     * <wallet, nonce>
     */
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { block_number: 0, nonce: BTreeMap::new() }
    }

    pub fn inc_block_number(&mut self) {
        // this is not safe
        // self.block_number += 1;

        // crashes if overflow
        self.block_number = self.block_number.checked_add(1).unwrap()
    }

    pub fn inc_nonce(&mut self, who: &String) {
        let nonce = self.nonce.get(who).unwrap_or(&0);

        let new_nonce = nonce + 1;

        self.nonce.insert(who.clone(), new_nonce);
    }

    pub fn get_nonce(&mut self, who: &String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_system() {
        let system = Pallet::new();

        assert_eq!(system.block_number, 0);
    }

    #[test]
    fn test_inc_block_number() {
        let mut system = Pallet::new();

        system.inc_block_number();

        assert_eq!(system.block_number, 1);
    }

    #[test]
    fn test_inc_nonce() {
        let mut system = Pallet::new();
        let user = "Alice".to_string();

        system.inc_nonce(&user);

        // assert_eq!(system.nonce.get(&user), Some(&1));
        assert_eq!(system.get_nonce(&user), 1);
    }
}
