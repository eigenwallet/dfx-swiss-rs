# KycSessionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kyc_level** | **f64** |  | 
**trading_limit** | [**models::TradingLimit**](TradingLimit.md) |  | 
**kyc_clients** | [**Vec<Vec<String>>**](Vec.md) | Connected KYC clients | 
**language** | [**models::LanguageDto**](LanguageDto.md) |  | 
**kyc_steps** | [**Vec<models::KycStepDto>**](KycStepDto.md) |  | 
**current_step** | Option<[**models::KycStepSessionDto**](KycStepSessionDto.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


