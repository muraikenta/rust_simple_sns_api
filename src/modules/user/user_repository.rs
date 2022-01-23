use super::user_dto::NewUser;
use super::user_entity::User;
use crate::db::get_conn;
use crate::error::DbError;
use diesel::prelude::*;

pub fn save(item: &NewUser) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
    let conn = get_conn()?;
    diesel::insert_into(users).values(item).execute(&conn)?;
    Ok(users.order(id.desc()).first(&conn)?)
}
