use num::traits::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + Copy + AddAssign;
    type Nonce: Ord + Zero + One + Clone + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,

    /**
     * Nonce is the amount of transactions that every user did on the blockchain.
     * <wallet, nonce>
     */
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
    }

    pub fn get_block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // this is not safe
        // self.block_number += 1;

        // crashes if overflow
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());

        let new_nonce = nonce + T::Nonce::one();

        self.nonce.insert(who.clone(), new_nonce);
    }

    pub fn get_nonce(&mut self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let system: super::Pallet<TestConfig> = Pallet::new();

        assert_eq!(system.block_number, 0);
    }

    #[test]
    fn test_inc_block_number() {
        let mut system: super::Pallet<TestConfig> = Pallet::new();

        system.inc_block_number();

        assert_eq!(system.block_number, 1);
    }

    #[test]
    fn test_inc_nonce() {
        let mut system: super::Pallet<TestConfig> = Pallet::new();
        let user = "Alice".to_string();

        system.inc_nonce(&user);

        // assert_eq!(system.nonce.get(&user), Some(&1));
        assert_eq!(system.get_nonce(&user), 1);
    }
}
