// Conditional use statements
#[cfg(feature="ssr")]
use diesel::prelude::*;
#[cfg(feature="ssr")]
use crate::schema; // Or crate::schema::users directly if that's what you mean for both
use serde::Deserialize;
use serde::Serialize;
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::users))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
#[derive(Debug, Clone, PartialEq)] // Example of derives you might want always (or make these conditional too)
#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub nom: String,
    pub balance: Option<i32>,
}

// Conditionally compiled NewUser struct
#[cfg(feature="ssr")]
#[derive(Insertable)]
#[cfg(feature="ssr")]
#[diesel(table_name = schema::users)] // Assuming schema::users is what you intend
#[cfg(feature="ssr")]
pub struct NewUser<'a> {
    pub nom: &'a String,
}
