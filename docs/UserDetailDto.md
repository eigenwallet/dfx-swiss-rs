# UserDetailDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_type** | Option<**String**> |  | [optional]
**wallet** | **String** |  | 
**address** | **String** |  | 
**status** | **String** |  | 
**currency** | [**serde_json::Value**](.md) |  | 
**mail** | **String** |  | 
**phone** | **String** |  | 
**language** | [**models::LanguageDto**](LanguageDto.md) |  | 
**kyc_status** | **String** |  | 
**kyc_state** | **String** |  | 
**kyc_level** | **f64** |  | 
**kyc_hash** | **String** |  | 
**trading_limit** | [**models::TradingLimit**](TradingLimit.md) |  | 
**kyc_data_complete** | **bool** |  | 
**api_key_ct** | **String** |  | 
**api_filter_ct** | **Vec<String>** |  | 
**r#ref** | Option<**String**> |  | [optional]
**ref_fee_percent** | Option<**f64**> |  | [optional]
**ref_volume** | Option<**f64**> | Referral volume in EUR | [optional]
**ref_credit** | Option<**f64**> |  | [optional]
**paid_ref_credit** | Option<**f64**> |  | [optional]
**ref_count** | Option<**f64**> |  | [optional]
**ref_count_active** | Option<**f64**> |  | [optional]
**buy_volume** | [**models::VolumeInformation**](VolumeInformation.md) | Buy volume in CHF | 
**sell_volume** | [**models::VolumeInformation**](VolumeInformation.md) | Sell volume in CHF | 
**crypto_volume** | [**models::VolumeInformation**](VolumeInformation.md) | Crypto volume in CHF | 
**staking_balance** | **f64** |  | 
**linked_addresses** | Option<[**Vec<models::LinkedUserOutDto>**](LinkedUserOutDto.md)> |  | [optional]
**bs_link** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


