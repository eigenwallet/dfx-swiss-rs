# \SellApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sell_controller_confirm_sell**](SellApi.md#sell_controller_confirm_sell) | **PUT** /v1/sell/paymentInfos/{id}/confirm | 
[**sell_controller_create_sell_with_payment_info**](SellApi.md#sell_controller_create_sell_with_payment_info) | **PUT** /v1/sell/paymentInfos | 
[**sell_controller_get_sell**](SellApi.md#sell_controller_get_sell) | **GET** /v1/sell/{id} | 
[**sell_controller_get_sell_quote**](SellApi.md#sell_controller_get_sell_quote) | **PUT** /v1/sell/quote | 



## sell_controller_confirm_sell

> models::TransactionDto sell_controller_confirm_sell(id, confirm_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**confirm_dto** | [**ConfirmDto**](ConfirmDto.md) |  | [required] |

### Return type

[**models::TransactionDto**](TransactionDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sell_controller_create_sell_with_payment_info

> models::SellPaymentInfoDto sell_controller_create_sell_with_payment_info(get_sell_payment_info_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sell_payment_info_dto** | [**GetSellPaymentInfoDto**](GetSellPaymentInfoDto.md) |  | [required] |

### Return type

[**models::SellPaymentInfoDto**](SellPaymentInfoDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sell_controller_get_sell

> models::SellDto sell_controller_get_sell(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SellDto**](SellDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sell_controller_get_sell_quote

> models::SellQuoteDto sell_controller_get_sell_quote(get_sell_quote_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sell_quote_dto** | [**GetSellQuoteDto**](GetSellQuoteDto.md) |  | [required] |

### Return type

[**models::SellQuoteDto**](SellQuoteDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

