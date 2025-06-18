#![cfg(feature="ssr")]
use actix_web::{web::{self, Query}, HttpResponse, Responder};
use oauth2::{AuthorizationCode, TokenResponse};

// Make sure your App_State and Token_URL types are public or imported correctly
use crate::oauth::types::{App_State, Token_URL};

// This is our new, native Actix handler
pub async fn oauth_callback(
    Query(res): Query<Token_URL>,
    data: web::Data<App_State>,
) -> impl Responder {
    // 1. Validate the CSRF token
    if data.csrftok.clone().into_secret() != res.state {
        eprintln!("CSRF token mismatch.");
        return HttpResponse::BadRequest().body("CSRF token mismatch.");
    }

    // 2. Create a new reqwest client
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build reqwest client");

    // 3. Retrieve the PKCE verifier from the app state
    let pkce_verifier = match data.pkce.lock() {
        Ok(mut pkce_lock) => {
            pkce_lock.take().ok_or_else(|| {
                eprintln!("PKCE verifier not found in state.");
            })
        },
        Err(_) => Err({
            eprintln!("Failed to lock PKCE mutex.");
        })
    };
    
    // Check if we got the verifier
    let pkce_verifier = match pkce_verifier {
        Ok(verifier) => verifier,
        Err(msg) => return HttpResponse::InternalServerError().body(msg),
    };

    // 4. Exchange the authorization code for an access token
    let token_result = data.client
        .exchange_code(AuthorizationCode::new(res.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await;

    match token_result {
        Ok(token_response) => {
            println!("Successfully exchanged code for token!");
            println!("Access Token: {}", token_response.access_token().secret());

            // TODO: Store the token in a secure session cookie
            // For now, we redirect the user to a dashboard or home page.
            
            HttpResponse::Found()
                .append_header(("Location", "/dashboard")) // Redirect to a logged-in page
                .finish()
        },
        Err(e) => {
            eprintln!("Failed to exchange authorization code for token: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to log in. Please try again.")
        }
    }
}
