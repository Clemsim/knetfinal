use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

use crate::balance_component::Balance;
#[cfg(feature="ssr")]
use crate::{connection::establish_connection};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/knetfinal.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let posts = RwSignal::new(0);

    let on_click2 = {
        // Clone the signal for use in the async block
        let posts = posts.clone();
        move |_| {
            spawn_local(async move {
                if let Ok(num) = read_posts_number().await {
                    posts.set(num);  // Update the RwSignal with the value
                }
            });
        }
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        // <button on:click=move |_| {
        //     spawn_local(async {
        //         test().await.unwrap();
        //     })
        // }></button>

        <button on:click=on_click2>"Load Posts Count"</button>
        <p>"Number of posts: " {posts}</p>
        <Balance/>
    }

}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}


#[server]
pub async fn read_posts_number()->Result<usize, ServerFnError>{
    let mut conn = establish_connection();
    use crate::schema::users::dsl::*;
    use crate::models::User;
    use diesel::prelude::*;
    let results = users.select(User::as_select()).load(&mut conn).expect("");
    return Ok(results.len())
}