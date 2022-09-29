use crate::consts::COUNT_SEPARATOR;
use crate::royalty::RoyaltyPercentage;
use crate::usn::UsnAmount;
use crate::{JsUint, Url, Value};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::{Base64VecU8, U128},
    schemars::{
        gen::SchemaGenerator,
        schema::{InstanceType, Metadata, Schema, SchemaObject},
        JsonSchema,
    },
    serde::{Deserialize, Serialize},
    AccountId,
};
use std::collections::HashMap;

// ====================================================================== //
/// Information about a token group.
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug, PartialEq))]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
#[serde(deny_unknown_fields)]
#[schemars(crate = "near_sdk::schemars")]
pub struct TokenOffer {
    /// The group name on which this type of token is part of.
    pub token_group_id: TokenIdGroupName,

    /// The metadata setting for this type of token.
    pub metadata: TokenMetadata,

    /// The royalty percentage setting for this type of token.
    ///
    /// See
    /// [NEP-199](https://nomicon.io/Standards/Tokens/NonFungibleToken/Payout)
    /// for more info.
    pub royalty: HashMap<AccountId, RoyaltyPercentage>,

    /// How many tokens were created under this group.
    pub units_created: JsUint,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
#[serde(deny_unknown_fields)]
#[schemars(crate = "near_sdk::schemars")]
#[schemars(example = "token_metadata_example")]
pub struct TokenMetadata {
    /// The name of this specific token.
    ///
    /// Eg. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub title: Option<String>,

    /// A longer description of the token.
    pub description: Option<String>,

    /// URL to associated media.
    ///
    /// Preferably to decentralized, content-addressed storage.
    pub media: Option<String>,

    /// Base64-encoded sha256 hash of content referenced by the `media` field.
    ///
    /// This is to guard against off-chain tampering.
    ///
    /// Required if `media` is included.
    pub media_hash: Option<Base64VecU8>,

    /// The number of tokens with this set of metadata or `media` known to exist at
    /// time of minting.
    pub copies: Option<JsUint>,

    /// Unix epoch in milliseconds when token was issued or minted (an unsigned
    /// 32-bit integer would suffice until the year 2106).
    pub issued_at: Option<String>,

    /// Unix epoch in milliseconds when token expires.
    pub expires_at: Option<String>,

    /// Unix epoch in milliseconds when token starts being valid.
    pub starts_at: Option<String>,

    /// Unix epoch in milliseconds when token was last updated.
    pub updated_at: Option<String>,

    /// Anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub extra: Option<TokenExtra>,

    /// URL to an off-chain JSON file with more info.
    pub reference: Option<String>,

    /// Base64-encoded sha256 hash of JSON from `reference` field. Required if reference is included.
    pub reference_hash: Option<Base64VecU8>,
}

pub fn token_metadata_example() -> TokenMetadata {
    TokenMetadata {
        title: Some("Arch Nemesis: Mail Carrier".into()),
        description: Some("My free-form description".into()),
        media: Some("https://example.com/token/media.xyz".into()),
        // from the hex:
        // 00112233445566778899AABBCCDDEEFF0112233445566778899AABBCCDDEEFF0
        media_hash: Some(
            b"ABEiM0RVZneImaq7zN3u/wESIzRFVmd4iZqrvM3e7/A="
                .to_vec()
                .into(),
        ),
        copies: Some(JsUint::new(1)),
        issued_at: Some("1640995200000".into()),
        expires_at: Some("1640995200000".into()),
        starts_at: Some("1640995200000".into()),
        updated_at: Some("1640995200000".into()),
        extra: Some(token_extra_example()),
        reference: Some("https://example.com/token.json".into()),
        // from the hex:
        // 00112233445566778899AABBCCDDEEFF0112233445566778899AABBCCDDEEFF0
        reference_hash: Some(
            b"ABEiM0RVZneImaq7zN3u/wESIzRFVmd4iZqrvM3e7/A="
                .to_vec()
                .into(),
        ),
    }
}

/// Collectible minting info.
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug, PartialEq))]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
#[serde(deny_unknown_fields)]
#[schemars(crate = "near_sdk::schemars")]
pub struct MintedCollectibleInfo {
    /// The index of the first minted token.
    ///
    /// For example, for a group that had 10 tokens already minted, if
    /// 5 new tokens get minted, then this should be 11.
    pub starting_index: JsUint,

    /// How many tokens for minted.
    ///
    /// For example, for a group that had 10 tokens already minted, if
    /// 5 new tokens get minted, then this should be 5.
    pub minted_amount: u16,

    // the ids of the minted tokens
    pub token_ids: Vec<String>,
}

/// Custom extra information that tokens can have.
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
#[schemars(crate = "near_sdk::schemars")]
#[schemars(example = "token_extra_example")]
pub struct TokenExtra {
    /// Token price (in USN).
    pub price: Option<U128>,

    /// Audio URL.
    ///
    /// Eg. `https://audio.com/audio.mp3`.
    pub audio_url: Option<Url>,

    /// Video URL.
    ///
    /// Eg. `https://video.com/video.mp4`.
    pub video_url: Option<Url>,

