use lazy_static::lazy_static;
use near_contract_standards::non_fungible_token::metadata::NFT_METADATA_SPEC;
use crate::usn::UsnAmount;

// Prices
lazy_static! {
    #[derive(Debug)]
    pub static ref MIN_USN_RESALE_PRICE: UsnAmount = UsnAmount::new(1., None);
    #[derive(Debug)]
    pub static ref MAX_USN_RESALE_PRICE: UsnAmount = UsnAmount::new(100_000_000., None);
}

// Hashmap Keys
pub const RESALE_PRICE_KEY: &'static str = "resale_price";
pub const MAX_RESALE_PRICE_KEY: &'static str = "max_resale_price";
pub const TRANSFERS_ENABLED_KEY: &'static str = "transfers_enabled";
pub const RESALES_ENABLED_KEY: &'static str = "resales_enabled";

// NFT Metadata
pub const NFT_STANDARD: &'static str = "nep171";
pub const NFT_VERSION: &'static str = NFT_METADATA_SPEC;
pub const MAX_PAYOUT_BENEFICIARIES: u32 = 10;
pub const COUNT_SEPARATOR: &str = "_";
pub const SLUG_DELIMITER: &'static str = "_";
pub const DATA_IMAGE_SVG_NEAR_ICON: &'static str = "data:image/svg+xml,%3C%3Fxml version='1.0' encoding='utf-8'%3F%3E%3C!-- Generator: Adobe Illustrator 26.3.0, SVG Export Plug-In . SVG Version: 6.00 Build 0) --%3E%3Csvg version='1.1' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' x='0px' y='0px' width='32px' height='32px' viewBox='0 0 32 32' style='enable-background:new 0 0 32 32;' xml:space='preserve'%3E%3Cstyle type='text/css'%3E .st0%7Bfill:%23050035;%7D .st1%7Bfill:url(%23SVGID_1_);%7D%0A%3C/style%3E%3Cg id='Layer_1'%3E%3Ccircle class='st0' cx='16' cy='16' r='16'/%3E%3C/g%3E%3Cg id='Isolation_Mode'%3E%3ClinearGradient id='SVGID_1_' gradientUnits='userSpaceOnUse' x1='6.2375' y1='16' x2='25.7625' y2='16'%3E%3Cstop offset='0' style='stop-color:%2300C2FF'/%3E%3Cstop offset='1' style='stop-color:%230067FF'/%3E%3C/linearGradient%3E%3Cpath class='st1' d='M14,6.1c-0.3,0-0.5,0.1-0.8,0.2L8.1,9.4L7.8,9.6L7.2,10c-0.5,0.3-0.9,0.6-0.9,1.7v4.1c0,0.9,0.3,1.6,0.9,1.9 l6.2,3.8c1,0.6,2.3-0.1,2.3-1.3v-2.9c0-0.7-0.4-1.4-1-1.8l-2.9-1.8l3-1.9c0.5-0.3,0.8-0.9,0.8-1.5V7.6C15.6,6.7,14.8,6.1,14,6.1z M14.7,10.4c0,0.3-0.2,0.6-0.4,0.8l-3.4,2.1l-1.8-1.1c-0.5-0.3-0.8-0.5-1.1-0.8c-0.3-0.3-0.2-0.8,0.2-1c0,0,0,0,0,0L13.7,7 C13.8,7,13.9,6.9,14,6.9c0.3,0,0.7,0.3,0.7,0.7V10.4z M24.9,14.3l-6.2-3.8c-1-0.6-2.3,0.1-2.3,1.3v2.9c0,0.7,0.4,1.4,1,1.8l2.9,1.8 l-3,1.9c-0.5,0.3-0.8,0.9-0.8,1.5v2.8c0,0.9,0.8,1.6,1.6,1.6c0.3,0,0.5-0.1,0.8-0.2l5.1-3.1l0.4-0.2l0.6-0.4 c0.5-0.3,0.9-0.6,0.9-1.7v-4.1C25.8,15.3,25.4,14.6,24.9,14.3z M23.8,21.6C23.8,21.6,23.8,21.6,23.8,21.6L18.3,25 c-0.1,0.1-0.2,0.1-0.4,0.1c-0.3,0-0.7-0.3-0.7-0.7v-2.8c0-0.3,0.2-0.6,0.4-0.8l3.4-2.1l1.8,1.1c0.5,0.3,0.8,0.5,1.1,0.8 C24.3,20.9,24.2,21.4,23.8,21.6z'/%3E%3C/g%3E%3C/svg%3E%0A";