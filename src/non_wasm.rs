use near_sdk::AccountId;

/// Enables the creation of the "MyContractContract" wrapper given an `account_id`.  
/// This may be necessary for tests.
///
/// The usual implementation on the contract code is as follows:
///
/// ```ignore
/// #[cfg(not(target_arch = "wasm32"))]
/// impl common::non_wasm::WithAccount for MyContractContract {
///     fn with_account(account_id: AccountId) -> Self {
///         Self { account_id }
///     }
/// }
/// ```
pub trait WithAccount {
    fn with_account(account_id: AccountId) -> Self;
}
