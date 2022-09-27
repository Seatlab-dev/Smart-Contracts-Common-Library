use near_contract_standards::non_fungible_token::metadata::NFT_METADATA_SPEC;

// NFT Metadata
pub const NFT_STANDARD: &'static str = "nep171";
pub const NFT_VERSION: &'static str = NFT_METADATA_SPEC;
pub const MAX_PAYOUT_BENEFICIARIES: u32 = 10;
pub const COUNT_SEPARATOR: &str = "_";

// Hashmap Keys
pub const RESALE_PRICE_KEY: &'static str = "resale_price";
pub const MAX_RESALE_PRICE_KEY: &'static str = "max_resale_price";
pub const TRANSFERS_ENABLED_KEY: &'static str = "transfers_enabled";
pub const RESALES_ENABLED_KEY: &'static str = "resales_enabled";
