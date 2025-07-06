# \PaymentLinkApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_link_controller_cancel_payment**](PaymentLinkApi.md#payment_link_controller_cancel_payment) | **DELETE** /v1/paymentLink/payment | 
[**payment_link_controller_confirm_payment**](PaymentLinkApi.md#payment_link_controller_confirm_payment) | **PUT** /v1/paymentLink/payment/confirm | 
[**payment_link_controller_create_payment**](PaymentLinkApi.md#payment_link_controller_create_payment) | **POST** /v1/paymentLink/payment | 
[**payment_link_controller_create_payment_link**](PaymentLinkApi.md#payment_link_controller_create_payment_link) | **POST** /v1/paymentLink | 
[**payment_link_controller_get_all_payment_links**](PaymentLinkApi.md#payment_link_controller_get_all_payment_links) | **GET** /v1/paymentLink | 
[**payment_link_controller_get_payment_history**](PaymentLinkApi.md#payment_link_controller_get_payment_history) | **GET** /v1/paymentLink/history | 
[**payment_link_controller_get_user_payment_links_config**](PaymentLinkApi.md#payment_link_controller_get_user_payment_links_config) | **GET** /v1/paymentLink/config | 
[**payment_link_controller_update_payment_link**](PaymentLinkApi.md#payment_link_controller_update_payment_link) | **PUT** /v1/paymentLink | 
[**payment_link_controller_update_user_payment_links_config**](PaymentLinkApi.md#payment_link_controller_update_user_payment_links_config) | **PUT** /v1/paymentLink/config | 
[**payment_link_controller_wait_for_payment**](PaymentLinkApi.md#payment_link_controller_wait_for_payment) | **GET** /v1/paymentLink/payment/wait | 



## payment_link_controller_cancel_payment

> models::PaymentLinkDto payment_link_controller_cancel_payment(link_id, external_link_id, external_payment_id, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_id** | Option<**String**> | Link ID |  |
**external_link_id** | Option<**String**> | External link ID |  |
**external_payment_id** | Option<**String**> | External payment ID |  |
**key** | Option<**String**> | Payment link access key |  |

### Return type

[**models::PaymentLinkDto**](PaymentLinkDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_link_controller_confirm_payment

> models::PaymentLinkDto payment_link_controller_confirm_payment(link_id, external_link_id, external_payment_id, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_id** | Option<**String**> | Link ID |  |
**external_link_id** | Option<**String**> | External link ID |  |
**external_payment_id** | Option<**String**> | External payment ID |  |
**key** | Option<**String**> | Payment link access key |  |

### Return type

[**models::PaymentLinkDto**](PaymentLinkDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_link_controller_create_payment

> models::PaymentLinkDto payment_link_controller_create_payment(create_payment_link_payment_dto, link_id, external_link_id, key, route)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_link_payment_dto** | [**CreatePaymentLinkPaymentDto**](CreatePaymentLinkPaymentDto.md) |  | [required] |
**link_id** | Option<**String**> | Link ID |  |
**external_link_id** | Option<**String**> | External link ID |  |
**key** | Option<**String**> | Payment link access key |  |
**route** | Option<**String**> | Route label |  |

### Return type

[**models::PaymentLinkDto**](PaymentLinkDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_link_controller_create_payment_link

> models::PaymentLinkDto payment_link_controller_create_payment_link(create_payment_link_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_link_dto** | [**CreatePaymentLinkDto**](CreatePaymentLinkDto.md) |  | [required] |

### Return type

[**models::PaymentLinkDto**](PaymentLinkDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_link_controller_get_all_payment_links

> Vec<models::PaymentLinkDto> payment_link_controller_get_all_payment_links(link_id, external_link_id, external_payment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_id** | Option<**String**> | Link ID |  |
**external_link_id** | Option<**String**> | External link ID |  |
**external_payment_id** | Option<**String**> | External payment ID |  |

### Return type

[**Vec<models::PaymentLinkDto>**](PaymentLinkDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_link_controller_get_payment_history

> Vec<models::PaymentLinkHistoryDto> payment_link_controller_get_payment_history(status, from, to, external_link_id, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Comma-separated list of statuses. Default is \"completed\" |  |
**from** | Option<**String**> | From date (yyyy-mm-dd). Default is first day of current month |  |
**to** | Option<**String**> | To date (yyyy-mm-dd). Default is last day of current month |  |
**external_link_id** | Option<**String**> | External link ID |  |
**key** | Option<**String**> | Payment link access key |  |

### Return type

[**Vec<models::PaymentLinkHistoryDto>**](PaymentLinkHistoryDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_link_controller_get_user_payment_links_config

> models::PaymentLinkConfigDto payment_link_controller_get_user_payment_links_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PaymentLinkConfigDto**](PaymentLinkConfigDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_link_controller_update_payment_link

> models::PaymentLinkDto payment_link_controller_update_payment_link(update_payment_link_dto, link_id, external_link_id, external_payment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_payment_link_dto** | [**UpdatePaymentLinkDto**](UpdatePaymentLinkDto.md) |  | [required] |
**link_id** | Option<**String**> | Link ID |  |
**external_link_id** | Option<**String**> | External link ID |  |
**external_payment_id** | Option<**String**> | External payment ID |  |

### Return type

[**models::PaymentLinkDto**](PaymentLinkDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_link_controller_update_user_payment_links_config

> payment_link_controller_update_user_payment_links_config(update_payment_link_config_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_payment_link_config_dto** | [**UpdatePaymentLinkConfigDto**](UpdatePaymentLinkConfigDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_link_controller_wait_for_payment

> models::PaymentLinkDto payment_link_controller_wait_for_payment(link_id, external_link_id, external_payment_id, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_id** | Option<**String**> | Link ID |  |
**external_link_id** | Option<**String**> | External link ID |  |
**external_payment_id** | Option<**String**> | External payment ID |  |
**key** | Option<**String**> | Payment link access key |  |

### Return type

[**models::PaymentLinkDto**](PaymentLinkDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

