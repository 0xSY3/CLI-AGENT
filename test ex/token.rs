#![cfg_attr(not(feature = "export"), no_main)]
extern crate alloc;

use stylus_sdk::{prelude::*, storage::StorageVec};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

sol_storage! {
    #[entrypoint]
    pub struct Token {
        mapping(address => uint256) balances;
        StorageVec<address> holders;
        uint256 total_supply;
    }
}

impl Token {
    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let sender = msg::sender();
        let sender_balance = self.balances.get(&sender);
        
        // Multiple storage reads
        if self.balances.get(&sender) < amount {
            return Ok(false);
        }
        
        // Unbounded loop
        for i in 0..self.holders.len() {
            if self.holders.get(i) == Some(sender) {
                break;
            }
        }
        
        // Multiple storage writes
        self.balances.insert(&sender, sender_balance - amount);
        self.balances.insert(&to, self.balances.get(&to) + amount);
        
        Ok(true)
    }
}
