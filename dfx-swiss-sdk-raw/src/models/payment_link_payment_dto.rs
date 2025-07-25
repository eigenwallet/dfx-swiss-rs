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
pub struct PaymentLinkPaymentDto {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "mode")]
    pub mode: Mode,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "expiryDate")]
    pub expiry_date: String,
    #[serde(rename = "txCount")]
    pub tx_count: f64,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "lnurl")]
    pub lnurl: String,
}

impl PaymentLinkPaymentDto {
    pub fn new(id: f64, status: Status, amount: f64, currency: String, mode: Mode, date: String, expiry_date: String, tx_count: f64, is_confirmed: bool, url: String, lnurl: String) -> PaymentLinkPaymentDto {
        PaymentLinkPaymentDto {
            id,
            external_id: None,
            note: None,
            status,
            amount,
            currency,
            mode,
            date,
            expiry_date,
            tx_count,
            is_confirmed,
            url,
            lnurl,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Expired")]
    Expired,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
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

