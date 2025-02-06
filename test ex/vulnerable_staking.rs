#![cfg_attr(not(feature = "export"), no_main)]
use std::collections::HashMap;

// Mock implementations for testing
pub struct StorageMap<K, V>(HashMap<K, V>);

impl<K, V> StorageMap<K, V> {
    pub fn get(&self, key: &K) -> V where K: Eq + std::hash::Hash, V: Clone + Default {
        self.0.get(key).cloned().unwrap_or_default()
    }

    pub fn insert(&mut self, key: K, value: V) where K: Eq + std::hash::Hash {
        self.0.insert(key, value);
    }
}

impl<K, V> Default for StorageMap<K, V> {
    fn default() -> Self {
        StorageMap(HashMap::new())
    }
}

pub struct VulnerableStaking {
    stakes: StorageMap<[u8; 20], u64>,
    rewards: StorageMap<[u8; 20], u64>,
    total_staked: u64,
    owner: [u8; 20],
}

impl VulnerableStaking {
    pub fn withdraw(&mut self) -> Result<bool, Vec<u8>> {
        let user = [0u8; 20];  // Mock user for testing
        let stake = self.stakes.get(&user);
        let reward = self.rewards.get(&user);

        if stake > 0 {
            // Vulnerability: State changes after external call
            // In real implementation, this would be an external call
            // msg::send(user, stake + reward)?;
            self.stakes.insert(user, 0);
            self.rewards.insert(user, 0);
        }

        Ok(true)
    }

    pub fn stake(&mut self) -> Result<bool, Vec<u8>> {
        let amount = 100;  // Mock value for testing
        let user = [0u8; 20];

        let new_stake = self.stakes.get(&user) + amount;
        self.stakes.insert(user, new_stake);

        self.total_staked = self.total_staked + amount;

        Ok(true)
    }

    pub fn set_rewards(&mut self, user: [u8; 20], amount: u64) -> Result<bool, Vec<u8>> {
        self.rewards.insert(user, amount);
        Ok(true)
    }
}

// For running the example
fn main() {
    let mut staking = VulnerableStaking {
        stakes: StorageMap::default(),
        rewards: StorageMap::default(),
        total_staked: 0,
        owner: [0u8; 20],
    };

    // Test basic functionality
    if let Ok(success) = staking.stake() {
        println!("Stake {}", if success { "succeeded" } else { "failed" });
    }
}