pub use dfx_swiss_sdk_raw;

use serde_derive::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{timeout, Duration};
use thiserror::Error;
use std::sync::Arc;

#[derive(Debug, Error)]
pub enum DfxError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Channel error: {0}")]
    ChannelError(String),
    #[error("Raw API error: {0}")]
    RawApiError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignRequest {
    pub message: String,
    pub blockchains: Vec<dfx_swiss_sdk_raw::models::sign_message_dto::Blockchains>,
}

#[derive(Debug, Clone)]
pub struct DfxClient {
    pub address: String,
    pub raw_client: Arc<dfx_swiss_sdk_raw::apis::configuration::Configuration>,
    pub auth_sender: mpsc::Sender<(SignRequest, oneshot::Sender<String>)>,
    pub access_token: Option<String>,
}

impl DfxClient {
    pub fn new(
        address: String,
        base_url: Option<String>,
        auth_sender: mpsc::Sender<(SignRequest, oneshot::Sender<String>)>,
    ) -> Self {
        let mut config = dfx_swiss_sdk_raw::apis::configuration::Configuration::new();
        if let Some(url) = base_url {
            config.base_path = url;
        }
        
        Self {
            address,
            raw_client: Arc::new(config),
            auth_sender,
            access_token: None,
        }
    }

    pub async fn authenticate(&mut self) -> Result<(), DfxError> {
        let sign_message_response = dfx_swiss_sdk_raw::apis::auth_api::auth_controller_get_sign_message(
            &self.raw_client,
            &self.address,
        )
        .await
        .map_err(|e| DfxError::RawApiError(format!("{:?}", e)))?;

        let sign_request = SignRequest {
            message: sign_message_response.message,
            blockchains: sign_message_response.blockchains,
        };

        let (response_tx, response_rx) = oneshot::channel();
        
        self.auth_sender
            .send((sign_request, response_tx))
            .await
            .map_err(|e| DfxError::ChannelError(format!("Failed to send sign request: {}", e)))?;

        let signature = timeout(Duration::from_secs(20), response_rx)
            .await
            .map_err(|_| DfxError::ChannelError("Timeout waiting for signature".to_string()))?
            .map_err(|e| DfxError::ChannelError(format!("Failed to receive signature: {}", e)))?;

        let sign_up_dto = dfx_swiss_sdk_raw::models::SignUpDto::new(
            self.address.clone(),
            signature,
        );

        let auth_response = dfx_swiss_sdk_raw::apis::auth_api::auth_controller_authenticate(
            &self.raw_client,
            sign_up_dto,
        )
        .await
        .map_err(|e| DfxError::RawApiError(format!("{:?}", e)))?;

        self.access_token = Some(auth_response.access_token);
        
        if self.access_token.is_none() {
            return Err(DfxError::AuthenticationError("No access token received".to_string()));
        }

        Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        self.access_token.is_some()
    }

    pub async fn get_user(&self) -> Result<dfx_swiss_sdk_raw::models::UserV2Dto, DfxError> {
        let token = self.access_token.as_ref().ok_or_else(|| {
            DfxError::AuthenticationError("Not authenticated".to_string())
        })?;

        let mut config = (*self.raw_client).clone();
        config.bearer_access_token = Some(token.clone());

        let user = dfx_swiss_sdk_raw::apis::user_api::user_v2_controller_get_user(&config)
            .await
            .map_err(|e| DfxError::RawApiError(format!("{:?}", e)))?;

        Ok(user)
    }
}