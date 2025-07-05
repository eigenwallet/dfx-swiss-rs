# \TransactionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transaction_controller_create_csv**](TransactionApi.md#transaction_controller_create_csv) | **PUT** /v1/transaction/csv | 
[**transaction_controller_create_detail_csv**](TransactionApi.md#transaction_controller_create_detail_csv) | **PUT** /v1/transaction/detail/csv | 
[**transaction_controller_generate_invoice_from_transaction**](TransactionApi.md#transaction_controller_generate_invoice_from_transaction) | **PUT** /v1/transaction/{id}/invoice | 
[**transaction_controller_generate_receipt_from_transaction**](TransactionApi.md#transaction_controller_generate_receipt_from_transaction) | **PUT** /v1/transaction/{id}/receipt | 
[**transaction_controller_get_csv**](TransactionApi.md#transaction_controller_get_csv) | **GET** /v1/transaction/csv | 
[**transaction_controller_get_single_transaction**](TransactionApi.md#transaction_controller_get_single_transaction) | **GET** /v1/transaction/single | 
[**transaction_controller_get_single_transaction_details**](TransactionApi.md#transaction_controller_get_single_transaction_details) | **GET** /v1/transaction/detail/single | 
[**transaction_controller_get_transaction_details**](TransactionApi.md#transaction_controller_get_transaction_details) | **GET** /v1/transaction/detail | 
[**transaction_controller_get_transaction_refund**](TransactionApi.md#transaction_controller_get_transaction_refund) | **GET** /v1/transaction/{id}/refund | 
[**transaction_controller_get_transactions**](TransactionApi.md#transaction_controller_get_transactions) | **GET** /v1/transaction | 
[**transaction_controller_set_transaction_refund_target**](TransactionApi.md#transaction_controller_set_transaction_refund_target) | **PUT** /v1/transaction/{id}/refund | 



## transaction_controller_create_csv

> transaction_controller_create_csv(user_address, buy, sell, staking, r#ref, lm, format, from, to)


Initiate CSV history export

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_address** | **String** |  | [required] |
**buy** | Option<**bool**> |  |  |
**sell** | Option<**bool**> |  |  |
**staking** | Option<**bool**> |  |  |
**r#ref** | Option<**bool**> |  |  |
**lm** | Option<**bool**> |  |  |
**format** | Option<**String**> |  |  |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_controller_create_detail_csv

> transaction_controller_create_detail_csv(from, to)


Initiate CSV history export

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_controller_generate_invoice_from_transaction

> models::PdfDto transaction_controller_generate_invoice_from_transaction(id)


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


## transaction_controller_generate_receipt_from_transaction

> models::PdfDto transaction_controller_generate_receipt_from_transaction(id)


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


## transaction_controller_get_csv

> serde_json::Value transaction_controller_get_csv(key)


Get initiated CSV history export

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_controller_get_single_transaction

> models::TransactionDto transaction_controller_get_single_transaction(uid, order_uid, cko_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | Option<**String**> | Transaction unique ID |  |
**order_uid** | Option<**String**> | Order unique ID |  |
**cko_id** | Option<**String**> | CKO ID |  |

### Return type

[**models::TransactionDto**](TransactionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_controller_get_single_transaction_details

> models::TransactionDetailDto transaction_controller_get_single_transaction_details(id, uid, order_id, order_uid, external_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Transaction ID |  |
**uid** | Option<**String**> | Transaction unique ID |  |
**order_id** | Option<**String**> | Order ID |  |
**order_uid** | Option<**String**> | Order unique ID |  |
**external_id** | Option<**String**> | External transaction ID |  |

### Return type

[**models::TransactionDetailDto**](TransactionDetailDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_controller_get_transaction_details

> Vec<models::TransactionDetailDto> transaction_controller_get_transaction_details(from, to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |

### Return type

[**Vec<models::TransactionDetailDto>**](TransactionDetailDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_controller_get_transaction_refund

> transaction_controller_get_transaction_refund(id)


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


## transaction_controller_get_transactions

> Vec<models::TransactionDto> transaction_controller_get_transactions(user_address, buy, sell, staking, r#ref, lm, format, from, to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_address** | **String** |  | [required] |
**buy** | Option<**bool**> |  |  |
**sell** | Option<**bool**> |  |  |
**staking** | Option<**bool**> |  |  |
**r#ref** | Option<**bool**> |  |  |
**lm** | Option<**bool**> |  |  |
**format** | Option<**String**> |  |  |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |

### Return type

[**Vec<models::TransactionDto>**](TransactionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_controller_set_transaction_refund_target

> transaction_controller_set_transaction_refund_target(id, transaction_refund_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**transaction_refund_dto** | [**TransactionRefundDto**](TransactionRefundDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

