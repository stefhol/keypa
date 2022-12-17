//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_department")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub department_id: Uuid,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tbl_room_department::Entity")]
    TblRoomDepartment,
    #[sea_orm(has_many = "super::tbl_request_department::Entity")]
    TblRequestDepartment,
}

impl Related<super::tbl_room_department::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRoomDepartment.def()
    }
}

impl Related<super::tbl_request_department::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRequestDepartment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
