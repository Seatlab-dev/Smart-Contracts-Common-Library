use near_sdk::AccountId;

/// Currently not used, but can be useful in case a functio increases (or decreases)
/// the storage requirements and must pay (or receive) for it.
pub fn refund_deposit<R>(
    deduct: impl Into<Option<u128>>,
    f: impl FnOnce() -> R,
) -> R {
    refund_deposit_to(near_sdk::env::predecessor_account_id(), deduct, f)
}

/// Currently not used, but can be useful in case a functio increases (or decreases)
/// the storage requirements and must pay (or receive) for it.
pub fn refund_deposit_to<R>(
    receiver: AccountId,
    deduct: impl Into<Option<u128>>,
    f: impl FnOnce() -> R,
) -> R {
    use near_sdk::env;

    let deduct = deduct.into().unwrap_or_default();

    let initial_storage_usage = env::storage_usage();
    let storage_byte_cost = env::storage_byte_cost();
    let attached_deposit = env::attached_deposit();

    let r = f();

    let final_storage_usage = env::storage_usage();

    if final_storage_usage >= initial_storage_usage {
        // increased storage usage
        let increased_storage = final_storage_usage - initial_storage_usage;
        let increased_cost = storage_byte_cost * near_sdk::Balance::from(increased_storage);

        let final_cost = increased_cost + deduct;

        near_sdk::require!(
            attached_deposit >= final_cost,
            format!(
                "Must attach {} yoctoNEAR more to cover storage and deductions",
                final_cost - attached_deposit
            )
        );

        let refund = attached_deposit - final_cost;
        if refund > 1 {
            near_sdk::Promise::new(receiver).transfer(refund);
        }
    } else {
        // decreased storage usage
        let decreased_storage = initial_storage_usage - final_storage_usage;
        let decreased_cost = storage_byte_cost * near_sdk::Balance::from(decreased_storage);

        let final_attached = attached_deposit + decreased_cost;

        near_sdk::require!(
            final_attached >= deduct,
            format!(
                "Must attach {} yoctoNEAR more to cover storage and deductions",
                deduct - final_attached
            )
        );
        let refund = final_attached - deduct;

        if refund > 1 {
            near_sdk::Promise::new(receiver).transfer(refund);
        }
    };

    r
}
