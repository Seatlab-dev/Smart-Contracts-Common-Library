use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    serde_json,
};

/// Wrapper that implements borsh de/serialization for [`serde_json::Value`].
///
/// For borsh, the structure is considered a `Vec<u8>`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct Value {
    inner: serde_json::Value,
}

impl BorshSerialize for Value {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::io::Result<()> {
        let bytes = serde_json::to_vec(&self.inner)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        borsh::BorshSerialize::serialize(&bytes, writer)?;
        Ok(())
    }
}

impl BorshDeserialize for Value {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let bytes: Vec<u8> = borsh::BorshDeserialize::deserialize(buf)?;
        Ok(Self {
            inner: serde_json::from_slice(&bytes)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?,
        })
    }
}

impl From<serde_json::Value> for Value {
    fn from(v: serde_json::Value) -> Self {
        Self { inner: v }
    }
}

impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        value.inner
    }
}
