#[cfg(feature="ssr")]
use crate::connection::establish_connection;
#[cfg(feature="ssr")]
use crate::models::User;

use leptos::task::spawn_local;
use leptos::prelude::*;
#[server]
pub async fn read_user_and_balance()->Result<Vec<User>, ServerFnError>{
    let mut conn = establish_connection();
    use crate::schema::users::dsl::*;
    use crate::models::User;
    use diesel::prelude::*;
    let results: Vec<User> = users.select(User::as_select()).load(&mut conn).expect("");
    return Ok(results)
}

#[component]
fn Balance()->impl IntoView{
    let (users, set_users) = signal(Vec::<User>::new());

    spawn_local(async move{
        match read_user_and_balance().await{
            Ok(user_list) => set_users(user_list),
            Err(e) => panic!("Error fetching user: {:?}", e)
        }
    });

    view! {
        <div>
            <h2>"User Balances"</h2>
            <ul>
                {move || users.get().map(|list| {
                    view! {
                        <>
                            {for list.iter().map(|user| view! {
                                <li>{format!("{}: ${}", user.name, user.balance)}</li>
                            })}
                        </>
                    }
                })}
            </ul>
        </div>
    }

}