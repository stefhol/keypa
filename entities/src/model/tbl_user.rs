//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub name: String,
    pub role_id: Option<Uuid>,
    pub is_active: bool,
    pub email: String,
    pub picture_url: Option<String>,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tbl_role::Entity",
        from = "Column::RoleId",
        to = "super::tbl_role::Column::RoleId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblRole,
    #[sea_orm(has_many = "super::tbl_worker::Entity")]
    TblWorker,
    #[sea_orm(has_many = "super::tbl_leader::Entity")]
    TblLeader,
    #[sea_orm(has_many = "super::tbl_admin::Entity")]
    TblAdmin,
    #[sea_orm(has_many = "super::tbl_door_group::Entity")]
    TblDoorGroup,
    #[sea_orm(has_many = "super::tbl_keycard::Entity")]
    TblKeycard,
    #[sea_orm(has_many = "super::tbl_request::Entity")]
    TblRequest,
    #[sea_orm(has_many = "super::tbl_request_comment::Entity")]
    TblRequestComment,
    #[sea_orm(has_many = "super::tbl_door_user_access::Entity")]
    TblDoorUserAccess,
}

impl Related<super::tbl_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRole.def()
    }
}

impl Related<super::tbl_worker::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblWorker.def()
    }
}

impl Related<super::tbl_leader::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblLeader.def()
    }
}

impl Related<super::tbl_admin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblAdmin.def()
    }
}

impl Related<super::tbl_door_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblDoorGroup.def()
    }
}

impl Related<super::tbl_keycard::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblKeycard.def()
    }
}

impl Related<super::tbl_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequest.def()
    }
}

impl Related<super::tbl_request_comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequestComment.def()
    }
}

impl Related<super::tbl_door_user_access::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblDoorUserAccess.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
