# \CustodyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**custody_controller_confirm_order**](CustodyApi.md#custody_controller_confirm_order) | **POST** /v1/custody/order/{id}/confirm | 
[**custody_controller_create_custody_account**](CustodyApi.md#custody_controller_create_custody_account) | **POST** /v1/custody | 
[**custody_controller_create_order**](CustodyApi.md#custody_controller_create_order) | **POST** /v1/custody/order | 
[**custody_controller_get_user_custody_balance**](CustodyApi.md#custody_controller_get_user_custody_balance) | **GET** /v1/custody | 
[**custody_controller_get_user_custody_history**](CustodyApi.md#custody_controller_get_user_custody_history) | **GET** /v1/custody/history | 



## custody_controller_confirm_order

> custody_controller_confirm_order(id)


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


## custody_controller_create_custody_account

> models::CustodyAuthDto custody_controller_create_custody_account(create_custody_account_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_custody_account_dto** | [**CreateCustodyAccountDto**](CreateCustodyAccountDto.md) |  | [required] |

### Return type

[**models::CustodyAuthDto**](CustodyAuthDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custody_controller_create_order

> models::CustodyOrderDto custody_controller_create_order(get_custody_info_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_custody_info_dto** | [**GetCustodyInfoDto**](GetCustodyInfoDto.md) |  | [required] |

### Return type

[**models::CustodyOrderDto**](CustodyOrderDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custody_controller_get_user_custody_balance

> models::CustodyBalanceDto custody_controller_get_user_custody_balance()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CustodyBalanceDto**](CustodyBalanceDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custody_controller_get_user_custody_history

> models::CustodyHistoryDto custody_controller_get_user_custody_history()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CustodyHistoryDto**](CustodyHistoryDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

