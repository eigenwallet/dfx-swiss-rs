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
pub struct UserKycDto {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "level")]
    pub level: Level,
    #[serde(rename = "dataComplete")]
    pub data_complete: bool,
}

impl UserKycDto {
    pub fn new(hash: String, level: Level, data_complete: bool) -> UserKycDto {
        UserKycDto {
            hash,
            level,
            data_complete,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "10")]
    Variant10,
    #[serde(rename = "20")]
    Variant20,
    #[serde(rename = "30")]
    Variant30,
    #[serde(rename = "40")]
    Variant40,
    #[serde(rename = "50")]
    Variant50,
    #[serde(rename = "-10")]
    Variant102,
    #[serde(rename = "-20")]
    Variant202,
}

impl Default for Level {
    fn default() -> Level {
        Self::Variant0
    }
}

