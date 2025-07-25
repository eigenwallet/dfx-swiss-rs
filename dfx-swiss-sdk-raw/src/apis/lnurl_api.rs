/*
 * DFX API
 *
 * DFX API PRD (updated on 7/3/2025, 12:42:13 PM)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`auth_lnurl_controller_get_lnurl_auth`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthLnurlControllerGetLnurlAuthError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_lnurl_controller_lnurl_auth_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthLnurlControllerLnurlAuthStatusError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_lnurl_controller_sign_in_with_lnurl_auth`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthLnurlControllerSignInWithLnurlAuthError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ln_url_p_forward_controller_cancel_payment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LnUrlPForwardControllerCancelPaymentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ln_url_p_forward_controller_ln_url_p_callback_forward`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LnUrlPForwardControllerLnUrlPCallbackForwardError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ln_url_p_forward_controller_ln_url_p_forward`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LnUrlPForwardControllerLnUrlPForwardError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ln_url_p_forward_controller_tx_hex_forward`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LnUrlPForwardControllerTxHexForwardError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ln_url_p_forward_controller_wait_for_payment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LnUrlPForwardControllerWaitForPaymentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ln_url_w_forward_controller_ln_url_w_callback_forward`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LnUrlWForwardControllerLnUrlWCallbackForwardError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ln_url_w_forward_controller_ln_url_w_forward`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LnUrlWForwardControllerLnUrlWForwardError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lnurld_forward_controller_lnurld_callback_forward`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LnurldForwardControllerLnurldCallbackForwardError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`lnurld_forward_controller_lnurld_forward`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LnurldForwardControllerLnurldForwardError {
    UnknownValue(serde_json::Value),
}


pub async fn auth_lnurl_controller_get_lnurl_auth(configuration: &configuration::Configuration, ) -> Result<models::AuthLnurlCreateLoginResponseDto, Error<AuthLnurlControllerGetLnurlAuthError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurla", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthLnurlControllerGetLnurlAuthError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn auth_lnurl_controller_lnurl_auth_status(configuration: &configuration::Configuration, k1: &str) -> Result<models::AuthLnurlStatusResponseDto, Error<AuthLnurlControllerLnurlAuthStatusError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurla/status", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("k1", &k1.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthLnurlControllerLnurlAuthStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn auth_lnurl_controller_sign_in_with_lnurl_auth(configuration: &configuration::Configuration, tag: &str, action: &str, k1: &str, sig: &str, key: &str, address: &str, signature: &str, used_ref: Option<&str>, wallet: Option<&str>, discount_code: Option<&str>, special_code: Option<&str>, moderator: Option<&str>) -> Result<models::AuthLnurlSignInResponseDto, Error<AuthLnurlControllerSignInWithLnurlAuthError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurla", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = used_ref {
        local_var_req_builder = local_var_req_builder.query(&[("usedRef", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wallet {
        local_var_req_builder = local_var_req_builder.query(&[("wallet", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = discount_code {
        local_var_req_builder = local_var_req_builder.query(&[("discountCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = special_code {
        local_var_req_builder = local_var_req_builder.query(&[("specialCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = moderator {
        local_var_req_builder = local_var_req_builder.query(&[("moderator", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("tag", &tag.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("action", &action.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("k1", &k1.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("sig", &sig.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("address", &address.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthLnurlControllerSignInWithLnurlAuthError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ln_url_p_forward_controller_cancel_payment(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<LnUrlPForwardControllerCancelPaymentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurlp/cancel/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LnUrlPForwardControllerCancelPaymentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ln_url_p_forward_controller_ln_url_p_callback_forward(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<LnUrlPForwardControllerLnUrlPCallbackForwardError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurlp/cb/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LnUrlPForwardControllerLnUrlPCallbackForwardError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ln_url_p_forward_controller_ln_url_p_forward(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<LnUrlPForwardControllerLnUrlPForwardError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurlp/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LnUrlPForwardControllerLnUrlPForwardError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ln_url_p_forward_controller_tx_hex_forward(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<LnUrlPForwardControllerTxHexForwardError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurlp/tx/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LnUrlPForwardControllerTxHexForwardError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ln_url_p_forward_controller_wait_for_payment(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<LnUrlPForwardControllerWaitForPaymentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurlp/wait/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LnUrlPForwardControllerWaitForPaymentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ln_url_w_forward_controller_ln_url_w_callback_forward(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<LnUrlWForwardControllerLnUrlWCallbackForwardError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurlw/cb/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LnUrlWForwardControllerLnUrlWCallbackForwardError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ln_url_w_forward_controller_ln_url_w_forward(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<LnUrlWForwardControllerLnUrlWForwardError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurlw/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LnUrlWForwardControllerLnUrlWForwardError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn lnurld_forward_controller_lnurld_callback_forward(configuration: &configuration::Configuration, id: &str, var: &str) -> Result<(), Error<LnurldForwardControllerLnurldCallbackForwardError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurld/cb/{id}/{var}", local_var_configuration.base_path, id=crate::apis::urlencode(id), var=crate::apis::urlencode(var));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LnurldForwardControllerLnurldCallbackForwardError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn lnurld_forward_controller_lnurld_forward(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<LnurldForwardControllerLnurldForwardError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lnurld/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<LnurldForwardControllerLnurldForwardError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

