# UserV2Dto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **f64** | Unique account id | 
**account_type** | Option<**String**> |  | [optional]
**mail** | Option<**String**> |  | [optional]
**phone** | Option<**String**> |  | [optional]
**language** | [**models::LanguageDto**](LanguageDto.md) |  | 
**currency** | [**models::FiatDto**](FiatDto.md) |  | 
**trading_limit** | [**models::TradingLimit**](TradingLimit.md) |  | 
**kyc** | [**models::UserKycDto**](UserKycDto.md) |  | 
**volumes** | [**models::VolumesDto**](VolumesDto.md) |  | 
**addresses** | [**Vec<models::UserAddressDto>**](UserAddressDto.md) |  | 
**disabled_addresses** | [**Vec<models::UserAddressDto>**](UserAddressDto.md) |  | 
**active_address** | Option<[**models::UserAddressDto**](UserAddressDto.md)> |  | [optional]
**payment_link** | [**models::UserPaymentLinkDto**](UserPaymentLinkDto.md) |  | 
**api_key_ct** | Option<**String**> |  | [optional]
**api_filter_ct** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


