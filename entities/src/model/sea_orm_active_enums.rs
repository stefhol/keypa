//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "history_action")]
pub enum HistoryAction {
    #[sea_orm(string_value = "add")]
    Add,
    #[sea_orm(string_value = "change")]
    Change,
    #[sea_orm(string_value = "create")]
    Create,
    #[sea_orm(string_value = "remove")]
    Remove,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "history_request_type"
)]
pub enum HistoryRequestType {
    #[sea_orm(string_value = "door")]
    Door,
    #[sea_orm(string_value = "keycard")]
    Keycard,
    #[sea_orm(string_value = "temp")]
    Temp,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "history_role")]
pub enum HistoryRole {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "leader")]
    Leader,
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "worker")]
    Worker,
}
