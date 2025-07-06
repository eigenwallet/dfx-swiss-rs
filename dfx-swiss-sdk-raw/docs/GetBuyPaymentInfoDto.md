# GetBuyPaymentInfoDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**iban** | Option<**String**> |  | [optional]
**currency** | [**models::EntityDto**](EntityDto.md) | Source currency | 
**asset** | [**models::EntityDto**](EntityDto.md) | Target asset | 
**amount** | Option<**f64**> | Amount in source currency | [optional]
**target_amount** | Option<**f64**> | Amount in target asset | [optional]
**external_transaction_id** | Option<**String**> | Custom transaction id | [optional]
**exact_price** | Option<**bool**> | Require an exact price (may take longer) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


