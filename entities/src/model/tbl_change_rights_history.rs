//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use super::sea_orm_active_enums::HistoryAction;
use super::sea_orm_active_enums::HistoryRole;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_change_rights_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub change_rights_history_id: i64,
    pub action: HistoryAction,
    pub internal_role: HistoryRole,
    pub target_user_id: Uuid,
    pub changed_by: Uuid,
    pub changed_at: DateTime,
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
    TblUser2,
    #[sea_orm(
        belongs_to = "super::tbl_user::Entity",
        from = "Column::TargetUserId",
        to = "super::tbl_user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblUser1,
}

impl ActiveModelBehavior for ActiveModel {}
