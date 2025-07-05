# SellQuoteDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fee_amount** | **f64** | Fee amount in source asset | 
**amount** | **f64** | Amount in source asset | 
**estimated_amount** | **f64** | Estimated amount in target currency | 
**exchange_rate** | **f64** | Exchange rate in source/target | 
**rate** | **f64** | Final rate (incl. fees) in source/target | 
**fees** | [**models::FeeDto**](FeeDto.md) | Fee infos in source asset | 
**fees_target** | [**models::FeeDto**](FeeDto.md) | Fee infos in target currency | 
**min_volume** | **f64** | Minimum volume in source asset | 
**max_volume** | **f64** | Maximum Volume in source asset | 
**min_volume_target** | **f64** | Minimum volume in target currency | 
**max_volume_target** | **f64** | Maximum volume in target currency | 
**price_steps** | [**Vec<models::PriceStep>**](PriceStep.md) |  | 
**is_valid** | **bool** |  | 
**error** | Option<**String**> | Error message in case isValid is false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


