# \UserApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_controller_add_discount_code**](UserApi.md#user_controller_add_discount_code) | **PUT** /v1/user/discountCodes | 
[**user_controller_add_special_code**](UserApi.md#user_controller_add_special_code) | **PUT** /v1/user/specialCodes | 
[**user_controller_change_user**](UserApi.md#user_controller_change_user) | **POST** /v1/user/change | 
[**user_controller_create_api_key**](UserApi.md#user_controller_create_api_key) | **POST** /v1/user/apiKey/CT | 
[**user_controller_delete_api_key**](UserApi.md#user_controller_delete_api_key) | **DELETE** /v1/user/apiKey/CT | 
[**user_controller_delete_user**](UserApi.md#user_controller_delete_user) | **DELETE** /v1/user | 
[**user_controller_delete_user_account**](UserApi.md#user_controller_delete_user_account) | **DELETE** /v1/user/account | 
[**user_controller_get_user_detail_v1**](UserApi.md#user_controller_get_user_detail_v1) | **GET** /v1/user/detail | 
[**user_controller_get_user_v1**](UserApi.md#user_controller_get_user_v1) | **GET** /v1/user | 
[**user_controller_update_api_filter**](UserApi.md#user_controller_update_api_filter) | **PUT** /v1/user/apiFilter/CT | 
[**user_controller_update_kyc_data**](UserApi.md#user_controller_update_kyc_data) | **POST** /v1/user/data | 
[**user_controller_update_user_v1**](UserApi.md#user_controller_update_user_v1) | **PUT** /v1/user | 
[**user_v2_controller_delete_account**](UserApi.md#user_v2_controller_delete_account) | **DELETE** /v2/user | 
[**user_v2_controller_delete_address**](UserApi.md#user_v2_controller_delete_address) | **DELETE** /v2/user/addresses/{address} | 
[**user_v2_controller_get_ref**](UserApi.md#user_v2_controller_get_ref) | **GET** /v2/user/ref | 
[**user_v2_controller_get_user**](UserApi.md#user_v2_controller_get_user) | **GET** /v2/user | 
[**user_v2_controller_update_address**](UserApi.md#user_v2_controller_update_address) | **PUT** /v2/user/addresses/{address} | 
[**user_v2_controller_update_user**](UserApi.md#user_v2_controller_update_user) | **PUT** /v2/user | 
[**user_v2_controller_update_user_mail**](UserApi.md#user_v2_controller_update_user_mail) | **PUT** /v2/user/mail | 
[**user_v2_controller_verify_mail**](UserApi.md#user_v2_controller_verify_mail) | **POST** /v2/user/mail/verify | 



## user_controller_add_discount_code

> user_controller_add_discount_code(code)


This endpoint is deprecated, use \"specialCodes\" instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_add_special_code

> user_controller_add_special_code(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_change_user

> models::AuthResponseDto user_controller_change_user(linked_user_in_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linked_user_in_dto** | [**LinkedUserInDto**](LinkedUserInDto.md) |  | [required] |

### Return type

[**models::AuthResponseDto**](AuthResponseDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_create_api_key

> models::ApiKeyDto user_controller_create_api_key(buy, sell, staking, r#ref, lm)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**buy** | Option<**bool**> |  |  |
**sell** | Option<**bool**> |  |  |
**staking** | Option<**bool**> |  |  |
**r#ref** | Option<**bool**> |  |  |
**lm** | Option<**bool**> |  |  |

### Return type

[**models::ApiKeyDto**](ApiKeyDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_delete_api_key

> user_controller_delete_api_key()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_delete_user

> user_controller_delete_user()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_delete_user_account

> user_controller_delete_user_account()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_get_user_detail_v1

> models::UserDetailDto user_controller_get_user_detail_v1()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserDetailDto**](UserDetailDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_get_user_v1

> models::UserDto user_controller_get_user_v1()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserDto**](UserDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_update_api_filter

> Vec<String> user_controller_update_api_filter(buy, sell, staking, r#ref, lm)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**buy** | Option<**bool**> |  |  |
**sell** | Option<**bool**> |  |  |
**staking** | Option<**bool**> |  |  |
**r#ref** | Option<**bool**> |  |  |
**lm** | Option<**bool**> |  |  |

### Return type

**Vec<String>**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_update_kyc_data

> models::UserDetailDto user_controller_update_kyc_data(kyc_input_data_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kyc_input_data_dto** | [**KycInputDataDto**](KycInputDataDto.md) |  | [required] |

### Return type

[**models::UserDetailDto**](UserDetailDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_update_user_v1

> models::UserDetailDto user_controller_update_user_v1(update_user_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_dto** | [**UpdateUserDto**](UpdateUserDto.md) |  | [required] |

### Return type

[**models::UserDetailDto**](UserDetailDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_v2_controller_delete_account

> user_v2_controller_delete_account()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_v2_controller_delete_address

> user_v2_controller_delete_address(address)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_v2_controller_get_ref

> models::ReferralDto user_v2_controller_get_ref()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReferralDto**](ReferralDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_v2_controller_get_user

> models::UserV2Dto user_v2_controller_get_user()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserV2Dto**](UserV2Dto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_v2_controller_update_address

> models::UserV2Dto user_v2_controller_update_address(address, update_address_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**update_address_dto** | [**UpdateAddressDto**](UpdateAddressDto.md) |  | [required] |

### Return type

[**models::UserV2Dto**](UserV2Dto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_v2_controller_update_user

> models::UserV2Dto user_v2_controller_update_user(update_user_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_dto** | [**UpdateUserDto**](UpdateUserDto.md) |  | [required] |

### Return type

[**models::UserV2Dto**](UserV2Dto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_v2_controller_update_user_mail

> user_v2_controller_update_user_mail(update_user_mail_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_mail_dto** | [**UpdateUserMailDto**](UpdateUserMailDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_v2_controller_verify_mail

> user_v2_controller_verify_mail(verify_mail_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_mail_dto** | [**VerifyMailDto**](VerifyMailDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

