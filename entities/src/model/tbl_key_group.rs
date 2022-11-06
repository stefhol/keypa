//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_key_group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub key_group_id: Uuid,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tbl_request::Entity")]
    TblRequest,
    #[sea_orm(has_many = "super::tbl_key_group_key::Entity")]
    TblKeyGroupKey,
}

impl Related<super::tbl_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequest.def()
    }
}

impl Related<super::tbl_key_group_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblKeyGroupKey.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
