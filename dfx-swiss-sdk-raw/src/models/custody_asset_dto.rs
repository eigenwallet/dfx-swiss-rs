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
pub struct CustodyAssetDto {
    /// Asset name
    #[serde(rename = "name")]
    pub name: String,
    /// Asset description
    #[serde(rename = "description")]
    pub description: String,
}

impl CustodyAssetDto {
    pub fn new(name: String, description: String) -> CustodyAssetDto {
        CustodyAssetDto {
            name,
            description,
        }
    }
}

