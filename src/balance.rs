#[cfg(feature="ssr")]
use crate::connection::establish_connection;
use crate::models::User;

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