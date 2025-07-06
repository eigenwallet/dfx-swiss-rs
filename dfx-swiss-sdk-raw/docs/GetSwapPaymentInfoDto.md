# GetSwapPaymentInfoDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_asset** | [**models::EntityDto**](EntityDto.md) | Source asset | 
**amount** | Option<**f64**> | Amount in source asset | [optional]
**target_asset** | [**models::EntityDto**](EntityDto.md) | Target asset | 
**target_amount** | Option<**f64**> | Amount in target asset | [optional]
**external_transaction_id** | Option<**String**> | Custom transaction id | [optional]
**exact_price** | Option<**bool**> | Require an exact price (may take longer) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


