//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_request_comment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub comment_id: Uuid,
    pub request_id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub comment: String,
    pub written_at: DateTime,
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
    #[sea_orm(
        belongs_to = "super::tbl_user::Entity",
        from = "Column::UserId",
        to = "super::tbl_user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblUser,
}

impl Related<super::tbl_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequest.def()
    }
}

impl Related<super::tbl_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
