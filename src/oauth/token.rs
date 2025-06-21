#![cfg(feature="ssr")]
use actix_web::{web::{self, Query}, HttpResponse};
use leptos::{prelude::ServerFnError, server};
use leptos_actix::extract;
use oauth2::{AuthorizationCode, TokenResponse};

use crate::oauth::types::{App_State, Token_URL};

#[server(
    prefix="/oauth/",
    endpoint="token"
)]
pub async fn token_state() -> Result<(), ServerFnError> {
    // Extract the query parameters and application state
    let (Query(res), data): (Query<Token_URL>, web::Data<App_State>) = extract().await?;

    // Validate the CSRF token
    if data.csrftok.clone().into_secret() != res.state {
        eprintln!("CSRF token mismatch.");
        // It's better to return an error that can be handled by the client
        return Err(ServerFnError::new("CSRF token mismatch."));
    }

    // Create a new reqwest client
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build reqwest client");

    // Retrieve the PKCE verifier from the app state
    let pkce_verifier = {
        let mut pkce_lock = data.pkce.lock().unwrap();
        pkce_lock.take().ok_or_else(|| {
            eprintln!("PKCE verifier not found in state.");
            ServerFnError::new("PKCE verifier not found.")
        })?
    };

    // Exchange the authorization code for an access token
    let token_result = data.oauth_client
        .exchange_code(AuthorizationCode::new(res.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await;

    match token_result {
        Ok(token_response) => {
            // THE BIG MOMENT: Printing the access token to your server's terminal
            println!("Successfully exchanged code for token!");
            println!("Access Token: {}", token_response.access_token().secret());

            if let Some(refresh_token) = token_response.refresh_token() {
                println!("Refresh Token: {}", refresh_token.secret());
            }

            println!("Scopes: {:?}", token_response.scopes());

            // Here you would typically store the token in a session or a secure cookie
            // and redirect the user to a logged-in area of your site.
            // For now, we'll just return Ok to signify success.
            // In a real application, you might redirect like this:
            // let mut response = HttpResponse::Found();
            // response.append_header(("Location", "/dashboard"));
            // Ok(response) // This part would require more setup in your Leptos app.

            Ok(())
        },
        Err(e) => {
            // Log the detailed error on the server
            eprintln!("Failed to exchange authorization code for token: {:?}", e);
            
            // Return a generic error to the client
            Err(ServerFnError::new(format!("Failed to exchange code for token: {}", e)))
        }
    }
}