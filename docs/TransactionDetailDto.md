# TransactionDetailDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**f64**> |  | [optional]
**uid** | **String** | UID of the transaction | 
**order_uid** | Option<**String**> | UID of the order of the transaction | [optional]
**r#type** | **String** |  | 
**state** | **String** |  | 
**input_amount** | Option<**f64**> |  | [optional]
**input_asset** | Option<**String**> |  | [optional]
**input_asset_id** | Option<**f64**> | Fiat ID for buy transactions, asset ID otherwise | [optional]
**input_blockchain** | Option<**String**> |  | [optional]
**input_payment_method** | Option<**String**> |  | [optional]
**input_tx_id** | Option<**String**> |  | [optional]
**input_tx_url** | Option<**String**> |  | [optional]
**chargeback_target** | Option<**String**> | Chargeback address or chargeback IBAN | [optional]
**chargeback_amount** | Option<**f64**> | Chargeback amount in chargeback asset | [optional]
**chargeback_asset** | Option<**String**> |  | [optional]
**chargeback_asset_id** | Option<**f64**> | Fiat ID for sell transaction refunds, asset ID otherwise | [optional]
**chargeback_tx_id** | Option<**String**> |  | [optional]
**chargeback_tx_url** | Option<**String**> |  | [optional]
**chargeback_date** | Option<**String**> |  | [optional]
**date** | **String** |  | 
**reason** | Option<**String**> |  | [optional]
**exchange_rate** | Option<**f64**> | Exchange rate in input/output | [optional]
**rate** | Option<**f64**> | Final rate (incl. fees) in input/output | [optional]
**output_amount** | Option<**f64**> |  | [optional]
**output_asset** | Option<**String**> |  | [optional]
**output_asset_id** | Option<**f64**> | Fiat ID for sell transactions, asset ID otherwise | [optional]
**output_blockchain** | Option<**String**> |  | [optional]
**output_payment_method** | Option<**String**> |  | [optional]
**output_tx_id** | Option<**String**> |  | [optional]
**output_tx_url** | Option<**String**> |  | [optional]
**output_date** | Option<**String**> |  | [optional]
**price_steps** | Option<**Vec<String>**> |  | [optional]
**fee_amount** | Option<**f64**> | Fee amount in input asset | [optional]
**fee_asset** | Option<**String**> |  | [optional]
**fees** | Option<[**models::FeeDto**](FeeDto.md)> | Fee infos in input asset | [optional]
**external_transaction_id** | Option<**String**> |  | [optional]
**source_account** | Option<**String**> |  | [optional]
**target_account** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


