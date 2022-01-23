use crate::{
    error::DbError,
    modules::user::{user_dto::NewUser, user_entity::User, user_repository},
};

pub fn signup(params: &NewUser) -> Result<(User, String), DbError> {
    let user = user_repository::save(&params)?;
    return Ok((user, String::from("token")));
}
