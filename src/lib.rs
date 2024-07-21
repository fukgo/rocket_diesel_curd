use bcrypt::{hash, verify, DEFAULT_COST};
pub mod models;
pub mod schema;
pub mod routers;
pub mod views;
use dotenvy::dotenv;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;
fn _hash(){
    let password = "supersecret";
    // 哈希密码
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    println!("Hashed password: {}", hashed_password);

    // 验证密码
    let password_matches = verify("supersecret", &hashed_password).unwrap();
    println!("Password matches: {}", password_matches);
}

use regex::Regex;
fn is_email_valid(email: &str) -> bool {
    // 简单的邮箱验证
    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    email_regex.is_match(email)

}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();//从根目录的.env文件中加载环境变量

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

