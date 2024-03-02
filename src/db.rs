use std::future::Future;

use chrono::NaiveDateTime;
use futures::FutureExt;
use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbErr, EntityName, EntityTrait, ExecResult, Set,
    Statement, StatementBuilder,
};
use sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Index, OnConflict, Table};

use crate::model::parking_info;
use crate::model::parking_stats;
use crate::{ParkingInfoIn, ParkingStatsIn};

pub async fn establish_connection(url: &str) -> DatabaseConnection {
    Database::connect(url)
        .await
        .expect(format!("Error connecting to {}", url).as_str())
}

pub async fn create_db_if_not_exists(conn: &DatabaseConnection, db_name: &str) {
    let not_exists = conn
        .execute(Statement::from_sql_and_values(
            conn.get_database_backend(),
            "SELECT FROM pg_database WHERE datname = $1;",
            [db_name.into()],
        ))
        .await
        .expect(&*format!(
            "Error while checking for existence of database {}",
            db_name
        ))
        .rows_affected()
        == 0;

    if not_exists {
        conn.execute_unprepared(&*format!("CREATE DATABASE {};", db_name))
            .await
            .expect(&*format!("Error while creating database {}", db_name));
    }
}

pub async fn create_tables(conn: &DatabaseConnection) {
    let create_info = Table::create()
        .table(parking_info::Entity.table_ref())
        .if_not_exists()
        .col(
            ColumnDef::new(parking_info::Column::Id)
                .integer()
                .primary_key()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_info::Column::Name)
                .char_len(100)
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_info::Column::ParkingType)
                .char_len(50)
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_info::Column::TotalCapacity)
                .small_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_info::Column::ShortCapacity)
                .small_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_info::Column::OtherCapacity)
                .small_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_info::Column::UpdatedTime)
                .timestamp()
                .default(Expr::current_timestamp())
                .not_null(),
        )
        .to_owned();

    let create_stats = Table::create()
        .table(parking_stats::Entity.table_ref())
        .if_not_exists()
        .col(
            ColumnDef::new(parking_stats::Column::Id)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_stats::Column::SourceTime)
                .timestamp()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_stats::Column::Opened)
                .boolean()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_stats::Column::UpdatedTime)
                .timestamp()
                .default(Expr::current_timestamp())
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_stats::Column::CurrentTotalCapacity)
                .small_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_stats::Column::CurrentShortCapacity)
                .small_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_stats::Column::CurrentOtherCapacity)
                .small_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_stats::Column::CurrentTotalOccupied)
                .small_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_stats::Column::CurrentShortOccupied)
                .small_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(parking_stats::Column::CurrentOtherOccupied)
                .small_integer()
                .not_null(),
        )
        .primary_key(
            Index::create()
                .col(parking_stats::Column::Id)
                .col(parking_stats::Column::SourceTime),
        )
        .foreign_key(
            ForeignKey::create()
                .from(parking_stats::Entity.table_ref(), parking_stats::Column::Id)
                .to(parking_info::Entity.table_ref(), parking_info::Column::Id)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned();

    execute(conn, &create_info)
        .then(|_| execute(conn, &create_stats))
        .await
        .expect("Creating tables failed");
}

fn execute<'a, S>(
    conn: &'a DatabaseConnection,
    query: &'a S,
) -> impl Future<Output = Result<ExecResult, DbErr>> + 'a
where
    S: StatementBuilder,
{
    conn.execute(conn.get_database_backend().build(query))
}

pub async fn insert_info(conn: &DatabaseConnection, values: Vec<ParkingInfoIn>) {
    let to_insert: Vec<parking_info::ActiveModel> = values.into_iter().map(convert_info).collect();

    parking_info::Entity::insert_many(to_insert)
        .on_conflict(
            OnConflict::column(parking_info::Column::Id)
                .update_columns([
                    parking_info::Column::Name,
                    parking_info::Column::ParkingType,
                    parking_info::Column::TotalCapacity,
                    parking_info::Column::ShortCapacity,
                    parking_info::Column::OtherCapacity,
                    parking_info::Column::UpdatedTime,
                ])
                .to_owned(),
        )
        .exec(conn)
        .await
        .expect("Error while trying to insert parking info.");
}

fn convert_info(p_in: ParkingInfoIn) -> parking_info::ActiveModel {
    parking_info::ActiveModel {
        id: Set(p_in
            .id
            .parse()
            .expect("Parking garage id could not be parsed")),
        name: Set(p_in.name),
        parking_type: Set(p_in.type_),
        total_capacity: Set(p_in.capacity),
        short_capacity: Set(p_in.short),
        other_capacity: Set(p_in.other),
        updated_time: Default::default(),
    }
}

pub async fn insert_stats(conn: &DatabaseConnection, values: Vec<ParkingStatsIn>) {
    let to_insert: Vec<parking_stats::ActiveModel> =
        values.into_iter().flat_map(convert_stats).collect();

    parking_stats::Entity::insert_many(to_insert)
        .on_conflict(
            OnConflict::columns([parking_stats::Column::Id, parking_stats::Column::SourceTime])
                .do_nothing()
                .to_owned(),
        )
        .do_nothing() // allow not inserting anything, panics with DbErr::RecordNotInserted otherwise
        .exec(conn)
        .await
        .expect("Error while trying to insert parking stats.");
}

fn convert_stats(p_in: ParkingStatsIn) -> Option<parking_stats::ActiveModel> {
    let source_time_opt =
        NaiveDateTime::parse_from_str(p_in.stand.as_str(), "%Y-%m-%d %H:%M:%S").ok();

    let opened = match p_in.parkeinrichtung.as_str() {
        "geoeffnet" => true,
        "geschlossen" => false,
        unexpected => {
            eprintln!(
                "Unexpected value for status of garage {}: {}",
                p_in.id, unexpected
            );
            false
        }
    };
    source_time_opt.map(|source_time| parking_stats::ActiveModel {
        id: Set(p_in
            .id
            .parse()
            .expect("Parking garage id could not be parsed")),
        source_time: Set(source_time),
        opened: Set(opened),
        current_total_capacity: Set(p_in.dyntotal),
        current_short_capacity: Set(p_in.dynshort),
        current_other_capacity: Set(p_in.dynother),
        current_total_occupied: Set(p_in.dtotalo),
        current_short_occupied: Set(p_in.dshorto),
        current_other_occupied: Set(p_in.dothero),
        updated_time: Default::default(),
    })
}
