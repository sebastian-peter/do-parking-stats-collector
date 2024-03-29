//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "parking_stats")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub source_time: DateTime,
    pub opened: bool,
    pub updated_time: DateTime,
    pub current_total_capacity: i16,
    pub current_short_capacity: i16,
    pub current_other_capacity: i16,
    pub current_total_occupied: i16,
    pub current_short_occupied: i16,
    pub current_other_occupied: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::parking_info::Entity",
        from = "Column::Id",
        to = "super::parking_info::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ParkingInfo,
}

impl Related<super::parking_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ParkingInfo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
