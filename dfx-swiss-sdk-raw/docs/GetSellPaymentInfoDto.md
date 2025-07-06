# GetSellPaymentInfoDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**iban** | **String** |  | 
**asset** | [**models::EntityDto**](EntityDto.md) | Source asset | 
**currency** | [**models::EntityDto**](EntityDto.md) | Target currency | 
**amount** | Option<**f64**> | Amount in source asset | [optional]
**target_amount** | Option<**f64**> | Amount in target currency | [optional]
**external_transaction_id** | Option<**String**> | Custom transaction id | [optional]
**exact_price** | Option<**bool**> | Require an exact price (may take longer) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


