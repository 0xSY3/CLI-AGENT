use std::collections::HashMap;

// Mock implementations for testing
pub struct StorageVec<T>(Vec<T>);
impl<T> StorageVec<T> {
    pub fn get(&self, index: usize) -> Option<T> where T: Clone {
        self.0.get(index).cloned()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

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

pub struct Token {
    balances: StorageMap<[u8; 20], u64>,
    holders: StorageVec<[u8; 20]>,
    total_supply: u64,
}

impl Token {
    pub fn transfer(&mut self, to: [u8; 20], amount: u64) -> Result<bool, Vec<u8>> {
        let sender = [0u8; 20];
        let sender_balance = self.balances.get(&sender);

        if sender_balance < amount {
            return Ok(false);
        }

        self.balances.insert(sender, sender_balance - amount);
        self.balances.insert(to, self.balances.get(&to) + amount);

        Ok(true)
    }
}

// For running the example
fn main() {
    let mut token = Token {
        balances: StorageMap::default(),
        holders: StorageVec(Vec::new()),
        total_supply: 0,
    };

    // Test basic functionality
    let recipient = [1u8; 20];
    if let Ok(success) = token.transfer(recipient, 100) {
        println!("Transfer {}", if success { "succeeded" } else { "failed" });
    }
}