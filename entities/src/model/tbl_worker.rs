//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_worker")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub worker_id: Uuid,
    pub user_id: Uuid,
    pub boss_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tbl_leader::Entity",
        from = "Column::BossId",
        to = "super::tbl_leader::Column::LeaderId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblLeader,
    #[sea_orm(
        belongs_to = "super::tbl_user::Entity",
        from = "Column::UserId",
        to = "super::tbl_user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblUser,
}

impl Related<super::tbl_leader::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblLeader.def()
    }
}

impl Related<super::tbl_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
