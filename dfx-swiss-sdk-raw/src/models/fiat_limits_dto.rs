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
pub struct FiatLimitsDto {
    #[serde(rename = "Bank")]
    pub bank: Box<models::VolumeLimitDto>,
    #[serde(rename = "Instant")]
    pub instant: Box<models::VolumeLimitDto>,
    #[serde(rename = "Card")]
    pub card: Box<models::VolumeLimitDto>,
}

impl FiatLimitsDto {
    pub fn new(bank: models::VolumeLimitDto, instant: models::VolumeLimitDto, card: models::VolumeLimitDto) -> FiatLimitsDto {
        FiatLimitsDto {
            bank: Box::new(bank),
            instant: Box::new(instant),
            card: Box::new(card),
        }
    }
}