    /// Other arbitrary key-value mapping.
    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}

pub fn token_extra_example() -> TokenExtra {
    TokenExtra {
        price: Some(1u128.into()),
        audio_url: Some(
            "https://audio.com/audio.mp3"
                .parse::<url::Url>()
                .unwrap()
                .into(),
        ),
        video_url: Some(
            "https://video.com/video.mp4"
                .parse::<url::Url>()
                .unwrap()
                .into(),
        ),
        others: [
            (
                "key_a".to_string(),
                Value::from(near_sdk::serde_json::json!(false)),
            ),
            (
                "key_b".to_string(),
                Value::from(near_sdk::serde_json::json!([0u8, 1u8])),
            ),
            (
                "key_c".to_string(),
                Value::from(near_sdk::serde_json::json!({
                    "inner_a": "value"
                })),
            ),
        ]
        .into_iter()
        .collect(),
    }
}

// ====================================================================

/// UUIDs.
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[derive(
    Clone, PartialOrd, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
pub struct TokenId(pub String);

impl AsRef<str> for TokenId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for TokenId {
    fn from(t: String) -> Self {
        TokenId(t)
    }
}

impl JsonSchema for TokenId {
    fn is_referenceable() -> bool {
        true
    }
    fn schema_name() -> String {
        ("TokenId").to_owned()
    }
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let s_validation = near_sdk::schemars::schema::StringValidation {
            min_length: Some(0),
            max_length: Some(256),
            ..Default::default()
        };
        let meta = Metadata {
            description: Some("Token ID. \n\nThis can be (or have been) created \"manually\", or \"from a group\".".into()),
            default: Some(serde_json::json!("some-token")),
            examples: vec![
                serde_json::json!("some-token"),
                serde_json::json!("some-group_77"),
            ],
            ..Default::default()
        };

        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            metadata: Box::new(meta).into(),
            string: Some(Box::new(s_validation)),
            subschemas: Some(Box::new(near_sdk::schemars::schema::SubschemaValidation {
                any_of: Some(vec![
                    gen.subschema_for::<TokenIdManual>(),
                    gen.subschema_for::<TokenIdGroupItem>(),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

// ====================================================================== //

/// TokenId for manually created tokens.
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[derive(
    Clone, PartialOrd, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
#[serde(deny_unknown_fields)]
pub struct TokenIdManual(pub String);

impl JsonSchema for TokenIdManual {
    fn is_referenceable() -> bool {
        true
    }
    fn schema_name() -> String {
        ("TokenIdManual").to_owned()
    }
    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        let s_validation = near_sdk::schemars::schema::StringValidation {
            min_length: Some(0),
            max_length: Some(256),
            pattern: Some(r#"^[^_]*$"#.into()),
        };
        let meta = Metadata {
            description: Some("Token ID for tokens created manually. \n\nThis is when a token doesn't refer to any named group of tokens.  \n\n - Cannot have any underscore (`_`) on it's id.  \n\n - If it has a metadata related to it, then that metadata's properties:  \n    - `copies` - should be either `null` or `1`.  \n\n    - `extra.price` - should not exist.".into()),
            default: Some(serde_json::json!("some-token")),
            examples: vec![
                serde_json::json!("some-token"),
            ],
            ..Default::default()
        };

        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            metadata: Box::new(meta).into(),
            string: Some(Box::new(s_validation)),
            ..Default::default()
        }
        .into()
    }
}

impl TokenIdManual {
    fn check(&self) {
        near_sdk::require!(
            !self.0.contains(COUNT_SEPARATOR),
            &format!(
                "Manually created tokens must not use the separator `{}` in their token id",
                COUNT_SEPARATOR
            )
        );
    }

    pub fn check_with(
        &self,
        metadata: &TokenMetadata,
    ) {
        self.check();
        near_sdk::require!(
            matches!(metadata.copies.map(JsUint::get), None | Some(1)),
            "Manually created tokens must have a `metadata.copies` set to either `null` or `1`"
        );

        near_sdk::require!(
            metadata.extra.as_ref().and_then(|x| x.price).is_none(),
            "Manually created tokens must not have a `metadata.extra.price` value",
        );
    }
}

// ====================================================================== //

/// A token that has been created as part of a group of tokens.
///
/// It's token-id is: "{group name}_{item index}", where `item_index`
/// is the index of this token in it's group.
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[derive(
    Clone, PartialOrd, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
#[serde(deny_unknown_fields)]
pub struct TokenIdGroupItem(pub String);

impl TokenIdGroupItem {
    pub fn new(
        group_name: &TokenIdGroupName,
        index: u64,
    ) -> Self {
        TokenIdGroupItem(format!("{}_{}", group_name.0, index))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl JsonSchema for TokenIdGroupItem {
    fn is_referenceable() -> bool {
        true
    }
    fn schema_name() -> String {
        ("TokenIdGroupItem").to_owned()
    }
    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        let s_validation = near_sdk::schemars::schema::StringValidation {
            min_length: Some(0),
            max_length: Some(256),
            pattern: Some(r#"^[^_]*_\d+$"#.into()),
        };
        let meta = Metadata {
            description: Some("Token ID for tokens created from a named group.  \n\n This is when one additional (copy) token is created from a named group.  \n\n - The token has an index in the group, where the first token created has the index of `1`.  \n\n - It's resulting id is \"{group name}_{token index}\".".into()),
            default: Some(serde_json::json!("some-group_123")),
            examples: vec![
                serde_json::json!("some-group_123"),
            ],
            ..Default::default()
        };

        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            metadata: Box::new(meta).into(),
            string: Some(Box::new(s_validation)),
            ..Default::default()
        }
        .into()
    }
}

/// Name for a group of tokens.
///
/// Each token created from this group has it's token-id of:
/// "{group name}_{token index}".
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[derive(
    Clone, PartialOrd, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[serde(crate = "near_sdk::serde")]
#[serde(transparent)]
#[serde(deny_unknown_fields)]
pub struct TokenIdGroupName(pub String);

impl JsonSchema for TokenIdGroupName {
    fn is_referenceable() -> bool {
        true
    }
    fn schema_name() -> String {
        ("TokenIdGroupName").to_owned()
    }
    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        let s_validation = near_sdk::schemars::schema::StringValidation {
            min_length: Some(0),
            max_length: Some(128),
            pattern: None,
        };
        let meta = Metadata {
            description: Some("Name for a group of tokens.  \n\nEach token created from this group has it's token-id of: \"{group name}_{token index}\".".into()),
            default: Some(serde_json::json!("some-group")),
            examples: vec![
                serde_json::json!("some-group"),
            ],
            ..Default::default()
        };

        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            metadata: Box::new(meta).into(),
            string: Some(Box::new(s_validation)),
            ..Default::default()
        }
        .into()
    }
}

/// The Json token is what will be returned from view calls.
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug, PartialEq))]
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
#[serde(deny_unknown_fields)]
#[schemars(crate = "near_sdk::schemars")]
pub struct JsonToken {
    /// Token ID.
    pub token_id: TokenId,

    /// Token metadata.
    pub metadata: TokenMetadata,

    /// More token information.
    #[serde(flatten)]
    pub token: Token,
}

/// Token information.
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug, PartialEq))]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
#[serde(deny_unknown_fields)]
#[schemars(crate = "near_sdk::schemars")]
pub struct Token {
    /// Owner of the token.
    pub owner_id: AccountId,

    /// List of approved account IDs that have access to transfer the token.
    ///
    /// This maps an account ID to an approval ID.
    ///
    /// See
    /// [NEP-178](https://nomicon.io/Standards/Tokens/NonFungibleToken/ApprovalManagement)
    /// for more info.
    pub approved_account_ids: HashMap<AccountId, JsUint>,

    /// The next approval ID to give out.
    ///
    /// See
    /// [NEP-178](https://nomicon.io/Standards/Tokens/NonFungibleToken/ApprovalManagement)
    /// for more info.
    pub next_approval_id: JsUint,

    /// The royalty percentages setting for this token.
    ///
    /// See
    /// [NEP-199](https://nomicon.io/Standards/Tokens/NonFungibleToken/Payout)
    /// for more info.
    pub royalty: HashMap<AccountId, RoyaltyPercentage>,
}

impl Token {
    pub fn to_json(
        self,
        token_id: TokenId,
        metadata: TokenMetadata,
    ) -> JsonToken {
        JsonToken {
            token_id,
            metadata,
            token: self,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(crate = "near_sdk::schemars")]
#[serde(crate = "near_sdk::serde")]
pub struct ResaleTicket {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub token_group_id: TokenIdGroupName,
    pub usn_price: UsnAmount,
    pub extra: Option<TokenExtra>,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug, PartialEq))]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(deny_unknown_fields)]
pub struct TransferredTokenPayout {
    pub token_id: TokenId,
    pub successful_transfer: bool,
    pub royalty_payout: Option<HashMap<AccountId, RoyaltyPercentage>>,
}
/// This is an externally serializable structure passed as an argument in function calls
#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(crate = "near_sdk::schemars")]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateCollectibleData {
    pub token_id: Option<TokenId>,
    pub token_group_id: Option<TokenIdGroupName>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
    pub copies: Option<JsUint>,
    pub extra: Option<TokenExtra>,

    pub expires_at: Option<String>,
    pub starts_at: Option<String>,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug, PartialEq))]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(deny_unknown_fields)]
pub struct TicketForSale {
    pub token_id: TokenId,
    pub owner: AccountId,
    pub token_group_id: TokenIdGroupName,
    pub original_price: UsnAmount,      // unscaled pice in usn
    pub usn_sell_price: UsnAmount,      // unscaled price in usn
    pub usn_transfer_amount: U128,      // scaled price in usn
    pub service_fee: RoyaltyPercentage, //expressed in percentage out of 10,000 = 100 %
    pub listed_at: Option<String>,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug, PartialEq))]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(deny_unknown_fields)]
pub struct BuyTicketRequest {
    pub token_id: TokenId,
    pub price_usn: String, // price in usn (unscaled)
}
