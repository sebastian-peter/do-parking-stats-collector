use diesel::{PgConnection, sql_query};
use diesel::prelude::*;
use diesel::sql_types::Text;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|err| err.to_string())
        .and_then(|var| if var.is_empty() { Err(String::from("Env variable empty"))} else {Ok(var)})
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn setup_db(conn: &mut PgConnection) {
    create_db_if_not_exists(conn, "do_parking")
}

fn create_db_if_not_exists(conn: &mut PgConnection, db_name: &str) {
    let not_exists = sql_query("SELECT FROM pg_database WHERE datname = $1;")
        .bind::<Text, _>(db_name)
        .execute(conn).expect("Error while checking for database existence") == 0;

    if not_exists {
        sql_query(format!("CREATE DATABASE {};", db_name))
            .execute(conn).expect("Error while creating database");
    }
}
