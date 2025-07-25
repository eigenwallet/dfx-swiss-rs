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
pub struct CreatePaymentLinkPaymentDto {
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "expiryDate", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
}

impl CreatePaymentLinkPaymentDto {
    pub fn new(amount: f64) -> CreatePaymentLinkPaymentDto {
        CreatePaymentLinkPaymentDto {
            mode: None,
            amount,
            external_id: None,
            note: None,
            currency: None,
            expiry_date: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "Single")]
    Single,
    #[serde(rename = "Multiple")]
    Multiple,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Single
    }
}

