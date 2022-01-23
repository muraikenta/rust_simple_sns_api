use std::borrow::Borrow;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use r2d2::Error;

use crate::env::get_database_url;

pub static mut DB: Option<Pool<ConnectionManager<MysqlConnection>>> = None;

pub fn get_conn() -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, Error> {
    unsafe { DB.borrow().clone().unwrap().get() }
}

pub fn establish_connection() {
    let database_url = get_database_url();
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");
    unsafe { DB = Some(pool) }
}
