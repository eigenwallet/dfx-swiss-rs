# \SupportApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**support_issue_controller_create_issue**](SupportApi.md#support_issue_controller_create_issue) | **POST** /v1/support/issue | 
[**support_issue_controller_create_support_message**](SupportApi.md#support_issue_controller_create_support_message) | **POST** /v1/support/issue/{id}/message | 
[**support_issue_controller_get_file**](SupportApi.md#support_issue_controller_get_file) | **GET** /v1/support/issue/{id}/message/{messageId}/file | 
[**support_issue_controller_get_issue**](SupportApi.md#support_issue_controller_get_issue) | **GET** /v1/support/issue/{id} | 
[**support_issue_controller_get_issues**](SupportApi.md#support_issue_controller_get_issues) | **GET** /v1/support/issue | 



## support_issue_controller_create_issue

> support_issue_controller_create_issue(create_support_issue_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_support_issue_dto** | [**CreateSupportIssueDto**](CreateSupportIssueDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_issue_controller_create_support_message

> support_issue_controller_create_support_message(id, create_support_message_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_support_message_dto** | [**CreateSupportMessageDto**](CreateSupportMessageDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_issue_controller_get_file

> support_issue_controller_get_file(id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_issue_controller_get_issue

> support_issue_controller_get_issue(id, from_message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**from_message_id** | Option<**f64**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## support_issue_controller_get_issues

> support_issue_controller_get_issues()


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

