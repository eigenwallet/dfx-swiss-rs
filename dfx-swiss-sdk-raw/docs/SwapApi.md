# \SwapApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**swap_controller_confirm_swap**](SwapApi.md#swap_controller_confirm_swap) | **PUT** /v1/swap/paymentInfos/{id}/confirm | 
[**swap_controller_create_swap_with_payment_info**](SwapApi.md#swap_controller_create_swap_with_payment_info) | **PUT** /v1/swap/paymentInfos | 
[**swap_controller_get_swap**](SwapApi.md#swap_controller_get_swap) | **GET** /v1/swap/{id} | 
[**swap_controller_get_swap_quote**](SwapApi.md#swap_controller_get_swap_quote) | **PUT** /v1/swap/quote | 



## swap_controller_confirm_swap

> models::TransactionDto swap_controller_confirm_swap(id, confirm_dto)


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


## swap_controller_create_swap_with_payment_info

> models::SwapPaymentInfoDto swap_controller_create_swap_with_payment_info(get_swap_payment_info_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_swap_payment_info_dto** | [**GetSwapPaymentInfoDto**](GetSwapPaymentInfoDto.md) |  | [required] |

### Return type

[**models::SwapPaymentInfoDto**](SwapPaymentInfoDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_controller_get_swap

> models::SwapDto swap_controller_get_swap(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SwapDto**](SwapDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_controller_get_swap_quote

> models::SwapQuoteDto swap_controller_get_swap_quote(get_swap_quote_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_swap_quote_dto** | [**GetSwapQuoteDto**](GetSwapQuoteDto.md) |  | [required] |

### Return type

[**models::SwapQuoteDto**](SwapQuoteDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

