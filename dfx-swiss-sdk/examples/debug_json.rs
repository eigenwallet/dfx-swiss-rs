use dfx_swiss_sdk_raw::models::UserV2Dto;
use serde_json;

fn main() {
    let json_response = r#"{"accountId":324142,"accountType":null,"mail":null,"phone":null,"language":{"id":2,"name":"German","symbol":"DE","foreignName":"Deutsch","enable":true},"currency":{"id":2,"name":"EUR","buyable":true,"sellable":true,"cardBuyable":false,"cardSellable":true,"instantBuyable":false,"instantSellable":true},"tradingLimit":{"limit":1000,"period":"Month"},"kyc":{"hash":"2F70FD2A-575A-F011-8F7C-6045BD8F0635","level":0,"dataComplete":false},"volumes":{"buy":{"total":0,"annual":0},"sell":{"total":0,"annual":0},"swap":{"total":0,"annual":0}},"addresses":[{"wallet":"DFX Wallet","label":null,"address":"46TKjuKbTZ9cG1boZsv7jMTCjaQCqCNAUgbKwarRcetxNgnAnFDiu66aw6z1CEE5QD1gyRd3PcAiyiDCVRmXs1ByPzm1RQ3","blockchains":["Monero"],"volumes":{"buy":{"total":0,"annual":0},"sell":{"total":0,"annual":0},"swap":{"total":0,"annual":0}},"refCode":null,"apiKeyCT":null,"isCustody":false}],"disabledAddresses":[],"activeAddress":{"wallet":"DFX Wallet","label":null,"address":"46TKjuKbTZ9cG1boZsv7jMTCjaQCqCNAUgbKwarRcetxNgnAnFDiu66aw6z1CEE5QD1gyRd3PcAiyiDCVRmXs1ByPzm1RQ3","blockchains":["Monero"],"volumes":{"buy":{"total":0,"annual":0},"sell":{"total":0,"annual":0},"swap":{"total":0,"annual":0}},"refCode":null,"apiKeyCT":null,"isCustody":false},"paymentLink":{"active":false},"apiKeyCT":null}"#;

    println!("🔍 Attempting to deserialize UserV2Dto...");
    
    match serde_json::from_str::<UserV2Dto>(json_response) {
        Ok(user) => {
            println!("✅ Deserialization successful!");
            println!("Account ID: {}", user.account_id);
        }
        Err(e) => {
            println!("❌ Deserialization failed: {}", e);
        }
    }
}