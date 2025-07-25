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
pub struct KycFileData {
    /// Base64 encoded file
    #[serde(rename = "file")]
    pub file: String,
    /// Name of the file
    #[serde(rename = "fileName")]
    pub file_name: String,
}

impl KycFileData {
    pub fn new(file: String, file_name: String) -> KycFileData {
        KycFileData {
            file,
            file_name,
        }
    }
}

