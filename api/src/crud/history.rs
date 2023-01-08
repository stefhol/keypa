use entities::model::{sea_orm_active_enums::HistoryAction, tbl_door_to_request_history};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use uuid::Uuid;

use crate::util::error::CrudError;

pub(crate) async fn create_door_to_request_history(
    db: &DatabaseConnection,
    action: &HistoryAction,
    changed_by: &Uuid,
    door_id: &Uuid,
    request_id: &Uuid,
) -> Result<tbl_door_to_request_history::Model, CrudError> {
    Ok(tbl_door_to_request_history::ActiveModel {
        door_id: Set(door_id.to_owned()),
        request_id: Set(request_id.to_owned()),
        action: Set(action.to_owned()),
        changed_by: Set(changed_by.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await?)
}
