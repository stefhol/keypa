//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_room_department")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub department_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub room_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tbl_department::Entity",
        from = "Column::DepartmentId",
        to = "super::tbl_department::Column::DepartmentId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblDepartment,
    #[sea_orm(
        belongs_to = "super::tbl_room::Entity",
        from = "Column::RoomId",
        to = "super::tbl_room::Column::RoomId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblRoom,
}

impl Related<super::tbl_department::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblDepartment.def()
    }
}

impl Related<super::tbl_room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblRoom.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
