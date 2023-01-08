use chrono::{DateTime, Local, Utc};
use entities::model::{
    sea_orm_active_enums::HistoryAction::Add, tbl_door, tbl_door_to_request, tbl_request,
    tbl_request_archive, tbl_request_comment, tbl_request_department, tbl_request_log, tbl_user,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    crud::{
        self,
        email::{create_email, Email},
        history::create_door_to_request_history,
        log::{
            create_log_message, ASSIGN_DEPARTMENT, ASSIGN_DOOR, CHANGE_REQUEST, DEACTIVATE_REQUEST,
            REMOVE_ALL_DEPARTMENT, REMOVE_DOORS,
        },
    },
    util::{error::CrudError, middleware::SecurityLevel},
};

use super::get::{get_single_request, RequestType};

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub enum ChangeStatus {
    None = -1,
    Complete = 0,
    FurtherActionRequired = 1,
}
#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub struct ChangeRequest {
    pub active_until: Option<DateTime<Utc>>,
    pub departments: Option<Vec<Uuid>>,
    pub rooms: Option<Vec<Uuid>>,
    pub accept: Option<bool>,
    pub reject: Option<bool>,
    pub pending: Option<bool>,
    pub active: Option<bool>,
}
async fn has_request_sensitive_doors(
    db: &DatabaseConnection,
    request: &ChangeRequest,
) -> Result<bool, CrudError> {
    let sens_room_id = crud::room::get_rooms_id_sensitive(db).await?;
    if let Some(rooms) = &request.rooms {
        Ok(sens_room_id
            .iter()
            .any(|sens_room| rooms.contains(sens_room)))
    } else {
        Ok(false)
    }
}
async fn has_request_sensitive_departments(
    db: &DatabaseConnection,
    request: &ChangeRequest,
) -> Result<bool, CrudError> {
    let sens_department_id = crud::department::query_sensitive_departments(db).await?;
    if let Some(departments) = &request.departments {
        Ok(sens_department_id
            .iter()
            .any(|sens_department| departments.contains(sens_department)))
    } else {
        Ok(false)
    }
}
/// handle that the incoming changes makes the request sensitive, in that case the changes are allowed but then need approval of a leader beacause it is sensitive
pub async fn is_allowed_to_change_status(
    db: &DatabaseConnection,
    request: &ChangeRequest,
    sercurity_level: &SecurityLevel,
) -> Result<bool, CrudError> {
    if sercurity_level == &SecurityLevel::Worker {
        if has_request_sensitive_doors(db, request).await?
            || has_request_sensitive_departments(db, request).await?
        {
            return Ok(false);
        }
    }
    Ok(true)
}
pub async fn change_request(
    db: &DatabaseConnection,
    worker_id: &Uuid,
    request_id: &Uuid,
    request: &ChangeRequest,
    sercurity_level: SecurityLevel,
) -> Result<ChangeStatus, CrudError> {
    let mut change_status = ChangeStatus::FurtherActionRequired;
    let mut log_vec: Vec<tbl_request_log::ActiveModel> = vec![];
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

                    create_log_message(
                        worker_id,
                        &format!("{}: {}", REMOVE_ALL_DEPARTMENT, request_id.to_string()),
                    )
                    .insert(db)
                    .await?;
                }
                // insert new departments
                if let Some(departments) = &request.departments {
                    if departments.len() > 0 {
                        let departments: Vec<_> = departments
                            .iter()
                            .map(|department| tbl_request_department::ActiveModel {
                                department_id: Set(department.to_owned()),
                                request_id: Set(request_id.to_owned()),
                            })
                            .collect();
                        for department in departments {
                            let res = department.insert(db).await?;
                            create_log_message(
                                worker_id,
                                &format!(
                                    "{}: department {} to request {} ",
                                    ASSIGN_DEPARTMENT,
                                    res.department_id.to_string(),
                                    res.request_id.to_string()
                                ),
                            )
                            .insert(db)
                            .await?;
                        }
                    }
                }

                if let Some(_) = &trans_og_request.doors {
                    let to_remove_vec = tbl_door_to_request::Entity::find()
                        .filter(tbl_door_to_request::Column::RequestId.eq(request_id.to_owned()))
                        .all(db)
                        .await?;
                    tbl_door_to_request::Entity::delete_many()
                        .filter(tbl_door_to_request::Column::RequestId.eq(request_id.to_owned()))
                        .exec(db)
                        .await?;
                    for to_remove in &to_remove_vec {
                        let history = create_door_to_request_history(
                            db,
                            &entities::model::sea_orm_active_enums::HistoryAction::Remove,
                            worker_id,
                            &to_remove.door_id,
                            &to_remove.request_id,
                        )
                        .await?;
                        let mut log = create_log_message(
                            worker_id,
                            &format!("{}: {}", REMOVE_DOORS, request_id.to_string()),
                        );
                        log.door_to_request_history_id =
                            Set(Some(history.door_to_request_history_id.to_owned()));
                        log.insert(db).await?;
                    }
                }

                if let Some(rooms) = &request.rooms {
                    if rooms.len() > 0 {
                        //get all doors and compare them if they are in the room of the request
                        let db_doors = tbl_door::Entity::find().all(db).await?;
                        let doors: Vec<_> = db_doors
                            .iter()
                            .filter(|door| rooms.iter().any(|f| f == &door.room_id))
                            .map(|door| tbl_door_to_request::ActiveModel {
                                door_id: Set(door.door_id.to_owned()),
                                request_id: Set(request_id.to_owned()),
                            })
                            .collect();

                        for door in doors {
                            let res = door.insert(db).await?;
                            let history = create_door_to_request_history(
                                db,
                                &Add,
                                worker_id,
                                &res.door_id,
                                &res.request_id,
                            )
                            .await?;
                            let mut log = create_log_message(
                                worker_id,
                                &format!(
                                    "{}: door {} to request {} ",
                                    ASSIGN_DOOR,
                                    res.door_id.to_string(),
                                    res.request_id.to_string()
                                ),
                            );
                            log.door_to_request_history_id =
                                Set(Some(history.door_to_request_history_id.to_owned()));
                            log.insert(db).await?;
                        }
                    }
                }
            }
        };
        if is_allowed_to_change_status(db, request, &sercurity_level).await? {
            change_status = ChangeStatus::Complete;
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
                        active_request.active = Set(false);
                    }
                    _ => {}
                }
                let ac = active_request.clone();
                log_vec.push(create_log_message(
                    worker_id,
                    &format!(
                        "{}: {} accept = {}, reject = {}, pending {}",
                        CHANGE_REQUEST,
                        request_id.to_string(),
                        &ac.accept.unwrap(),
                        &ac.reject.unwrap(),
                        &ac.pending.unwrap(),
                    ),
                ));
            }
        }

        // save new active until
        active_request.active_until = Set(request.active_until.map(|f| f.naive_utc()));
        log_vec.push(create_log_message(
            worker_id,
            &format!(
                "{}: {} active_until = {}",
                CHANGE_REQUEST,
                request_id.to_string(),
                &request
                    .active_until
                    .map(|f| f.to_string())
                    .unwrap_or_default(),
            ),
        ));
        // save new changed time
        active_request.changed_at = Set(Local::now().naive_utc());

        log_vec.push(create_log_message(
            worker_id,
            &format!(
                "{}: {} changed_at = {}",
                CHANGE_REQUEST,
                request_id.to_string(),
                Local::now().naive_utc()
            ),
        ));

        let request_model = active_request.update(db).await?;
        if let Some(active_until) = request_model.active_until {
            if active_until < Local::now().naive_utc() {
                let mut active_request = request_model.clone().into_active_model();
                active_request.active = Set(false);
                active_request.update(db).await?;
            }
        }
        tbl_request_log::Entity::insert_many(log_vec)
            .exec(db)
            .await?;
        if request_model.reject || !request_model.active {
            move_to_archive(worker_id, db, request_id).await?;
            if let Some(keycard_id) = &request_model.keycard_id {
                crud::keycard::move_to_archive(db, worker_id, keycard_id).await?;
            }
        }
        let user = tbl_user::Entity::find_by_id(og_request.requester_id.to_owned())
            .one(db)
            .await?;
        if let Some(user) = user {
            create_email(
                &db,
                Email {
                    email_to: user.email.to_string(),
                    message: format!("A Request from you have been changed"),
                    subject: format!("{}", "Change Request"),
                },
            )
            .await?;
        }
    }
    Ok(change_status)
}

