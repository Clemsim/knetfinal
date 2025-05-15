#![cfg(feature="ssr")]
use diesel::prelude::*;

#[cfg(not(feature="ssr"))]
#[derive(Queryable, Selectable, serde::Serialize, serde::Deserialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub nom: String,
    pub balance: Option<i32>,
}

use crate::schema::users;
#[cfg(not(feature="ssr"))]
#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser<'a>{
    pub nom: &'a String
}