/*
 * DFX API
 *
 * DFX API PRD (updated on 7/3/2025, 12:42:13 PM)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentLinkRecipientDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<models::PaymentLinkRecipientAddressDto>>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "mail", skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(rename = "registrationNumber", skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(rename = "storeType", skip_serializing_if = "Option::is_none")]
    pub store_type: Option<StoreType>,
    #[serde(rename = "merchantMcc", skip_serializing_if = "Option::is_none")]
    pub merchant_mcc: Option<MerchantMcc>,
    #[serde(rename = "goodsType", skip_serializing_if = "Option::is_none")]
    pub goods_type: Option<GoodsType>,
    #[serde(rename = "goodsCategory", skip_serializing_if = "Option::is_none")]
    pub goods_category: Option<GoodsCategory>,
}

impl PaymentLinkRecipientDto {
    pub fn new() -> PaymentLinkRecipientDto {
        PaymentLinkRecipientDto {
            name: None,
            address: None,
            phone: None,
            mail: None,
            website: None,
            registration_number: None,
            store_type: None,
            merchant_mcc: None,
            goods_type: None,
            goods_category: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StoreType {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "-1")]
    Variant12,
}

impl Default for StoreType {
    fn default() -> StoreType {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MerchantMcc {
    #[serde(rename = "1001")]
    Variant1001,
    #[serde(rename = "1002")]
    Variant1002,
    #[serde(rename = "1004")]
    Variant1004,
    #[serde(rename = "1005")]
    Variant1005,
    #[serde(rename = "1006")]
    Variant1006,
    #[serde(rename = "1007")]
    Variant1007,
    #[serde(rename = "1008")]
    Variant1008,
    #[serde(rename = "1009")]
    Variant1009,
    #[serde(rename = "1010")]
    Variant1010,
    #[serde(rename = "1011")]
    Variant1011,
    #[serde(rename = "1012")]
    Variant1012,
    #[serde(rename = "1013")]
    Variant1013,
    #[serde(rename = "1014")]
    Variant1014,
    #[serde(rename = "1015")]
    Variant1015,
    #[serde(rename = "1016")]
    Variant1016,
    #[serde(rename = "1017")]
    Variant1017,
    #[serde(rename = "1018")]
    Variant1018,
    #[serde(rename = "1019")]
    Variant1019,
    #[serde(rename = "1020")]
    Variant1020,
    #[serde(rename = "1021")]
    Variant1021,
    #[serde(rename = "1022")]
    Variant1022,
    #[serde(rename = "1024")]
    Variant1024,
    #[serde(rename = "1025")]
    Variant1025,
    #[serde(rename = "1026")]
    Variant1026,
    #[serde(rename = "1027")]
    Variant1027,
    #[serde(rename = "1028")]
    Variant1028,
    #[serde(rename = "1029")]
    Variant1029,
    #[serde(rename = "1030")]
    Variant1030,
    #[serde(rename = "1031")]
    Variant1031,
    #[serde(rename = "1032")]
    Variant1032,
    #[serde(rename = "1033")]
    Variant1033,
    #[serde(rename = "1034")]
    Variant1034,
    #[serde(rename = "1035")]
    Variant1035,
    #[serde(rename = "1036")]
    Variant1036,
    #[serde(rename = "1037")]
    Variant1037,
    #[serde(rename = "1038")]
    Variant1038,
    #[serde(rename = "1039")]
    Variant1039,
    #[serde(rename = "1040")]
    Variant1040,
    #[serde(rename = "1041")]
    Variant1041,
    #[serde(rename = "1042")]
    Variant1042,
    #[serde(rename = "1043")]
    Variant1043,
    #[serde(rename = "1044")]
    Variant1044,
    #[serde(rename = "1045")]
    Variant1045,
    #[serde(rename = "1046")]
    Variant1046,
    #[serde(rename = "1047")]
    Variant1047,
    #[serde(rename = "9999")]
    Variant9999,
}

impl Default for MerchantMcc {
    fn default() -> MerchantMcc {
        Self::Variant1001
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GoodsType {
    #[serde(rename = "01")]
    Variant01,
    #[serde(rename = "02")]
    Variant02,
}

impl Default for GoodsType {
    fn default() -> GoodsType {
        Self::Variant01
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GoodsCategory {
    #[serde(rename = "0000")]
    Variant0000,
    #[serde(rename = "1000")]
    Variant1000,
    #[serde(rename = "2000")]
    Variant2000,
    #[serde(rename = "3000")]
    Variant3000,
    #[serde(rename = "4000")]
    Variant4000,
    #[serde(rename = "5000")]
    Variant5000,
    #[serde(rename = "6000")]
    Variant6000,
    #[serde(rename = "7000")]
    Variant7000,
    #[serde(rename = "8000")]
    Variant8000,
    #[serde(rename = "9000")]
    Variant9000,
    #[serde(rename = "A000")]
    A000,
    #[serde(rename = "B000")]
    B000,
    #[serde(rename = "C000")]
    C000,
    #[serde(rename = "D000")]
    D000,
    #[serde(rename = "E000")]
    E000,
    #[serde(rename = "F000")]
    F000,
    #[serde(rename = "Z000")]
    Z000,
}

impl Default for GoodsCategory {
    fn default() -> GoodsCategory {
        Self::Variant0000
    }
}

