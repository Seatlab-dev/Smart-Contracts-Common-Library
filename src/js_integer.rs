use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

/// A number in which the max value should be `(2^53) - 1`.  
/// It's de/serialization is not stringfied. It de/serializes similarly to `u32`.
///
/// A value verification is made when getting or setting the number.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Debug,
    PartialOrd,
    Ord,
    Default,
    BorshDeserialize,
    BorshSerialize,
    Serialize,
    Deserialize,
)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct JsUint(u64);

impl JsUint {
    const MAX: u64 = (1u64 << 53) - 1u64;

    pub fn new(n: u64) -> Self {
        assert!(n <= Self::MAX);
        Self(n)
    }

    pub fn get(self) -> u64 {
        assert!(self.0 <= Self::MAX);
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catch<'s>(f: impl FnOnce() + std::panic::UnwindSafe) -> &'s str {
        *std::panic::catch_unwind(f)
            .err()
            .unwrap()
            .downcast::<&'static str>()
            .unwrap()
    }

    #[test]
    fn zero() {
        JsUint::new(0);
    }

    #[test]
    fn max() {
        let n = JsUint::new(JsUint::MAX);
        assert_eq!(n.get(), JsUint::MAX);

        let n = JsUint::new(9007199254740991);
        assert_eq!(n.get(), 9007199254740991);
    }

    #[test]
    fn max_1() {
        assert_eq!(
            catch(|| {
                JsUint::new(JsUint::MAX + 1);
            }),
            "assertion failed: n <= Self::MAX"
        );

        assert_eq!(
            catch(|| {
                JsUint::new(9007199254740991_u64 + 1);
            }),
            "assertion failed: n <= Self::MAX"
        );

        let mut n = JsUint(JsUint::MAX + 1);
        assert_eq!(
            catch(|| {
                n.get();
            }),
            "assertion failed: self.0 <= Self::MAX"
        );

        n.0 -= 1;
        assert_eq!(n.get(), JsUint::MAX);
    }
}
