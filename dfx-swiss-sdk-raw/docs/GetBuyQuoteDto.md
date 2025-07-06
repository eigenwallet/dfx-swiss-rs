# GetBuyQuoteDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | [**models::EntityDto**](EntityDto.md) | Source currency | 
**asset** | [**models::EntityDto**](EntityDto.md) | Target asset | 
**amount** | Option<**f64**> | Amount in source currency | [optional]
**target_amount** | Option<**f64**> | Amount in target asset | [optional]
**payment_method** | Option<**String**> | Payment method | [optional]
**discount_code** | Option<**String**> | This field is deprecated, use \"specialCode\" instead. | [optional]
**special_code** | Option<**String**> | Special code | [optional]
**wallet** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


