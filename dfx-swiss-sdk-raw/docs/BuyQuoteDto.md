# BuyQuoteDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fee_amount** | **f64** | Fee amount in source currency | 
**amount** | **f64** | Amount in source currency | 
**estimated_amount** | **f64** | Estimated amount in target asset | 
**exchange_rate** | **f64** | Exchange rate in source/target | 
**rate** | **f64** | Final rate (incl. fees) in source/target | 
**min_volume** | **f64** | Minimum volume in source currency | 
**max_volume** | **f64** | Maximum volume in source currency | 
**fees** | [**models::FeeDto**](FeeDto.md) | Fee infos in source currency | 
**fees_target** | [**models::FeeDto**](FeeDto.md) | Fee infos in target asset | 
**min_volume_target** | **f64** | Minimum volume in target asset | 
**max_volume_target** | **f64** | Maximum volume in target asset | 
**price_steps** | [**Vec<models::PriceStep>**](PriceStep.md) |  | 
**is_valid** | **bool** |  | 
**error** | Option<**String**> | Error message in case isValid is false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


