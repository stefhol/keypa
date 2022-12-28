//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_request")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub request_id: Uuid,
    pub requester_id: Uuid,
    pub created_at: DateTime,
    pub changed_at: DateTime,
    pub active_until: Option<DateTime>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub is_proposal: bool,
    pub active: bool,
    pub accept: bool,
    pub reject: bool,
    pub payed: Option<bool>,
    pub pending: bool,
    pub keycard_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tbl_door_to_request_history::Entity")]
    TblDoorToRequestHistory,
    #[sea_orm(
        belongs_to = "super::tbl_keycard::Entity",
        from = "Column::KeycardId",
        to = "super::tbl_keycard::Column::KeycardId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblKeycard,
    #[sea_orm(has_many = "super::tbl_request_comment::Entity")]
    TblRequestComment,
    #[sea_orm(has_many = "super::tbl_request_entrance::Entity")]
    TblRequestEntrance,
    #[sea_orm(
        belongs_to = "super::tbl_user::Entity",
        from = "Column::RequesterId",
        to = "super::tbl_user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblUser,
}

impl Related<super::tbl_door_to_request_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblDoorToRequestHistory.def()
    }
}

impl Related<super::tbl_keycard::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblKeycard.def()
    }
}

impl Related<super::tbl_request_comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequestComment.def()
    }
}

impl Related<super::tbl_request_entrance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequestEntrance.def()
    }
}

impl Related<super::tbl_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblUser.def()
    }
}

impl Related<super::tbl_department::Entity> for Entity {
    fn to() -> RelationDef {
        super::tbl_request_department::Relation::TblDepartment.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::tbl_request_department::Relation::TblRequest
                .def()
                .rev(),
        )
    }
}

impl Related<super::tbl_door::Entity> for Entity {
    fn to() -> RelationDef {
        super::tbl_door_to_request::Relation::TblDoor.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::tbl_door_to_request::Relation::TblRequest.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
