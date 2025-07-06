# \BuyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**buy_controller_confirm_buy**](BuyApi.md#buy_controller_confirm_buy) | **PUT** /v1/buy/paymentInfos/{id}/confirm | 
[**buy_controller_create_buy_with_payment_info**](BuyApi.md#buy_controller_create_buy_with_payment_info) | **PUT** /v1/buy/paymentInfos | 
[**buy_controller_generate_invoice_pdf**](BuyApi.md#buy_controller_generate_invoice_pdf) | **PUT** /v1/buy/paymentInfos/{id}/invoice | 
[**buy_controller_get_buy**](BuyApi.md#buy_controller_get_buy) | **GET** /v1/buy/{id} | 
[**buy_controller_get_buy_quote**](BuyApi.md#buy_controller_get_buy_quote) | **PUT** /v1/buy/quote | 



## buy_controller_confirm_buy

> buy_controller_confirm_buy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## buy_controller_create_buy_with_payment_info

> models::BuyPaymentInfoDto buy_controller_create_buy_with_payment_info(get_buy_payment_info_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_buy_payment_info_dto** | [**GetBuyPaymentInfoDto**](GetBuyPaymentInfoDto.md) |  | [required] |

### Return type

[**models::BuyPaymentInfoDto**](BuyPaymentInfoDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## buy_controller_generate_invoice_pdf

> models::PdfDto buy_controller_generate_invoice_pdf(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::PdfDto**](PdfDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## buy_controller_get_buy

> models::BuyDto buy_controller_get_buy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::BuyDto**](BuyDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## buy_controller_get_buy_quote

> models::BuyQuoteDto buy_controller_get_buy_quote(get_buy_quote_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_buy_quote_dto** | [**GetBuyQuoteDto**](GetBuyQuoteDto.md) |  | [required] |

### Return type

[**models::BuyQuoteDto**](BuyQuoteDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

