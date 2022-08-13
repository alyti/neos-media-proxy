use async_trait::async_trait;
use jellyfin_openapi_client::{
    apis::configuration, models::authenticate_user_by_name_request::AuthenticateUserByNameRequest,
};
use reqwest::header;

use crate::{LoginError, Session, SessionProvider};

struct JellyfinProvider;

#[async_trait]
impl SessionProvider for JellyfinProvider {
    fn name(&self) -> &'static str {
        "jellyfin"
    }

    async fn login(
        &self,
        hostname: &str,
        username: &str,
        password: &str,
    ) -> Result<Box<dyn Session>, LoginError> {
        let mut headers = header::HeaderMap::new(); 
        let app_name = format!(r#"MediaBrowser Client="{}", Device="{}", DeviceId="{}", Version="{}""#, "Neos Media Proxy", "Neos", "1234567890", "0.1");
        headers.insert("X-Emby-Authorization", header::HeaderValue::from_maybe_shared(app_name).unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
            
        let config = &configuration::Configuration {
            base_path: hostname.to_owned(),
            client: client,
            ..Default::default()
        };
        tracing::debug!("{:?}", config);

        let login_req = AuthenticateUserByNameRequest {
            username: Some(username.to_string()),
            password: None,
            pw: Some(password.to_string()),
        };
        let resp =
            jellyfin_openapi_client::apis::user_api::authenticate_user_by_name(config, login_req)
                .await;
        tracing::debug!("{:?}", resp);

        Ok(Box::new(JellyfinSession {}))
    }
}

struct JellyfinSession;

#[async_trait]
impl Session for JellyfinSession {
    async fn libraries(self: &Self) {
        tracing::debug!("seems to work");
    }
}

inventory::submit! {
    &JellyfinProvider as &dyn SessionProvider
}