pub(crate) async fn move_to_archive(
    worker_id: &Uuid,
    db: &DatabaseConnection,
    request_id: &Uuid,
) -> Result<(), CrudError> {
    let request_model = tbl_request::Entity::find_by_id(request_id.to_owned())
        .one(db)
        .await?;

    let Some(request_model) = request_model else { return Ok(()) };
    let mut request_active = request_model.clone().into_active_model();
    request_active.active = Set(false);
    request_active.active_until = Set(Some(Local::now().naive_utc()));
    request_active.keycard_id = Set(None);
    tbl_request_department::Entity::delete_many()
        .filter(tbl_request_department::Column::RequestId.eq(request_id.to_owned()))
        .exec(db)
        .await?;
    tbl_door_to_request::Entity::delete_many()
        .filter(tbl_door_to_request::Column::RequestId.eq(request_id.to_owned()))
        .exec(db)
        .await?;
    tbl_request_comment::Entity::delete_many()
        .filter(tbl_request_comment::Column::RequestId.eq(request_id.to_owned()))
        .exec(db)
        .await?;
    let request_model = request_active.update(db).await?;
    let _ = tbl_request_archive::ActiveModel {
        request_id: Set(request_model.request_id),
        requester_id: Set(request_model.requester_id),
        created_at: Set(request_model.created_at),
        changed_at: Set(Local::now().naive_utc()),
        active_until: Set(request_model.active_until),
        description: Set(request_model.description),
        additional_rooms: Set(request_model.additional_rooms),
        active: Set(request_model.active),
        accept: Set(request_model.accept),
        reject: Set(request_model.reject),
        payed: Set(request_model.payed),
        pending: Set(request_model.pending),
    }
    .insert(db)
    .await;
    create_log_message(
        worker_id,
        &format!(
            "{}: {} moving to archive",
            DEACTIVATE_REQUEST,
            request_id.to_string()
        ),
    )
    .insert(db)
    .await?;
    let user = tbl_user::Entity::find_by_id(request_model.requester_id.to_owned())
        .one(db)
        .await?;
    if let Some(user) = user {
        create_email(
            &db,
            Email {
                email_to: user.email.to_string(),
                message: format!("A Request from you has been archived"),
                subject: format!("{}", "Archived Request"),
            },
        )
        .await?;
    }
    let _ = tbl_request::Entity::delete_by_id(request_id.to_owned())
        .exec(db)
        .await;
    Ok(())
}
