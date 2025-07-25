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
pub struct FiatDetailDto {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "buyable")]
    pub buyable: bool,
    #[serde(rename = "sellable")]
    pub sellable: bool,
    #[serde(rename = "cardBuyable")]
    pub card_buyable: bool,
    #[serde(rename = "cardSellable")]
    pub card_sellable: bool,
    #[serde(rename = "instantBuyable")]
    pub instant_buyable: bool,
    #[serde(rename = "instantSellable")]
    pub instant_sellable: bool,
    #[serde(rename = "limits")]
    pub limits: Box<models::FiatLimitsDto>,
}

impl FiatDetailDto {
    pub fn new(id: f64, name: String, buyable: bool, sellable: bool, card_buyable: bool, card_sellable: bool, instant_buyable: bool, instant_sellable: bool, limits: models::FiatLimitsDto) -> FiatDetailDto {
        FiatDetailDto {
            id,
            name,
            buyable,
            sellable,
            card_buyable,
            card_sellable,
            instant_buyable,
            instant_sellable,
            limits: Box::new(limits),
        }
    }
}

