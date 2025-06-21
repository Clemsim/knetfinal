use diesel::prelude::*;

use crate::models::*;
pub fn create_user(conn: &mut PgConnection, nom1: &String){
    use crate::schema::users;

    let newuser = NewUser{
        nom:nom1,
    };

    let _ = diesel::insert_into(users::table)
        .values(newuser)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user");
}