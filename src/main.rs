mod db;
mod schema;

use ureq::serde_json;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, RunQueryDsl};

#[derive(Serialize, Deserialize, Debug)]
struct Results<T> {
    results: Vec<T>
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

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::parking_stats)]
struct CreateParkingStats {
    id: i32,
    // datetime of data creation according to api
    source_time: Option<NaiveDateTime>,
    opened: bool,
    current_total_capacity: i16,
    current_short_capacity: i16,
    current_other_capacity: i16,
    current_total_occupied: i16,
    current_short_occupied: i16,
    current_other_occupied: i16,
}

impl CreateParkingStats {
    fn from(p_in: ParkingStatsIn) -> CreateParkingStats {

        let source_time =
            NaiveDateTime::parse_from_str(p_in.stand.as_str(), "%Y-%m-%d %H:%M:%S").ok();

        let opened = match p_in.parkeinrichtung.as_str() {
            "geoeffnet" => true,
            "geschlossen" => false,
            unexpected => {
                eprintln!("Unexpected value for status of garage {}: {}", p_in.id, unexpected);
                false
            }
        };

        CreateParkingStats {
            current_total_capacity: p_in.dyntotal,
            current_short_capacity: p_in.dynshort,
            current_other_capacity: p_in.dynother,
            current_total_occupied: p_in.dtotalo,
            current_short_occupied: p_in.dshorto,
            current_other_occupied: p_in.dothero,
            id: p_in.id.parse().expect("Parking garage id could not be parsed"),
            source_time,
            opened
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::parking_info)]
struct CreateParkingInfo {
    id: i32,
    name: String,
    parking_type: String,
    total_capacity: i16,
    short_capacity: i16,
    other_capacity: i16
}

impl CreateParkingInfo {
    fn from(p_in: ParkingInfoIn) -> CreateParkingInfo {
      CreateParkingInfo {
        id: p_in.id.parse().expect("Parking garage id could not be parsed"),
        name: p_in.name,
        parking_type: p_in.type_,
        total_capacity: p_in.capacity,
        short_capacity: p_in.short,
        other_capacity: p_in.other
      }
    }
}

fn collect_info(conn: & mut PgConnection) {
    use schema::parking_info::*;

    let request_info_url = "https://open-data.dortmund.de/api/explore/v2.1/catalog/datasets/parkhauser/records?select=id,name,type,capacity,short,other&limit=100";

    let res: Results<ParkingInfoIn> = serde_json::from_reader(
        ureq::get(request_info_url).call().unwrap().into_reader()
    ).unwrap();

    let out: Vec<CreateParkingInfo> = res.results.into_iter().map(|p_in| CreateParkingInfo::from(p_in)).collect();

    diesel::insert_into(table)
        .values(out)
        .on_conflict(id)
        .do_update()
        .set((
            name.eq(diesel::upsert::excluded(name)),
            parking_type.eq(diesel::upsert::excluded(parking_type)),
            updated_time.eq(diesel::upsert::excluded(updated_time))
        ))
        .execute(conn)
        .expect("Error while inserting parking infos");

}

fn collect_stats(conn: & mut PgConnection) {
    let request_stats_url = "https://open-data.dortmund.de/api/explore/v2.1/catalog/datasets/parkhauser/records?select=id,stand,parkeinrichtung,dyntotal,dynshort,dynother,dtotalo,dshorto,dothero,frei&limit=30";

    let res: Results<ParkingStatsIn> = serde_json::from_reader(
        ureq::get(request_stats_url).call().unwrap().into_reader()
    ).unwrap();

    let out: Vec<CreateParkingStats> = res.results.into_iter()
        .map(|p_in| CreateParkingStats::from(p_in))
        .filter(|p_out| p_out.source_time.is_some())
        .collect();

    diesel::insert_into(schema::parking_stats::table)
        .values(out)
        .on_conflict_do_nothing()
        .execute(conn)
        .expect("Error while inserting parking stats");
}

fn main() {
    let mut conn = db::establish_connection();

    collect_stats(& mut conn);
}
