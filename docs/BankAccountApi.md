# \BankAccountApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bank_account_controller_create_bank_account**](BankAccountApi.md#bank_account_controller_create_bank_account) | **POST** /v1/bankAccount | 
[**bank_account_controller_get_all_user_bank_account**](BankAccountApi.md#bank_account_controller_get_all_user_bank_account) | **GET** /v1/bankAccount | 
[**bank_account_controller_update_bank_account**](BankAccountApi.md#bank_account_controller_update_bank_account) | **PUT** /v1/bankAccount/{id} | 



## bank_account_controller_create_bank_account

> models::BankAccountDto bank_account_controller_create_bank_account(create_bank_account_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_bank_account_dto** | [**CreateBankAccountDto**](CreateBankAccountDto.md) |  | [required] |

### Return type

[**models::BankAccountDto**](BankAccountDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_account_controller_get_all_user_bank_account

> Vec<models::BankAccountDto> bank_account_controller_get_all_user_bank_account()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::BankAccountDto>**](BankAccountDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_account_controller_update_bank_account

> models::BankAccountDto bank_account_controller_update_bank_account(id, update_bank_account_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_bank_account_dto** | [**UpdateBankAccountDto**](UpdateBankAccountDto.md) |  | [required] |

### Return type

[**models::BankAccountDto**](BankAccountDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

