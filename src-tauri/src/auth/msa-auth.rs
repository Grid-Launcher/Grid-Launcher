use minecraft_msa_auth::MinecraftAuthorizationFlow;
use oauth2::basic::BasicClient;
use oauth2::devicecode::StandardDeviceAuthorizationResponse;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, ClientId, DeviceAuthorizationUrl, Scope, TokenResponse, TokenUrl};
use reqwest::Client;

const DEVICE_CODE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
const MSA_AUTHORIZE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const MSA_TOKEN_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";


async fn get_msa_auth(client_id) -> Result<(), Box<dyn std::error::Error>> {
    let client = BasicClient::new(
        ClientId::new(client_id),
        None,
        AuthUrl::new(MSA_AUTHORIZE_URL.to_string())?,
        Some(TokenUrl::new(MSA_TOKEN_URL.to_string())?),
    )
    .set_device_authorization_url(DeviceAuthorizationUrl::new(DEVICE_CODE_URL.to_string())?);
}

