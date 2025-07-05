# \LnurlApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_lnurl_controller_get_lnurl_auth**](LnurlApi.md#auth_lnurl_controller_get_lnurl_auth) | **POST** /v1/lnurla | 
[**auth_lnurl_controller_lnurl_auth_status**](LnurlApi.md#auth_lnurl_controller_lnurl_auth_status) | **GET** /v1/lnurla/status | 
[**auth_lnurl_controller_sign_in_with_lnurl_auth**](LnurlApi.md#auth_lnurl_controller_sign_in_with_lnurl_auth) | **GET** /v1/lnurla | 
[**ln_url_p_forward_controller_cancel_payment**](LnurlApi.md#ln_url_p_forward_controller_cancel_payment) | **DELETE** /v1/lnurlp/cancel/{id} | 
[**ln_url_p_forward_controller_ln_url_p_callback_forward**](LnurlApi.md#ln_url_p_forward_controller_ln_url_p_callback_forward) | **GET** /v1/lnurlp/cb/{id} | 
[**ln_url_p_forward_controller_ln_url_p_forward**](LnurlApi.md#ln_url_p_forward_controller_ln_url_p_forward) | **GET** /v1/lnurlp/{id} | 
[**ln_url_p_forward_controller_tx_hex_forward**](LnurlApi.md#ln_url_p_forward_controller_tx_hex_forward) | **GET** /v1/lnurlp/tx/{id} | 
[**ln_url_p_forward_controller_wait_for_payment**](LnurlApi.md#ln_url_p_forward_controller_wait_for_payment) | **GET** /v1/lnurlp/wait/{id} | 
[**ln_url_w_forward_controller_ln_url_w_callback_forward**](LnurlApi.md#ln_url_w_forward_controller_ln_url_w_callback_forward) | **GET** /v1/lnurlw/cb/{id} | 
[**ln_url_w_forward_controller_ln_url_w_forward**](LnurlApi.md#ln_url_w_forward_controller_ln_url_w_forward) | **GET** /v1/lnurlw/{id} | 
[**lnurld_forward_controller_lnurld_callback_forward**](LnurlApi.md#lnurld_forward_controller_lnurld_callback_forward) | **GET** /v1/lnurld/cb/{id}/{var} | 
[**lnurld_forward_controller_lnurld_forward**](LnurlApi.md#lnurld_forward_controller_lnurld_forward) | **GET** /v1/lnurld/{id} | 



## auth_lnurl_controller_get_lnurl_auth

> models::AuthLnurlCreateLoginResponseDto auth_lnurl_controller_get_lnurl_auth()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthLnurlCreateLoginResponseDto**](AuthLnurlCreateLoginResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_lnurl_controller_lnurl_auth_status

> models::AuthLnurlStatusResponseDto auth_lnurl_controller_lnurl_auth_status(k1)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**k1** | **String** |  | [required] |

### Return type

[**models::AuthLnurlStatusResponseDto**](AuthLnurlStatusResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_lnurl_controller_sign_in_with_lnurl_auth

> models::AuthLnurlSignInResponseDto auth_lnurl_controller_sign_in_with_lnurl_auth(tag, action, k1, sig, key, address, signature, used_ref, wallet, discount_code, special_code, moderator)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** |  | [required] |
**action** | **String** |  | [required] |
**k1** | **String** |  | [required] |
**sig** | **String** |  | [required] |
**key** | **String** |  | [required] |
**address** | **String** |  | [required] |
**signature** | **String** |  | [required] |
**used_ref** | Option<**String**> |  |  |
**wallet** | Option<**String**> |  |  |
**discount_code** | Option<**String**> | This field is deprecated, use \"specialCode\" instead. |  |
**special_code** | Option<**String**> | Special code |  |
**moderator** | Option<**String**> | Moderator |  |

### Return type

[**models::AuthLnurlSignInResponseDto**](AuthLnurlSignInResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ln_url_p_forward_controller_cancel_payment

> ln_url_p_forward_controller_cancel_payment(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ln_url_p_forward_controller_ln_url_p_callback_forward

> ln_url_p_forward_controller_ln_url_p_callback_forward(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ln_url_p_forward_controller_ln_url_p_forward

> ln_url_p_forward_controller_ln_url_p_forward(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ln_url_p_forward_controller_tx_hex_forward

> ln_url_p_forward_controller_tx_hex_forward(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ln_url_p_forward_controller_wait_for_payment

> ln_url_p_forward_controller_wait_for_payment(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ln_url_w_forward_controller_ln_url_w_callback_forward

> ln_url_w_forward_controller_ln_url_w_callback_forward(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ln_url_w_forward_controller_ln_url_w_forward

> ln_url_w_forward_controller_ln_url_w_forward(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lnurld_forward_controller_lnurld_callback_forward

> lnurld_forward_controller_lnurld_callback_forward(id, var)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**var** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lnurld_forward_controller_lnurld_forward

> lnurld_forward_controller_lnurld_forward(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

