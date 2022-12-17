//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_keycard")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub keycard_id: Uuid,
    pub is_lost: bool,
    pub is_locked: bool,
    pub is_deactivated: bool,
    pub is_given_back: bool,
    pub request_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tbl_request::Entity",
        from = "Column::RequestId",
        to = "super::tbl_request::Column::RequestId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblRequest,
    #[sea_orm(has_many = "super::tbl_request_history::Entity")]
    TblRequestHistory,
    #[sea_orm(has_many = "super::tbl_keycard_history::Entity")]
    TblKeycardHistory,
}

impl Related<super::tbl_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequest.def()
    }
}

impl Related<super::tbl_request_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequestHistory.def()
    }
}

impl Related<super::tbl_keycard_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblKeycardHistory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
