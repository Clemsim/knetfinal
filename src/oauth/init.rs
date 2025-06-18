#![cfg(feature="ssr")]
use std::sync::Mutex;

use leptos::logging::log;

use crate::oauth::types::App_State;

pub async fn init_oauth()->App_State{
    use oauth2::{basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl};

    let client = BasicClient::new(ClientId::new("30_2w53177iozwg48ko8sokkk0sog04wwo48wosswcggk0kwks08o".into()))
        .set_client_secret(ClientSecret::new("zgha7u74vmog8808s8o404s0osoc8kk4ck8os48o048w0cwc".into()))
        .set_redirect_uri(RedirectUrl::new("http://127.0.0.1:3000/oauth/token".into()).unwrap())
        .set_auth_uri(AuthUrl::new("https://my.centrale-assos.fr/oauth/v2/auth".into()).unwrap())
        .set_token_uri(TokenUrl::new("https://my.centrale-assos.fr/oauth/v2/token".into()).unwrap());

    let (pkce_challenge, pkceverifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("scope_users".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let app_state = App_State{
        csrftok: csrf_token,
        pkce: Mutex::new(Some(pkceverifier)),
        client: client
    };
    println!("Browse to: {}", auth_url);
    return app_state
}
