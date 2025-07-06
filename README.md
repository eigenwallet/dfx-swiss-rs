# DFX Swiss SDK

A Rust SDK for the DFX Swiss API with Monero wallet integration.

## Project Structure

### dfx-swiss-sdk-raw
- Generated OpenAPI client for the DFX Swiss API
- Direct 1:1 mapping to the DFX API [specification](https://api.dfx.swiss/swagger-json)

### dfx-swiss-sdk
- Higher-level wrapper around dfx-swiss-sdk-raw
- Integrates with [`monero-sys`](https://github.com/eigenwallet/wallet/tree/master/monero-sys) for message signing
- Handles JWT token management

## Authentication Flow

1. Create DfxClient with Monero wallet address and tokio channel
2. Client requests sign message from DFX API
3. Message sent through tokio channel to signing service
4. Monero wallet signs the message
5. Signature sent back through channel
6. Client authenticates with DFX API using signed message
7. JWT token stored for subsequent API calls