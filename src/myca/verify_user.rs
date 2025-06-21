use diesel::prelude::*;
use crate::models::*;
use crate::oauth::types::App_State;
pub async fn create_user(conn: &mut PgConnection, nom1: &String){
    use crate::schema::users;
    use crate::schema::users::dsl::*;
    let client = todo!();
    let utilisateur = client.get("").headers("Machin", "acess token").send.await;
    let bool = users.filter(id).optional::<User>(&mut conn);
    if let(existe) Some(machin){
        creeate_user(utilisateur)
    }
}