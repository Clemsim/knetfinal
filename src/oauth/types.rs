#![cfg(feature="ssr")]
use std::sync::Mutex;

use oauth2::{basic::BasicClient, CsrfToken, EndpointNotSet, EndpointSet, PkceCodeVerifier};
use serde::{Deserialize, Serialize};
pub type FullyconfiguredClient = BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet>;

#[derive(Deserialize, Debug, Serialize)]
pub struct Token_URL{
    pub code:String,
    pub state: String
}
#[derive(Debug)]
pub struct App_State{
    pub csrftok:CsrfToken,
    pub pkce:Mutex<Option<PkceCodeVerifier>>,
    pub oauth_client: FullyconfiguredClient,
    pub reqwest_client: reqwest::Client,
}