use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
};
use serde_with::DisplayFromStr;

// /// Wrapper that implements borsh de/serialization for [`url::Url`].
// ///
// /// For borsh, the structure is considered a `String`.
// #[serde_as]
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(crate = "near_sdk::serde")]
// #[serde(transparent)]
// // #[schemars(crate = "near_sdk::schemars")]
// pub struct Url {
//     #[serde_as(as = "DisplayFromStr")]
//     inner: url::Url,
// }

/// Wrapper that implements borsh de/serialization for [`url::Url`].
///
/// For borsh, the structure is considered a `String`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
#[schemars(crate = "near_sdk::schemars")]
pub struct Url {
    // #[serde(with = "serde_with::As::<DisplayFromStr>")]
    #[serde(deserialize_with = "serde_with::As::<DisplayFromStr>::deserialize")]
    #[serde(serialize_with = "serde_with::As::<DisplayFromStr>::serialize")]
    inner: url::Url,
}

impl BorshSerialize for Url {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::io::Result<()> {
        borsh::BorshSerialize::serialize(&self.inner.as_str(), writer)?;
        Ok(())
    }
}

impl BorshDeserialize for Url {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let s: String = borsh::BorshDeserialize::deserialize(buf)?;
        Ok(Self {
            inner: s
                .parse()
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?,
        })
    }
}

impl From<url::Url> for Url {
    fn from(u: url::Url) -> Self {
        Self { inner: u }
    }
}

impl From<Url> for url::Url {
    fn from(u: Url) -> Self {
        u.inner
    }
}
