# \AuthApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_controller_authenticate**](AuthApi.md#auth_controller_authenticate) | **POST** /v1/auth | 
[**auth_controller_company_challenge**](AuthApi.md#auth_controller_company_challenge) | **GET** /v1/auth/challenge | 
[**auth_controller_get_sign_message**](AuthApi.md#auth_controller_get_sign_message) | **GET** /v1/auth/signMessage | 
[**auth_controller_sign_in**](AuthApi.md#auth_controller_sign_in) | **POST** /v1/auth/signIn | 
[**auth_controller_sign_in_by_mail**](AuthApi.md#auth_controller_sign_in_by_mail) | **POST** /v1/auth/mail | 
[**auth_controller_sign_up**](AuthApi.md#auth_controller_sign_up) | **POST** /v1/auth/signUp | 



## auth_controller_authenticate

> models::AuthResponseDto auth_controller_authenticate(sign_up_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_up_dto** | [**SignUpDto**](SignUpDto.md) |  | [required] |

### Return type

[**models::AuthResponseDto**](AuthResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_controller_company_challenge

> models::ChallengeDto auth_controller_company_challenge(address)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**models::ChallengeDto**](ChallengeDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_controller_get_sign_message

> models::SignMessageDto auth_controller_get_sign_message(address)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**models::SignMessageDto**](SignMessageDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_controller_sign_in

> models::AuthResponseDto auth_controller_sign_in(sign_in_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_in_dto** | [**SignInDto**](SignInDto.md) |  | [required] |

### Return type

[**models::AuthResponseDto**](AuthResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_controller_sign_in_by_mail

> auth_controller_sign_in_by_mail(auth_mail_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_mail_dto** | [**AuthMailDto**](AuthMailDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_controller_sign_up

> models::AuthResponseDto auth_controller_sign_up(sign_up_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_up_dto** | [**SignUpDto**](SignUpDto.md) |  | [required] |

### Return type

[**models::AuthResponseDto**](AuthResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

