#![cfg_attr(not(feature = "export"), no_main)]
extern crate alloc;

use stylus_sdk::{prelude::*, storage::StorageMap};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

sol_storage! {
    #[entrypoint]
    pub struct VulnerableStaking {
        // Security issue: Public state variables without proper access control
        mapping(address => uint256) stakes;
        mapping(address => uint256) rewards;
        uint256 total_staked;
        address owner;
    }
}

impl VulnerableStaking {
    // Vulnerability: No reentrancy protection
    pub fn withdraw(&mut self) -> Result<bool, Vec<u8>> {
        let user = msg::sender();
        let stake = self.stakes.get(&user);
        let reward = self.rewards.get(&user);
        
        // Vulnerability: State changes after external call
        if stake > U256::zero() {
            // Potential reentrancy vulnerability
            msg::send(user, stake + reward)?;
            self.stakes.insert(&user, U256::zero());
            self.rewards.insert(&user, U256::zero());
        }
        
        Ok(true)
    }

    // Vulnerability: Integer overflow possible
    pub fn stake(&mut self) -> Result<bool, Vec<u8>> {
        let amount = msg::value();
        let user = msg::sender();
        
        // Vulnerability: No overflow check
        let new_stake = self.stakes.get(&user) + amount;
        self.stakes.insert(&user, new_stake);
        
        // Vulnerability: No overflow check
        self.total_staked = self.total_staked + amount;
        
        Ok(true)
    }

    // Vulnerability: No access control
    pub fn set_rewards(&mut self, user: Address, amount: U256) -> Result<bool, Vec<u8>> {
        // Missing owner check
        self.rewards.insert(&user, amount);
        Ok(true)
    }
}
