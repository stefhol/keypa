use chrono::{DateTime, Utc};
use entities::model::{tbl_door, tbl_door_to_request, tbl_request, tbl_request_department};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::{error::CrudError, middleware::SecurityLevel};

use super::get::{get_single_request, RequestType};

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub struct ChangeRequest {
    pub active_until: Option<DateTime<Utc>>,
    pub departments: Option<Vec<Uuid>>,
    pub rooms: Option<Vec<Uuid>>,
    pub accept: Option<bool>,
    pub reject: Option<bool>,
    pub pending: Option<bool>,
}

pub async fn change_request(
    db: &DatabaseConnection,
    request_id: &Uuid,
    request: &ChangeRequest,
    sercurity_level: SecurityLevel,
) -> Result<(), CrudError> {
    let og_request = tbl_request::Entity::find_by_id(request_id.to_owned())
        .one(db)
        .await?;

    if let Some(og_request) = og_request {
        let mut active_request = og_request.to_owned().into_active_model();
        let trans_og_request = get_single_request(db, request_id).await?;
        //check if sensitive if true then leader is required to change
        if let Some(true) = trans_og_request.is_sensitive {
            sercurity_level.has_high_enough_security_level(SecurityLevel::Leader)?;
        }
        match trans_og_request.request_type {
            RequestType::Keycard => {}
            RequestType::None => {}
            // room and temp are the same
            _ => {
                // delete all departments
                if let Some(_) = &trans_og_request.departments {
                    tbl_request_department::Entity::delete_many()
                        .filter(tbl_request_department::Column::RequestId.eq(request_id.to_owned()))
                        .exec(db)
                        .await?;
                }
                // insert new departments
                if let Some(departments) = &request.departments {
                    tbl_request_department::Entity::insert_many(departments.iter().map(|f| {
                        tbl_request_department::ActiveModel {
                            department_id: Set(f.to_owned()),
                            request_id: Set(request_id.to_owned()),
                        }
                    }))
                    .exec(db)
                    .await?;
                }

                if let Some(_) = &trans_og_request.doors {
                    tbl_door_to_request::Entity::delete_many()
                        .filter(tbl_door_to_request::Column::RequestId.eq(request_id.to_owned()))
                        .exec(db)
                        .await?;
                }

                if let Some(rooms) = &request.rooms {
                    //get all doors and compare them if they are in the room of the request
                    let db_doors = tbl_door::Entity::find().all(db).await?;
                    let doors: Vec<_> = db_doors
                        .iter()
                        .filter(|door| rooms.iter().any(|f| f == &door.room_id))
                        .collect();

                    tbl_door_to_request::Entity::insert_many(doors.iter().map(|f| {
                        tbl_door_to_request::ActiveModel {
                            door_id: Set(f.door_id.to_owned()),
                            request_id: Set(request_id.to_owned()),
                        }
                    }))
                    .exec(db)
                    .await?;
                }
            }
        };

        // set status of request
        //get status
        let status = vec![request.accept, request.reject, request.pending];
        // get a vec with all status filter it and check if only one status value is provided
        let status: Vec<_> = status
            .iter()
            .enumerate()
            .filter(|f| f.1.is_some())
            .map(|f| (f.0, f.1.unwrap()))
            .collect();
        if status.len() > 1 {
            return Err(CrudError::InvalidInput(
                "Only provide one Status boolean".to_owned(),
            ));
        }

        // overwrite status with optional value
        let status = status.iter().next();

        if let Some(status) = status {
            match status.0 {
                0 => {
                    active_request.accept = Set(true);
                    active_request.reject = Set(false);
                    active_request.pending = Set(false);
                }
                1 => {
                    active_request.accept = Set(false);
                    active_request.reject = Set(true);
                    active_request.pending = Set(false);
                }
                2 => {
                    active_request.accept = Set(false);
                    active_request.reject = Set(false);
                    active_request.pending = Set(true);
                }
                _ => {}
            }
        }
        // save new active until
        active_request.active_until = Set(request.active_until.map(|f| f.naive_utc()));
        // save new changed time
        active_request.changed_at = Set(Utc::now().naive_utc());
        active_request.update(db).await?;
    }
    Ok(())
}
