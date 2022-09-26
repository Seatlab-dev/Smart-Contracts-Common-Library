pub mod collections;
pub mod js_integer;
pub mod owners;
pub mod refund;
pub mod royalty;
pub mod usn;
pub mod wrapped_url;
pub mod wrapped_value;

#[cfg(feature = "sim")]
pub mod sim;

pub use contract_version as version;
pub use js_integer::JsUint;
pub use wrapped_url::Url;
pub use wrapped_value::Value;

/// Stringifies some yoctoNEAR amount into it's NEAR representation.
/// The NEAR unit is not appended into the string.
///
/// Examples:
/// - 1000000000000000000000000 (yN) => "1"
/// - 1100000000000000000000000 (yN) => "1.1"
/// - 1010000000000000000000000 (yN) => "1.01"
/// - 1000000000000000000000001 (yN) => "1.000000000000000000000001"
/// - 0100000000000000000000000 (yN) => "0.1"
pub fn yocto_to_near(yoctos: u128) -> String {
    const KILO: u128 = 1000;
    const MEGA: u128 = KILO * KILO;
    const TERA: u128 = MEGA * MEGA;
    const YOTTA: u128 = TERA * TERA;

    let a = (yoctos / YOTTA).to_string();
    let b = (yoctos % YOTTA).to_string();

    let padding = 24 - b.chars().count();

    format!("{}.{}{}", a, "0".repeat(padding), b)
}
