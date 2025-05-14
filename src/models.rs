#![cfg(feature="ssr")]
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub nom: String,
    pub balance: Option<i32>,
}

use crate::schema::users;
#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser<'a>{
    pub nom: &'a String
}

pub fn create_user(conn: &mut PgConnection, nom: &String){
    use crate::schema::users;

    let newuser = NewUser{nom};

    let _ = diesel::insert_into(users::table)
        .values(newuser)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user");
}