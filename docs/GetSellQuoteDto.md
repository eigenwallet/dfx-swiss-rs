# GetSellQuoteDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset** | [**models::EntityDto**](EntityDto.md) | Source asset | 
**amount** | Option<**f64**> | Amount in source asset | [optional]
**currency** | [**models::EntityDto**](EntityDto.md) | Target currency | 
**target_amount** | Option<**f64**> | Amount in target currency | [optional]
**discount_code** | Option<**String**> | This field is deprecated, use \"specialCode\" instead. | [optional]
**special_code** | Option<**String**> | Special code | [optional]
**wallet** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


