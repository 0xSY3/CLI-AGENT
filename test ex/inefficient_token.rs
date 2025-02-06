#![cfg_attr(not(feature = "export"), no_main)]
extern crate alloc;

use stylus_sdk::{prelude::*, storage::StorageVec, storage::StorageMap};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

sol_storage! {
    #[entrypoint]
    pub struct InefficientToken {
        // Inefficient: Multiple storage accesses
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        StorageVec<address> holders;
        uint256 total_supply;
    }
}

impl InefficientToken {
    // Inefficient: Multiple storage reads in loop
    pub fn get_holder_balances(&self) -> Result<Vec<U256>, Vec<u8>> {
        let mut balances = Vec::new();
        for i in 0..self.holders.len() {
            if let Some(holder) = self.holders.get(i) {
                balances.push(self.balances.get(&holder));
            }
        }
        Ok(balances)
    }

    // Inefficient: Multiple storage reads without caching
    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let sender = msg::sender();
        
        // Multiple storage reads of same value
        if self.balances.get(&sender) < amount {
            return Ok(false);
        }
        
        // Unbounded loop with storage reads
        for i in 0..self.holders.len() {
            if self.holders.get(i) == Some(to) {
                break;
            }
        }
        
        // Multiple storage writes without caching
        self.balances.insert(&sender, self.balances.get(&sender) - amount);
        self.balances.insert(&to, self.balances.get(&to) + amount);
        
        Ok(true)
    }

    // Inefficient: Unnecessary storage writes
    pub fn approve(&mut self, spender: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let sender = msg::sender();
        // Unnecessary read before write
        let current = self.allowances.get(&sender).get(&spender);
        if current == amount {
            // Unnecessary write when value hasn't changed
            self.allowances.get_mut(&sender).insert(&spender, amount);
        }
        Ok(true)
    }
}
