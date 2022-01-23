use dotenv::dotenv;
use std::env;

pub fn load() {
    dotenv().ok();
}

pub fn get_database_url() -> String {
    return env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}
