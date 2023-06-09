//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tbl_room")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub room_id: Uuid,
    pub name: String,
    pub floor: i32,
    pub is_sensitive: Option<bool>,
    pub building_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tbl_building::Entity",
        from = "Column::BuildingId",
        to = "super::tbl_building::Column::BuildingId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TblBuilding,
    #[sea_orm(has_many = "super::tbl_door::Entity")]
    TblDoor,
}

impl Related<super::tbl_building::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblBuilding.def()
    }
}

impl Related<super::tbl_door::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TblDoor.def()
    }
}

impl Related<super::tbl_department::Entity> for Entity {
    fn to() -> RelationDef {
        super::tbl_room_department::Relation::TblDepartment.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::tbl_room_department::Relation::TblRoom.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
