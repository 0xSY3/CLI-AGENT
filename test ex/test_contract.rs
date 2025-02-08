use stylus_sdk::{prelude::*, msg::Args};

#[stylus_sdk::contract]
pub struct Counter {
    value: StorageU64,
}

#[stylus_sdk::contractimpl]
impl Counter {
    pub fn new() -> Self {
        Self {
            value: StorageU64::new(0),
        }
    }

    pub fn increment(&mut self) {
        let current = self.value.get();
        self.value.set(current + 1);
    }

    pub fn get(&self) -> u64 {
        self.value.get()
    }
}
