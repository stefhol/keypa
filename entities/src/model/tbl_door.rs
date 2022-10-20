//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_door")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub door_id: Uuid,
    pub name: String,
    pub room_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tbl_room::Entity",
        from = "Column::RoomId",
        to = "super::tbl_room::Column::RoomId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblRoom,
    #[sea_orm(has_many = "super::tbl_key::Entity")]
    TblKey,
}

impl Related<super::tbl_room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRoom.def()
    }
}

impl Related<super::tbl_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblKey.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
