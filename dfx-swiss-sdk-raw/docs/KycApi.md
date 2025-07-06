# \KycApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**kyc_controller_check2fa**](KycApi.md#kyc_controller_check2fa) | **GET** /v2/kyc/2fa | 
[**kyc_controller_continue_kyc**](KycApi.md#kyc_controller_continue_kyc) | **PUT** /v2/kyc | 
[**kyc_controller_get_file**](KycApi.md#kyc_controller_get_file) | **GET** /v2/kyc/file/{id} | 
[**kyc_controller_get_financial_data**](KycApi.md#kyc_controller_get_financial_data) | **GET** /v2/kyc/data/financial/{id} | 
[**kyc_controller_get_kyc_countries**](KycApi.md#kyc_controller_get_kyc_countries) | **GET** /v2/kyc/countries | 
[**kyc_controller_get_kyc_countries_by_code_v1**](KycApi.md#kyc_controller_get_kyc_countries_by_code_v1) | **GET** /v1/kyc/{code}/countries | 
[**kyc_controller_get_kyc_countries_v1**](KycApi.md#kyc_controller_get_kyc_countries_v1) | **GET** /v1/kyc/countries | 
[**kyc_controller_get_kyc_level**](KycApi.md#kyc_controller_get_kyc_level) | **GET** /v2/kyc | 
[**kyc_controller_get_kyc_progress_by_code_v1**](KycApi.md#kyc_controller_get_kyc_progress_by_code_v1) | **GET** /v1/kyc/{code} | 
[**kyc_controller_get_kyc_progress_v1**](KycApi.md#kyc_controller_get_kyc_progress_v1) | **GET** /v1/kyc | 
[**kyc_controller_request_kyc_by_code_v1**](KycApi.md#kyc_controller_request_kyc_by_code_v1) | **POST** /v1/kyc/{code} | 
[**kyc_controller_request_kyc_v1**](KycApi.md#kyc_controller_request_kyc_v1) | **POST** /v1/kyc | 
[**kyc_controller_start2fa**](KycApi.md#kyc_controller_start2fa) | **POST** /v2/kyc/2fa | 
[**kyc_controller_transfer_kyc_data_v1**](KycApi.md#kyc_controller_transfer_kyc_data_v1) | **PUT** /v1/kyc/transfer | 
[**kyc_controller_update_additional_documents_data**](KycApi.md#kyc_controller_update_additional_documents_data) | **PUT** /v2/kyc/data/additional/{id} | 
[**kyc_controller_update_authority_data**](KycApi.md#kyc_controller_update_authority_data) | **PUT** /v2/kyc/data/authority/{id} | 
[**kyc_controller_update_beneficial_data**](KycApi.md#kyc_controller_update_beneficial_data) | **PUT** /v2/kyc/data/beneficial/{id} | 
[**kyc_controller_update_commercial_register_data**](KycApi.md#kyc_controller_update_commercial_register_data) | **PUT** /v2/kyc/data/commercial/{id} | 
[**kyc_controller_update_contact_data**](KycApi.md#kyc_controller_update_contact_data) | **PUT** /v2/kyc/data/contact/{id} | 
[**kyc_controller_update_financial_data**](KycApi.md#kyc_controller_update_financial_data) | **PUT** /v2/kyc/data/financial/{id} | 
[**kyc_controller_update_ident_data**](KycApi.md#kyc_controller_update_ident_data) | **PUT** /v2/kyc/ident/manual/{id} | 
[**kyc_controller_update_legal_entity_data**](KycApi.md#kyc_controller_update_legal_entity_data) | **PUT** /v2/kyc/data/legal/{id} | 
[**kyc_controller_update_nationality_data**](KycApi.md#kyc_controller_update_nationality_data) | **PUT** /v2/kyc/data/nationality/{id} | 
[**kyc_controller_update_operational_data**](KycApi.md#kyc_controller_update_operational_data) | **PUT** /v2/kyc/data/operational/{id} | 
[**kyc_controller_update_owner_directory_data**](KycApi.md#kyc_controller_update_owner_directory_data) | **PUT** /v2/kyc/data/owner/{id} | 
[**kyc_controller_update_payments_data**](KycApi.md#kyc_controller_update_payments_data) | **PUT** /v2/kyc/data/payment/{id} | 
[**kyc_controller_update_personal_data**](KycApi.md#kyc_controller_update_personal_data) | **PUT** /v2/kyc/data/personal/{id} | 
[**kyc_controller_update_residence_permit_data**](KycApi.md#kyc_controller_update_residence_permit_data) | **PUT** /v2/kyc/data/residence/{id} | 
[**kyc_controller_update_signatory_power_data**](KycApi.md#kyc_controller_update_signatory_power_data) | **PUT** /v2/kyc/data/signatory/{id} | 
[**kyc_controller_verify2fa**](KycApi.md#kyc_controller_verify2fa) | **POST** /v2/kyc/2fa/verify | 



## kyc_controller_check2fa

> kyc_controller_check2fa(level)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**level** | Option<**String**> | 2FA level |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_continue_kyc

> models::KycSessionDto kyc_controller_continue_kyc(x_kyc_code, auto_step)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**auto_step** | **String** |  | [required] |

### Return type

[**models::KycSessionDto**](KycSessionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_get_file

> kyc_controller_get_file(id)


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


## kyc_controller_get_financial_data

> models::KycFinancialOutData kyc_controller_get_financial_data(x_kyc_code, id, lang)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**lang** | **String** |  | [required] |

### Return type

[**models::KycFinancialOutData**](KycFinancialOutData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_get_kyc_countries

> Vec<models::CountryDto> kyc_controller_get_kyc_countries(x_kyc_code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |

### Return type

[**Vec<models::CountryDto>**](CountryDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_get_kyc_countries_by_code_v1

> Vec<models::CountryDto> kyc_controller_get_kyc_countries_by_code_v1(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

[**Vec<models::CountryDto>**](CountryDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_get_kyc_countries_v1

> Vec<models::CountryDto> kyc_controller_get_kyc_countries_v1()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CountryDto>**](CountryDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_get_kyc_level

> models::KycLevelDto kyc_controller_get_kyc_level(x_kyc_code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |

### Return type

[**models::KycLevelDto**](KycLevelDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_get_kyc_progress_by_code_v1

> models::KycInfo kyc_controller_get_kyc_progress_by_code_v1(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

[**models::KycInfo**](KycInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_get_kyc_progress_v1

> models::KycInfo kyc_controller_get_kyc_progress_v1()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::KycInfo**](KycInfo.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_request_kyc_by_code_v1

> models::KycInfo kyc_controller_request_kyc_by_code_v1(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

[**models::KycInfo**](KycInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_request_kyc_v1

> models::KycInfo kyc_controller_request_kyc_v1()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::KycInfo**](KycInfo.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_start2fa

> models::Setup2faDto kyc_controller_start2fa(x_kyc_code, level)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**level** | Option<**String**> | 2FA level |  |

### Return type

[**models::Setup2faDto**](Setup2faDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_transfer_kyc_data_v1

> kyc_controller_transfer_kyc_data_v1(kyc_data_transfer_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kyc_data_transfer_dto** | [**KycDataTransferDto**](KycDataTransferDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_additional_documents_data

> models::KycStepBase kyc_controller_update_additional_documents_data(x_kyc_code, id, kyc_file_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_file_data** | [**KycFileData**](KycFileData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_authority_data

> models::KycStepBase kyc_controller_update_authority_data(x_kyc_code, id, kyc_file_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_file_data** | [**KycFileData**](KycFileData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_beneficial_data

> models::KycStepBase kyc_controller_update_beneficial_data(x_kyc_code, id, kyc_beneficial_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_beneficial_data** | [**KycBeneficialData**](KycBeneficialData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_commercial_register_data

> models::KycStepBase kyc_controller_update_commercial_register_data(x_kyc_code, id, kyc_file_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_file_data** | [**KycFileData**](KycFileData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_contact_data

> models::KycStepBase kyc_controller_update_contact_data(x_kyc_code, id, kyc_contact_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_contact_data** | [**KycContactData**](KycContactData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_financial_data

> models::KycStepBase kyc_controller_update_financial_data(x_kyc_code, id, kyc_financial_in_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_financial_in_data** | [**KycFinancialInData**](KycFinancialInData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_ident_data

> models::KycStepBase kyc_controller_update_ident_data(x_kyc_code, id, kyc_manual_ident_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_manual_ident_data** | [**KycManualIdentData**](KycManualIdentData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_legal_entity_data

> models::KycStepBase kyc_controller_update_legal_entity_data(x_kyc_code, id, kyc_legal_entity_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_legal_entity_data** | [**KycLegalEntityData**](KycLegalEntityData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_nationality_data

> models::KycStepBase kyc_controller_update_nationality_data(x_kyc_code, id, kyc_nationality_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_nationality_data** | [**KycNationalityData**](KycNationalityData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_operational_data

> models::KycStepBase kyc_controller_update_operational_data(x_kyc_code, id, kyc_operational_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_operational_data** | [**KycOperationalData**](KycOperationalData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_owner_directory_data

> models::KycStepBase kyc_controller_update_owner_directory_data(x_kyc_code, id, kyc_file_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_file_data** | [**KycFileData**](KycFileData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_payments_data

> models::KycStepBase kyc_controller_update_payments_data(x_kyc_code, id, payment_data_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**payment_data_dto** | [**PaymentDataDto**](PaymentDataDto.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_personal_data

> models::KycStepBase kyc_controller_update_personal_data(x_kyc_code, id, kyc_personal_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_personal_data** | [**KycPersonalData**](KycPersonalData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_residence_permit_data

> models::KycStepBase kyc_controller_update_residence_permit_data(x_kyc_code, id, kyc_file_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_file_data** | [**KycFileData**](KycFileData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_update_signatory_power_data

> models::KycStepBase kyc_controller_update_signatory_power_data(x_kyc_code, id, kyc_signatory_power_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**id** | **String** |  | [required] |
**kyc_signatory_power_data** | [**KycSignatoryPowerData**](KycSignatoryPowerData.md) |  | [required] |

### Return type

[**models::KycStepBase**](KycStepBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kyc_controller_verify2fa

> kyc_controller_verify2fa(x_kyc_code, verify2fa_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_kyc_code** | **String** |  | [required] |
**verify2fa_dto** | [**Verify2faDto**](Verify2faDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

