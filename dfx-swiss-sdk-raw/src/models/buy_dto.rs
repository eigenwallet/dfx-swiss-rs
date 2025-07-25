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
pub struct BuyDto {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "asset")]
    pub asset: Box<models::AssetDto>,
    #[serde(rename = "bankUsage", skip_serializing_if = "Option::is_none")]
    pub bank_usage: Option<String>,
    /// Volume in CHF
    #[serde(rename = "volume")]
    pub volume: f64,
    /// Annual volume in CHF
    #[serde(rename = "annualVolume")]
    pub annual_volume: f64,
    #[serde(rename = "fee")]
    pub fee: f64,
    #[serde(rename = "minDeposits")]
    pub min_deposits: Vec<models::MinAmount>,
    #[serde(rename = "minFee")]
    pub min_fee: Box<models::MinAmount>,
}

impl BuyDto {
    pub fn new(id: f64, active: bool, asset: models::AssetDto, volume: f64, annual_volume: f64, fee: f64, min_deposits: Vec<models::MinAmount>, min_fee: models::MinAmount) -> BuyDto {
        BuyDto {
            id,
            active,
            iban: None,
            asset: Box::new(asset),
            bank_usage: None,
            volume,
            annual_volume,
            fee,
            min_deposits,
            min_fee: Box::new(min_fee),
        }
    }
}

