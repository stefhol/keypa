//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use super::sea_orm_active_enums::HistoryAction;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_door_to_request_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub door_to_request_history_id: i64,
    pub door_id: Uuid,
    pub request_id: Uuid,
    pub action: HistoryAction,
    pub changed_by: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tbl_user::Entity",
        from = "Column::ChangedBy",
        to = "super::tbl_user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblUser,
    #[sea_orm(
        belongs_to = "super::tbl_door::Entity",
        from = "Column::DoorId",
        to = "super::tbl_door::Column::DoorId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblDoor,
    #[sea_orm(
        belongs_to = "super::tbl_request::Entity",
        from = "Column::RequestId",
        to = "super::tbl_request::Column::RequestId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblRequest,
    #[sea_orm(has_many = "super::tbl_log::Entity")]
    TblLog,
}

impl Related<super::tbl_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblUser.def()
    }
}

impl Related<super::tbl_door::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblDoor.def()
    }
}

impl Related<super::tbl_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequest.def()
    }
}

impl Related<super::tbl_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}