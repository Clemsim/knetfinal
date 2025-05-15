use leptos::{prelude::*, task::spawn_local};

use crate::models::User;
use crate::balance::read_user_and_balance;
#[component]
pub fn Balance()->impl IntoView{
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
                <For
                    each = move || users()

                    key=|users| users.id

                    children=move |user|{
                        view! {
                            <div>{user.nom}</div>
                        }
                    }

                    />
            </ul>
        </div>
    }

}