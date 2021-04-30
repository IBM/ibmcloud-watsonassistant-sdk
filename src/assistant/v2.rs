use std::future::Future;
use ibmcloud_core::authenticators::token_api::{AuthenticatorApiClient,  ResponseType, TokenResponse};
use anyhow::Result;
use reqwest::header::{HeaderMap, USER_AGENT, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Session {
    session_id: String,
}

pub type IamAuthenticator = AuthenticatorApiClient;

#[derive(Debug, Clone)]
pub struct AssistantClient {
    authenticator: IamAuthenticator,
    url: String,
    version: String,
    api_key: String,
    assistant_id: String
}

impl AssistantClient {
    pub fn new(iam: IamAuthenticator) -> AssistantClient {
        AssistantClient {
            authenticator: iam,
            url: "".to_string(),
            version: "".to_string(),
            api_key: "".to_string(),
            assistant_id: "".to_string(),

        }
    }
    pub fn set_service_url(&mut self, url: &str) {
        self.url = url.to_string();
    }
    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = api_key.to_string();
    }
    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_string();
    }
    pub fn set_assistant_id(&mut self, assistant_id: &str) {
        self.assistant_id = assistant_id.to_string();
    }
    pub fn get_url(&mut self)-> String{
        String::from(self.url.clone() + self.assistant_id.as_str() +"sessions?version=" + self.version.as_str() )
    }
    pub async fn create_session_async(&mut self) -> Session {
        let token = self.authenticator.get_token().await;
        let bearer = token.get_access_token();
        let result = get_session_id(bearer, self.get_url()).await;
        result.unwrap()
    }

}


async fn get_session_id(bearer: String, url: String )-> Result<Session>{
    let response:Session = reqwest::Client::new()
        .post(url)
        .headers(construct_headers(bearer))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
fn construct_headers(bearer: String)-> HeaderMap{
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    let value = String::from("Bearer ".to_owned() + bearer.as_str());
    headers.insert(AUTHORIZATION, HeaderValue::from_str(value.as_str()).unwrap());
    headers
}

//--------------------------------------------------------------------------------------------------
//--------------------------Unit Tests
//
// #[cfg(test)]
// mod tests {
//     use crate::assistant::v2::AssistantClient;
//
//     #[test]
//     fn new_assistant() {
//         let mut client: AssistantClient = AssistantClient::new();
//         let url = "";
//         let version = "";
//         let api_key = "";
//         client.set_service_url(url);
//         client.set_version(version);
//         client.set_api_key(api_key);
//
//         assert_eq!(client.url, url.to_string());
//         assert_eq!(client.api_key, api_key.to_string());
//         assert_eq!(client.version, version.to_string());
//     }
//
//     #[test]
//     fn new_sessesion() {}
// }
