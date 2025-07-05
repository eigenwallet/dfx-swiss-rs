# SwapPaymentInfoDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **f64** | Transaction order ID | 
**uid** | **String** | UID of the transaction order | 
**timestamp** | **String** | Price timestamp | 
**route_id** | **f64** |  | 
**deposit_address** | Option<**String**> |  | [optional]
**blockchain** | **String** |  | 
**min_deposit** | [**models::MinAmount**](MinAmount.md) |  | 
**fee** | **f64** | Fee in percentage | 
**min_fee** | **f64** | Minimum fee in source asset | 
**fees** | [**models::FeeDto**](FeeDto.md) | Fee infos in source asset | 
**min_volume** | **f64** | Minimum volume in source asset | 
**max_volume** | **f64** | Maximum volume in source asset | 
**amount** | **f64** | Amount in source asset | 
**source_asset** | [**models::AssetDto**](AssetDto.md) | Source asset | 
**min_fee_target** | **f64** | Minimum fee in target asset | 
**fees_target** | [**models::FeeDto**](FeeDto.md) | Fee infos in target asset | 
**min_volume_target** | **f64** | Minimum volume in target asset | 
**max_volume_target** | **f64** | Maximum volume in target asset | 
**exchange_rate** | **f64** | Exchange rate in source/target | 
**rate** | **f64** | Final rate (incl. fees) in source/target | 
**exact_price** | **bool** | Exact or approximate price | 
**price_steps** | [**Vec<models::PriceStep>**](PriceStep.md) |  | 
**estimated_amount** | **f64** | Estimated amount in target asset | 
**target_asset** | [**models::AssetDto**](AssetDto.md) | Target asset | 
**payment_request** | Option<**String**> | Payment request (e.g. Lightning invoice) | [optional]
**is_valid** | **bool** |  | 
**error** | Option<**String**> | Error message in case isValid is false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


