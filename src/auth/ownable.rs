// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    pub struct Ownable {
        address owner;
    }
}

sol! {
    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);

    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
}

pub enum OwanbleError {
    OwnableInvalidOwner(OwnableInvalidOwner),
    OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
}


impl Ownable {
    pub fn transfer_ownership_impl(&mut self, new_owner: address) -> Result<(), Vec<u8>> {
        let previous_owner = self.owner.get();
        if previous_owner != msg::sender() {
            return Err(OwnableUnauthorizedAccount {
                account: msg::sender(),
            });
        }
        self.owner.set(new_owner);
        evm::log(OwnershipTransferred { previous_owner, new_owner });
        Ok(())
    }

    pub fn only_owner_impl(&mut self) -> Result<(), Vec<u8>> {
        let owner = self.owner.get();
        if owner != msg::sender() {
            return Err(OwnableInvalidOwner {
                account: msg::sender(),
            });
        }
        Ok(())
    }

}


#[external]
impl Ownable {   
    pub fn owner(&self) -> Result<U256, Vec<u8>> {
        Ok(self.owner.get())
    }

    pub fn transfer_ownership(&mut self, new_owner: address) -> Result<(), Vec<u8>> {
        if new_owner == Address::ZERO {
            return Err(OwnableInvalidOwner {
                owner: Address::ZERO,
            });
        }
        transfer_impl(self, new_owner)?;
        Ok(())
    }

    pub fn renounce_ownership(&mut self) -> Result<(), Vec<u8>> {
        transfer_impl(self, Address::ZERO)?;
        Ok(())
    }

    pub fn only_owner(&mut self) -> Result<(), Vec<u8>> {
        self.only_owner_impl()?;
        Ok(())
    } 
}
