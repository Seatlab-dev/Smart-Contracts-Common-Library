use crate::usn::UsnAmount;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
    AccountId, Balance,
};
use std::collections::HashMap;

#[cfg_attr(not(target_arch = "wasm32"), derive(Copy, Debug))]
#[derive(
    Clone,
    PartialOrd,
    PartialEq,
    Serialize,
    Deserialize,
    BorshSerialize,
    BorshDeserialize,
    JsonSchema,
)]
#[serde(crate = "near_sdk::serde")]
#[schemars(crate = "near_sdk::schemars")]
#[serde(transparent)]
#[serde(deny_unknown_fields)]
pub struct RoyaltyPercentage(pub u16);

impl RoyaltyPercentage {
    pub const MAX: Self = Self(10_000);

    pub fn new(percentage: u16) -> Self {
        let this = Self(percentage);
        this
    }

    /// Checks if the percentage is not above 100%.
    pub fn check(&self) {
        near_sdk::require!(
            *self <= Self::MAX,
            &format!("percentage {} is too high", self.0)
        );
    }

    /// Convert the royalty percentage and amount to pay into a payout (u128).
    pub fn royalty_to_payout(
        &self,
        amount_to_pay: Balance,
    ) -> Balance {
        (self.0 as u128 * amount_to_pay) / (Self::MAX.0 as u128)
    }

    pub fn royalty_to_payout_f64(
        &self,
        amount_to_pay: f64,
    ) -> f64 {
        (self.0 as f64 * amount_to_pay) / (Self::MAX.0 as f64)
    }
}

impl From<u16> for RoyaltyPercentage {
    fn from(rp: u16) -> Self {
        Self::new(rp)
    }
}

impl From<RoyaltyPercentage> for u16 {
    fn from(rp: RoyaltyPercentage) -> Self {
        rp.0
    }
}

impl std::fmt::Display for RoyaltyPercentage {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add<Self> for RoyaltyPercentage {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}

impl std::ops::AddAssign<Self> for RoyaltyPercentage {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        self.0.add_assign(rhs.0)
    }
}

impl std::ops::SubAssign<Self> for RoyaltyPercentage {
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        self.0.sub_assign(rhs.0)
    }
}

impl std::ops::Sub<Self> for RoyaltyPercentage {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}

impl std::ops::Div<Self> for RoyaltyPercentage {
    type Output = Self;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        self.0.div(rhs.0).into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, UsnAmount>,
}

impl Default for Payout {
    fn default() -> Self {
        Self {
            payout: Default::default(),
        }
    }
}
