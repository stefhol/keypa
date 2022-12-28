//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_keycard_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub keycard_history_id: i64,
    pub keycard_id: Uuid,
    pub door_id: Uuid,
    pub used_at: DateTime,
    pub success: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tbl_door::Entity",
        from = "Column::DoorId",
        to = "super::tbl_door::Column::DoorId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblDoor,
    #[sea_orm(
        belongs_to = "super::tbl_keycard::Entity",
        from = "Column::KeycardId",
        to = "super::tbl_keycard::Column::KeycardId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblKeycard,
    #[sea_orm(has_many = "super::tbl_request_log::Entity")]
    TblRequestLog,
}

impl Related<super::tbl_door::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblDoor.def()
    }
}

impl Related<super::tbl_keycard::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblKeycard.def()
    }
}

impl Related<super::tbl_request_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequestLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
