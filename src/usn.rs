use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    require,
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
    Balance,
};
use std::{num::ParseFloatError, str::FromStr};

pub const USN_DEFAULT_DECIMALS: u8 = 18;

#[cfg_attr(not(target_arch = "wasm32"), derive(Copy))]
#[derive(
    Debug,
    Clone,
    PartialOrd,
    PartialEq,
    Serialize,
    Deserialize,
    BorshSerialize,
    BorshDeserialize,
    JsonSchema,
)]
#[schemars(crate = "near_sdk::schemars")]
#[serde(crate = "near_sdk::serde")]
#[serde(deny_unknown_fields)]
pub struct UsnAmount {
    pub unscaled_value: f64,
    pub decimals: u8,
    pub scaling_factor: u128,
}

impl Default for UsnAmount {
    fn default() -> Self {
        UsnAmount::new(0., None)
    }
}

impl UsnAmount {
    pub fn new(
        value: f64,
        decimals: Option<u8>,
    ) -> Self {
        require!(value >= 0., "usn value must be positive");
        let decimals = decimals.unwrap_or(USN_DEFAULT_DECIMALS);
        let this = Self {
            unscaled_value: value,
            decimals,
            scaling_factor: u128::pow(10, decimals as u32),
        };
        this
    }

    pub fn to_scaled(&self) -> Balance {
        ((self.unscaled_value * self.scaling_factor as f64).round() as u128).into()
    }
}

impl FromStr for UsnAmount {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount = s.parse::<f64>()?;
        Ok(UsnAmount::new(amount, None))
    }
}

impl From<f64> for UsnAmount {
    fn from(value: f64) -> Self {
        Self::new(value, None)
    }
}

impl From<UsnAmount> for f64 {
    fn from(usn: UsnAmount) -> Self {
        usn.unscaled_value
    }
}

impl From<u128> for UsnAmount {
    fn from(value: u128) -> Self {
        let mut usn = UsnAmount::new(0., None);
        if value.gt(&0) {
            let unscaled_value = (value as f64) / (usn.scaling_factor as f64);
            usn.unscaled_value = unscaled_value;
        }
        usn
    }
}

impl std::fmt::Display for UsnAmount {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}-{}", self.unscaled_value, self.to_scaled())
    }
}

impl std::ops::Add<Self> for UsnAmount {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        self.unscaled_value.add(rhs.unscaled_value).into()
    }
}

impl std::ops::AddAssign<Self> for UsnAmount {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        self.unscaled_value.add_assign(rhs.unscaled_value)
    }
}

impl std::ops::SubAssign<Self> for UsnAmount {
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        self.unscaled_value.sub_assign(rhs.unscaled_value)
    }
}

impl std::ops::Sub<Self> for UsnAmount {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        self.unscaled_value.sub(rhs.unscaled_value).into()
    }
}

impl std::ops::Div<Self> for UsnAmount {
    type Output = Self;

    fn div(
        self,
        rhs: Self,
    ) -> Self::Output {
        self.unscaled_value.div(rhs.unscaled_value).into()
    }
}
