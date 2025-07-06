# SwapDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **f64** |  | 
**active** | **bool** |  | 
**asset** | [**models::AssetDto**](AssetDto.md) |  | 
**deposit** | Option<[**models::DepositDto**](DepositDto.md)> |  | [optional]
**volume** | **f64** | Volume in CHF | 
**annual_volume** | **f64** | Annual volume in CHF | 
**fee** | **f64** |  | 
**blockchain** | **String** |  | 
**min_deposits** | [**Vec<models::MinAmount>**](MinAmount.md) |  | 
**min_fee** | [**models::MinAmount**](MinAmount.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


