
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    #[ink(storage)]
    pub struct Incrementer {
        value: i32,
        my_value: ink_storage::collections::HashMap<AccountId, u64>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                my_value: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
                my_value: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        #[ink(message)]
        pub fn get_mine(&self) -> u64 {
            self.my_value_or_zero(&self.env().caller())
        }

        #[ink(message)]
        pub fn inc_mine(&mut self, by: u64) {
            // ACTION: Get the `caller` of this function.
            // ACTION: Get `my_value` that belongs to `caller` by using `my_value_or_zero`.
            // ACTION: Insert the incremented `value` back into the mapping.
        }

        fn my_value_or_zero(&self, of: &AccountId) -> u64 {
            *self.my_value.get(of).unwrap_or(&0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }

        #[ink::test]
        fn my_value_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(10);
            assert_eq!(contract.get_mine(), 15);
        }
    }
}