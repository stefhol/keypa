use entities::model::tbl_request_log;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

pub const CREATE_REQUEST: &'static str = "Create Request";
pub const DEACTIVATE_REQUEST: &'static str = "Deactivate Request";
pub const CHANGE_REQUEST: &'static str = "Change Request";
pub const CREATE_KEYCARD: &'static str = "Create Keycard";
pub const CHANGE_KEYCARD: &'static str = "Change Keycard";
pub const DEACTIVE_KEYCARD: &'static str = "Deactivate Keycard";
pub const ASSIGN_DEPARTMENT: &'static str = "Create Department";
pub const ASSIGN_DOOR: &'static str = "Create Door";
pub const REMOVE_ALL_DEPARTMENT: &'static str = "Remove all Departments";
pub const REMOVE_ALL_DOORS: &'static str = "Remove all Doors";

pub(crate) fn create_log_message(changed_by: &Uuid, message: &str) -> tbl_request_log::ActiveModel {
    tbl_request_log::ActiveModel {
        changed_by: Set(changed_by.to_owned()),
        message: Set(Some(message.to_owned())),

        ..Default::default()
    }
}
