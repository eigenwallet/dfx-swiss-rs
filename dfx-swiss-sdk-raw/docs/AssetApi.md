# \AssetApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**asset_controller_get_all_asset**](AssetApi.md#asset_controller_get_all_asset) | **GET** /v1/asset | 



## asset_controller_get_all_asset

> Vec<models::AssetDetailDto> asset_controller_get_all_asset(blockchains)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchains** | Option<**String**> | Comma-separated blockchain list |  |

### Return type

[**Vec<models::AssetDetailDto>**](AssetDetailDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

