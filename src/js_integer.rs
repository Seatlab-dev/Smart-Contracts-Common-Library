use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    schemars::{
        gen::SchemaGenerator,
        schema::{
            InstanceType, Metadata, NumberValidation, Schema, SchemaObject, StringValidation,
        },
        JsonSchema,
    },
    serde::{Deserialize, Serialize},
    serde_json::json,
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

impl JsonSchema for JsUint {
    fn is_referenceable() -> bool {
        true
    }
    fn schema_name() -> String {
        ("JsUint").to_owned()
    }
    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        let n_validation = NumberValidation {
            // maximum value that can be represented that is below u64::MAX
            maximum: Some(9007199254740991.),
            minimum: Some(0.0),
            ..Default::default()
        };

        let s_validation = StringValidation {
            max_length: Some(JsUint::MAX.to_string().chars().count() as u32),
            min_length: Some(0.to_string().chars().count() as u32),
            // 9007199254740991
            pattern: Some(r#"^[0-9]{1,16}$"#.into()),
        };

        let meta = Metadata {
            description: Some("Unsigned integer.".into()),
            default: Some(json!(0)),
            examples: vec![json!(0), json!(JsUint::MAX)],
            ..Default::default()
        };

        SchemaObject {
            instance_type: Some(InstanceType::Integer.into()),
            format: None,
            metadata: Box::new(meta).into(),
            number: Some(Box::new(n_validation)),
            string: Some(Box::new(s_validation)),
            ..Default::default()
        }
        .into()
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
