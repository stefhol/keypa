//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_door")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub door_id: Uuid,
    pub name: String,
    pub room_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tbl_door_to_request_history::Entity")]
    TblDoorToRequestHistory,
    #[sea_orm(has_many = "super::tbl_keycard_history::Entity")]
    TblKeycardHistory,
    #[sea_orm(
        belongs_to = "super::tbl_room::Entity",
        from = "Column::RoomId",
        to = "super::tbl_room::Column::RoomId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblRoom,
}

impl Related<super::tbl_door_to_request_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblDoorToRequestHistory.def()
    }
}

impl Related<super::tbl_keycard_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblKeycardHistory.def()
    }
}

impl Related<super::tbl_room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRoom.def()
    }
}

impl Related<super::tbl_request::Entity> for Entity {
    fn to() -> RelationDef {
        super::tbl_door_to_request::Relation::TblRequest.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::tbl_door_to_request::Relation::TblDoor.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
