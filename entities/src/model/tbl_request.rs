//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_request")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub request_id: Uuid,
    pub requester_id: Uuid,
    pub door_group_id: Uuid,
    pub created_at: DateTime,
    pub changed_at: DateTime,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub accept: bool,
    pub reject: bool,
    pub pending: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tbl_door_group::Entity",
        from = "Column::DoorGroupId",
        to = "super::tbl_door_group::Column::DoorGroupId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblDoorGroup,
    #[sea_orm(
        belongs_to = "super::tbl_user::Entity",
        from = "Column::RequesterId",
        to = "super::tbl_user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblUser,
    #[sea_orm(has_many = "super::tbl_request_comment::Entity")]
    TblRequestComment,
}

impl Related<super::tbl_door_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblDoorGroup.def()
    }
}

impl Related<super::tbl_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblUser.def()
    }
}

impl Related<super::tbl_request_comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequestComment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
