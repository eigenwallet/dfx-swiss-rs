# CustodyOrderResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **f64** | Transaction order ID | 
**uid** | **String** | UID of the transaction order | 
**timestamp** | **String** | Price timestamp | 
**min_volume** | **f64** | Minimum volume in source asset | 
**max_volume** | **f64** | Maximum Volume in source asset | 
**amount** | **f64** | Amount in source asset | 
**source_asset** | **String** | Source asset name, Asset or Fiat | 
**target_asset** | **String** | Target asset name, Asset or Fiat | 
**fees** | [**models::FeeDto**](FeeDto.md) | Fee infos in source asset | 
**fees_target** | [**models::FeeDto**](FeeDto.md) | Fee infos in target asset | 
**min_volume_target** | **f64** | Minimum volume in target asset | 
**max_volume_target** | **f64** | Maximum volume in target asset | 
**exchange_rate** | **f64** | Exchange rate in source/target | 
**rate** | **f64** | Final rate (incl. fees) in source/target | 
**price_steps** | [**Vec<models::PriceStep>**](PriceStep.md) |  | 
**estimated_amount** | **f64** | Estimated amount in target asset | 
**payment_request** | Option<**String**> | Payment request (e.g. Lightning invoice) | [optional]
**is_valid** | **bool** |  | 
**error** | Option<**String**> | Error message in case isValid is false | [optional]
**beneficiary** | Option<[**models::BeneficiaryDto**](BeneficiaryDto.md)> | Bank transaction beneficiary | [optional]
**buy_infos** | Option<[**models::CustodyOrderBuyResponseDto**](CustodyOrderBuyResponseDto.md)> | Infos for Buy Custody Orders | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


