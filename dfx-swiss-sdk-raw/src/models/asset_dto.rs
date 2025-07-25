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
pub struct AssetDto {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "chainId", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<f64>,
    #[serde(rename = "explorerUrl", skip_serializing_if = "Option::is_none")]
    pub explorer_url: Option<String>,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "category")]
    pub category: Category,
    #[serde(rename = "dexName")]
    pub dex_name: String,
    #[serde(rename = "feeTier")]
    pub fee_tier: FeeTier,
    #[serde(rename = "comingSoon")]
    pub coming_soon: bool,
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
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<f64>,
}

impl AssetDto {
    pub fn new(id: f64, name: String, unique_name: String, description: String, r#type: Type, category: Category, dex_name: String, fee_tier: FeeTier, coming_soon: bool, buyable: bool, sellable: bool, card_buyable: bool, card_sellable: bool, instant_buyable: bool, instant_sellable: bool, blockchain: Blockchain) -> AssetDto {
        AssetDto {
            id,
            name,
            chain_id: None,
            decimals: None,
            explorer_url: None,
            unique_name,
            description,
            r#type,
            category,
            dex_name,
            fee_tier,
            coming_soon,
            buyable,
            sellable,
            card_buyable,
            card_sellable,
            instant_buyable,
            instant_sellable,
            blockchain,
            sort_order: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Coin")]
    Coin,
    #[serde(rename = "Token")]
    Token,
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "Custody")]
    Custody,
    #[serde(rename = "Pool")]
    Pool,
    #[serde(rename = "Presale")]
    Presale,
}

impl Default for Type {
    fn default() -> Type {
        Self::Coin
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "Public")]
    Public,
    #[serde(rename = "Private")]
    Private,
}

impl Default for Category {
    fn default() -> Category {
        Self::Public
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeeTier {
    #[serde(rename = "Tier0")]
    Tier0,
    #[serde(rename = "Tier1")]
    Tier1,
    #[serde(rename = "Tier2")]
    Tier2,
    #[serde(rename = "Tier3")]
    Tier3,
    #[serde(rename = "Tier4")]
    Tier4,
}

impl Default for FeeTier {
    fn default() -> FeeTier {
        Self::Tier0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Blockchain {
    #[serde(rename = "Bitcoin")]
    Bitcoin,
    #[serde(rename = "Lightning")]
    Lightning,
    #[serde(rename = "Monero")]
    Monero,
    #[serde(rename = "Ethereum")]
    Ethereum,
    #[serde(rename = "BinanceSmartChain")]
    BinanceSmartChain,
    #[serde(rename = "Optimism")]
    Optimism,
    #[serde(rename = "Arbitrum")]
    Arbitrum,
    #[serde(rename = "Polygon")]
    Polygon,
    #[serde(rename = "Base")]
    Base,
    #[serde(rename = "Haqq")]
    Haqq,
    #[serde(rename = "Liquid")]
    Liquid,
    #[serde(rename = "Arweave")]
    Arweave,
    #[serde(rename = "Cardano")]
    Cardano,
    #[serde(rename = "DeFiChain")]
    DeFiChain,
    #[serde(rename = "Railgun")]
    Railgun,
    #[serde(rename = "BinancePay")]
    BinancePay,
    #[serde(rename = "Solana")]
    Solana,
    #[serde(rename = "Gnosis")]
    Gnosis,
}

impl Default for Blockchain {
    fn default() -> Blockchain {
        Self::Bitcoin
    }
}

