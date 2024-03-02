use std::env;

use dotenvy::dotenv;
use futures::executor::block_on;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use ureq::serde_json;

mod db;
mod model;

#[derive(Serialize, Deserialize, Debug)]
struct Results<T> {
    results: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ParkingStatsIn {
    // garage id
    id: String,
    // date/time or error string
    stand: String,
    // garage status: geoffnet/geschlossen
    parkeinrichtung: String,
    /* current capacities */
    dyntotal: i16,
    dynshort: i16,
    dynother: i16,
    /* current occupancy */
    dtotalo: i16,
    dshorto: i16,
    dothero: i16,
    // available lots
    frei: i16,
}

#[derive(Serialize, Deserialize, Debug)]
struct ParkingInfoIn {
    id: String,
    name: String,
    #[serde(rename = "type")]
    type_: String,
    capacity: i16,
    short: i16,
    other: i16,
}

fn collect_info() -> Vec<ParkingInfoIn> {
    let request_info_url = "https://open-data.dortmund.de/api/explore/v2.1/catalog/datasets/parkhauser/records?select=id,name,type,capacity,short,other&limit=100";

    let res: Results<ParkingInfoIn> =
        serde_json::from_reader(ureq::get(request_info_url).call().unwrap().into_reader()).unwrap();

    res.results
}

fn collect_stats() -> Vec<ParkingStatsIn> {
    let request_stats_url = "https://open-data.dortmund.de/api/explore/v2.1/catalog/datasets/parkhauser/records?select=id,stand,parkeinrichtung,dyntotal,dynshort,dynother,dtotalo,dshorto,dothero,frei&limit=30";

    let res: Results<ParkingStatsIn> =
        serde_json::from_reader(ureq::get(request_stats_url).call().unwrap().into_reader())
            .unwrap();

    res.results
}

fn main() {
    dotenv().ok();
    let setup = env::args().any(|item| item.eq("--setup"));

    if setup {
        println!("Setting up databases...");

        let connection_url = env::var("CONNECTION_URL").expect("CONNECTION_URL must be set");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_name = env::var("DATABASE_NAME").unwrap_or("do_parking".to_string());

        let conn = block_on(db::establish_connection(connection_url.as_str()));
        block_on(db::create_db_if_not_exists(&conn, database_name.as_str()));
        block_on(conn.close()).expect("Closing connection failed.");

        let conn = block_on(db::establish_connection(database_url.as_str()));
        block_on(db::create_tables(&conn).then(|_| {
            let info = collect_info();
            db::insert_info(&conn, info)
        }));
        block_on(conn.close()).expect("Closing connection failed.");
    } else {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let conn = block_on(db::establish_connection(database_url.as_str()));
        block_on({
            let stats = collect_stats();
            db::insert_stats(&conn, stats)
        });
        block_on(conn.close()).expect("Closing connection failed.");
    }
}
