use near_sdk::{AccountId, Gas};

pub mod contract_ext;
pub mod execution_ext;

pub use contract_ext::ContractExt;
pub use execution_ext::{pretty_near, ExecutionExt};

pub trait IntoGas {
    fn into_gas(self) -> Gas;
}

impl IntoGas for u128 {
    fn into_gas(self) -> Gas {
        assert!(self < u64::MAX as u128);
        Gas(self as u64)
    }
}

/// Creates a user with a certain length.
pub fn long_user(s: &str) -> AccountId {
    let name = long_name(s, 64);
    AccountId::new_unchecked(name)
}

/// Creates a string with a certain length.
pub fn long_name(
    s: &str,
    len: u16,
) -> String {
    let len = len as usize - s.len();
    str::repeat("o", len) + s
}

/// Creates a user given an id.  
/// Eg. `user0`.
pub fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}
