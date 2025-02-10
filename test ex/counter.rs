use stylus_sdk::{prelude::*, msg::Args};

#[stylus_sdk::contract]
pub struct Counter {
    value: StorageU64,
    owner: StorageAddress,
}

#[stylus_sdk::contractimpl]
impl Counter {
    pub fn new() -> Self {
        Self {
            value: StorageU64::new(0),
            owner: StorageAddress::new(msg::sender()),
        }
    }

    pub fn increment(&mut self) {
        require(msg::sender() == self.owner.get(), "Only owner can increment");
        let current = self.value.get();
        self.value.set(current + 1);
    }

    pub fn get(&self) -> u64 {
        self.value.get()
    }

    pub fn transfer_ownership(&mut self, new_owner: Address) {
        require(msg::sender() == self.owner.get(), "Only owner can transfer ownership");
        self.owner.set(new_owner);
    }
}
