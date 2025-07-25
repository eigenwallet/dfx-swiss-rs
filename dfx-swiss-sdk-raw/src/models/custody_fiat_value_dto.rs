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
pub struct CustodyFiatValueDto {
    /// Value in Swiss Franc
    #[serde(rename = "chf")]
    pub chf: f64,
    /// Value in Euro
    #[serde(rename = "eur")]
    pub eur: f64,
    /// Value in US Dollar
    #[serde(rename = "usd")]
    pub usd: f64,
}

impl CustodyFiatValueDto {
    pub fn new(chf: f64, eur: f64, usd: f64) -> CustodyFiatValueDto {
        CustodyFiatValueDto {
            chf,
            eur,
            usd,
        }
    }
}

