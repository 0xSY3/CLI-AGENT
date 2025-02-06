#![cfg_attr(not(feature = "export"), no_main)]
extern crate alloc;

use std::collections::HashMap;

// Mock implementations for testing
pub struct StorageMap<K, V>(HashMap<K, V>);
pub struct StorageVec<T>(Vec<T>);

impl<K, V> StorageMap<K, V> {
    pub fn get(&self, key: &K) -> V where K: Eq + std::hash::Hash, V: Clone {
        self.0.get(key).cloned().unwrap_or_default()
    }

    pub fn insert(&mut self, key: K, value: V) where K: Eq + std::hash::Hash {
        self.0.insert(key, value);
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> where K: Eq + std::hash::Hash {
        self.0.get_mut(key)
    }
}

impl<T> StorageVec<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<T> where T: Clone {
        self.0.get(index).cloned()
    }
}

pub struct InefficientToken {
    balances: StorageMap<[u8; 20], u64>,
    allowances: StorageMap<[u8; 20], StorageMap<[u8; 20], u64>>,
    holders: StorageVec<[u8; 20]>,
    total_supply: u64,
}

impl InefficientToken {
    pub fn get_holder_balances(&self) -> Result<Vec<u64>, Vec<u8>> {
        let mut balances = Vec::new();
        for i in 0..self.holders.len() {
            if let Some(holder) = self.holders.get(i) {
                balances.push(self.balances.get(&holder));
            }
        }
        Ok(balances)
    }

    pub fn transfer(&mut self, to: [u8; 20], amount: u64) -> Result<bool, Vec<u8>> {
        let sender = [0u8; 20];  // Mock sender for testing

        if self.balances.get(&sender) < amount {
            return Ok(false);
        }

        let mut found = false;
        for i in 0..self.holders.len() {
            if self.holders.get(i) == Some(to) {
                found = true;
                break;
            }
        }
        if !found && amount > 0 {
            self.holders.0.push(to);
        }

        self.balances.insert(sender, self.balances.get(&sender) - amount);
        self.balances.insert(to, self.balances.get(&to) + amount);

        Ok(true)
    }

    pub fn approve(&mut self, spender: [u8; 20], amount: u64) -> Result<bool, Vec<u8>> {
        let sender = [0u8; 20]; // Mock sender for testing
        let mut allowances_sender = self.allowances.get_mut(&sender);
        if let Some(allowances) = allowances_sender {
            let current = allowances.get(&spender);
            if current != amount {
                allowances.insert(spender, amount);
            }
        } else {
            let mut new_allowances = StorageMap::new();
            new_allowances.insert(spender, amount);
            self.allowances.insert(sender, new_allowances);
        }
        Ok(true)
    }
}

impl StorageMap<_, _> {
    pub fn new() -> Self {
        StorageMap(HashMap::new())
    }
}

impl StorageVec<_> {
    pub fn new() -> Self {
        StorageVec(Vec::new())
    }
}

impl Default for StorageMap<[u8; 20], u64> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StorageVec<[u8; 20]> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StorageMap<[u8; 20], StorageMap<[u8; 20], u64>> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

// For running the example
fn main() {
    let mut token = InefficientToken {
        balances: StorageMap::new(),
        allowances: StorageMap::new(),
        holders: StorageVec::new(),
        total_supply: 0,
    };

    // Test basic functionality
    let recipient = [1u8; 20];
    if let Ok(success) = token.transfer(recipient, 100) {
        println!("Transfer {}", if success { "succeeded" } else { "failed" });
    }
}