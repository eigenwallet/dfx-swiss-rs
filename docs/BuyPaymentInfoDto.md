# BuyPaymentInfoDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**bank** | **String** |  | 
**street** | **String** |  | 
**number** | **String** |  | 
**zip** | **String** |  | 
**city** | **String** |  | 
**country** | **String** |  | 
**iban** | Option<**String**> |  | [optional]
**bic** | **String** |  | 
**sepa_instant** | **bool** |  | 
**id** | **f64** | Transaction order ID | 
**uid** | **String** | UID of the transaction order | 
**timestamp** | **String** | Price timestamp | 
**route_id** | **f64** |  | 
**remittance_info** | Option<**String**> |  | [optional]
**min_deposit** | [**models::MinAmount**](MinAmount.md) |  | 
**fee** | **f64** | Fee in percentage | 
**min_fee** | **f64** | Minimum fee in source currency | 
**fees** | [**models::FeeDto**](FeeDto.md) | Fee infos in source currency | 
**min_volume** | **f64** | Minimum volume in source currency | 
**max_volume** | **f64** | Maximum volume in source currency | 
**amount** | **f64** | Amount in source currency | 
**currency** | [**models::FiatDto**](FiatDto.md) | Source currency | 
**min_fee_target** | **f64** | Minimum fee in target asset | 
**fees_target** | [**models::FeeDto**](FeeDto.md) | Fee infos in target asset | 
**min_volume_target** | **f64** | Minimum volume in target asset | 
**max_volume_target** | **f64** | Maximum volume in target asset | 
**exchange_rate** | **f64** | Exchange rate in source/target | 
**rate** | **f64** | Final rate (incl. fees) in source/target | 
**exact_price** | **bool** | Exact or approximate price | 
**price_steps** | [**Vec<models::PriceStep>**](PriceStep.md) |  | 
**estimated_amount** | **f64** | Estimated amount in target asset | 
**asset** | [**models::AssetDto**](AssetDto.md) | Target asset | 
**payment_request** | Option<**String**> | Payment request (e.g. GiroCode content) | [optional]
**is_valid** | **bool** |  | 
**error** | Option<**String**> | Error message in case isValid is false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


