use chrono::{DateTime, Utc};
use entities::model::{
    tbl_door_to_request_history, tbl_keycard_usage_history, tbl_request_log, tbl_user,
};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::user::GetUser;

pub const CREATE_REQUEST: &'static str = "Create Request";
pub const DEACTIVATE_REQUEST: &'static str = "Deactivate Request";
pub const CHANGE_REQUEST: &'static str = "Change Request";
pub const CREATE_KEYCARD: &'static str = "Create Keycard";
pub const CHANGE_KEYCARD: &'static str = "Change Keycard";
pub const DEACTIVE_KEYCARD: &'static str = "Deactivate Keycard";
pub const ASSIGN_DEPARTMENT: &'static str = "Create Department";
pub const ASSIGN_DOOR: &'static str = "Create Door";
pub const REMOVE_ALL_DEPARTMENT: &'static str = "Remove all Departments";
pub const REMOVE_DOORS: &'static str = "Remove Door";

pub(crate) fn create_log_message(changed_by: &Uuid, message: &str) -> tbl_request_log::ActiveModel {
    tbl_request_log::ActiveModel {
        changed_by: Set(changed_by.to_owned()),
        message: Set(Some(message.to_owned())),

        ..Default::default()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetLogs {
    pub log_id: i64,
    pub message: Option<String>,
    pub keycard_history_id: Option<i64>,
    pub keycard_history: Option<tbl_keycard_usage_history::Model>,
    pub door_to_request_history_id: Option<i64>,
    pub door_to_request_history: Option<tbl_door_to_request_history::Model>,
    pub changed_at: DateTime<Utc>,
    pub changed_by_id: Uuid,
    pub changed_by: Option<GetUser>,
}
impl From<&tbl_request_log::Model> for GetLogs {
    fn from(value: &tbl_request_log::Model) -> Self {
        let value = value.to_owned();
        Self {
            log_id: value.log_id,
            message: value.message,
            keycard_history_id: value.keycard_history_id,
            door_to_request_history_id: value.door_to_request_history_id,
            changed_at: DateTime::from_utc(value.changed_at, Utc),
            changed_by_id: value.changed_by,
            keycard_history: None,
            door_to_request_history: None,
            changed_by: None,
        }
    }
}
impl GetLogs {
    fn add_addional(
        &mut self,
        door_to_request_history_vec: &Vec<tbl_door_to_request_history::Model>,
        keycard_history_vec: &Vec<tbl_keycard_usage_history::Model>,
        user_vec: &Vec<GetUser>,
    ) {
        self.door_to_request_history = door_to_request_history_vec
            .iter()
            .find(|f| {
                Some(f.door_to_request_history_id.to_owned())
                    == self.door_to_request_history_id.to_owned()
            })
            .map(|f| f.to_owned());
        self.keycard_history = keycard_history_vec
            .iter()
            .find(|f| Some(f.keycard_history_id.to_owned()) == self.keycard_history_id.to_owned())
            .map(|f| f.to_owned());

        if self.changed_by_id.is_nil() {
            self.changed_by = Some(GetUser{
                user_id: self.changed_by_id.to_owned(),
                name: "System".to_owned(),
                role_id: None,
                email: "".to_owned(),
                tel: None,
                address: None,
                picture_url: None,
            });
        }else{
            self.changed_by = user_vec
            .iter()
            .find(|f| Some(f.user_id.to_owned()) == Some(self.changed_by_id.to_owned()))
            .map(|f| f.to_owned());
        }
    }
}
pub async fn get_all_logs(db: &DatabaseConnection) -> Result<Vec<GetLogs>, CrudError> {
    let door_to_request_history_vec = tbl_door_to_request_history::Entity::find().all(db).await?;
    let keycard_history_vec = tbl_keycard_usage_history::Entity::find().all(db).await?;
    let user_vec: Vec<GetUser> = tbl_user::Entity::find()
        .all(db)
        .await?
        .iter()
        .map(|f| f.into())
        .collect();
    let mut request_log: Vec<GetLogs> = tbl_request_log::Entity::find()
        .all(db)
        .await?
        .iter()
        .map(|f| f.into())
        .collect();
    request_log.iter_mut().for_each(|f| {
        f.add_addional(
            &door_to_request_history_vec,
            &keycard_history_vec,
            &user_vec,
        )
    });
    request_log.sort_by(|a,b|a.changed_at.cmp(&b.changed_at));
    request_log.reverse();
    Ok(request_log)
}
pub async fn get_all_logs_raw(
    db: &DatabaseConnection,
) -> Result<Vec<tbl_request_log::Model>, CrudError> {
    let request_log = tbl_request_log::Entity::find().all(db).await?;
    Ok(request_log)
}
