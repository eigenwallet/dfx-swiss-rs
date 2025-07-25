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
pub struct MergedDto {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "statusCode")]
    pub status_code: f64,
    #[serde(rename = "switchToCode")]
    pub switch_to_code: String,
}

impl MergedDto {
    pub fn new(error: String, message: String, status_code: f64, switch_to_code: String) -> MergedDto {
        MergedDto {
            error,
            message,
            status_code,
            switch_to_code,
        }
    }
}

