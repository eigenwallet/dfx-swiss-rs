# \KycClientApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**kyc_client_controller_get_all_kyc_data**](KycClientApi.md#kyc_client_controller_get_all_kyc_data) | **GET** /v2/kyc/client/users | 
[**kyc_client_controller_get_all_kyc_data_v1**](KycClientApi.md#kyc_client_controller_get_all_kyc_data_v1) | **GET** /v1/kyc/users | 
[**kyc_client_controller_get_all_payments**](KycClientApi.md#kyc_client_controller_get_all_payments) | **GET** /v2/kyc/client/payments | 
[**kyc_client_controller_get_kyc_file**](KycClientApi.md#kyc_client_controller_get_kyc_file) | **GET** /v2/kyc/client/users/{id}/documents/{type} | 
[**kyc_client_controller_get_kyc_file_v1**](KycClientApi.md#kyc_client_controller_get_kyc_file_v1) | **GET** /v1/kyc/{id}/documents/{type} | 
[**kyc_client_controller_get_kyc_files**](KycClientApi.md#kyc_client_controller_get_kyc_files) | **GET** /v2/kyc/client/users/{id}/documents | 
[**kyc_client_controller_get_kyc_files_v1**](KycClientApi.md#kyc_client_controller_get_kyc_files_v1) | **GET** /v1/kyc/{id}/documents | 
[**kyc_client_controller_get_user_payments**](KycClientApi.md#kyc_client_controller_get_user_payments) | **GET** /v2/kyc/client/users/{id}/payments | 



## kyc_client_controller_get_all_kyc_data

> Vec<models::KycClientDataDto> kyc_client_controller_get_all_kyc_data()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::KycClientDataDto>**](KycClientDataDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_client_controller_get_all_kyc_data_v1

> Vec<models::KycDataDto> kyc_client_controller_get_all_kyc_data_v1()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::KycDataDto>**](KycDataDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_client_controller_get_all_payments

> Vec<models::PaymentWebhookData> kyc_client_controller_get_all_payments(from, to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** |  | [required] |
**to** | **String** |  | [required] |

### Return type

[**Vec<models::PaymentWebhookData>**](PaymentWebhookData.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_client_controller_get_kyc_file

> serde_json::Value kyc_client_controller_get_kyc_file(id, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**r#type** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_client_controller_get_kyc_file_v1

> serde_json::Value kyc_client_controller_get_kyc_file_v1(id, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**r#type** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_client_controller_get_kyc_files

> Vec<models::KycReportDto> kyc_client_controller_get_kyc_files(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**Vec<models::KycReportDto>**](KycReportDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_client_controller_get_kyc_files_v1

> Vec<models::KycFileDto> kyc_client_controller_get_kyc_files_v1(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**Vec<models::KycFileDto>**](KycFileDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_client_controller_get_user_payments

> Vec<models::PaymentWebhookData> kyc_client_controller_get_user_payments(id, from, to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**from** | **String** |  | [required] |
**to** | **String** |  | [required] |

### Return type

[**Vec<models::PaymentWebhookData>**](PaymentWebhookData.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

