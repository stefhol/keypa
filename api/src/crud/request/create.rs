use std::collections::HashSet;

use crate::crud;
use crate::crud::history::create_door_to_request_history;
use crate::crud::log::{create_log_message, ASSIGN_DEPARTMENT, ASSIGN_DOOR};
use crate::util::mail::{send_mail, Email};
use crate::util::{error::CrudError, middleware::SecurityLevel};
use chrono::{DateTime, Utc};
use entities::model::sea_orm_active_enums::HistoryAction::Add;
use entities::model::{
    tbl_door_to_request, tbl_keycard, tbl_request, tbl_request_department, tbl_user,
};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, IntoActiveModel,
    Set, Statement,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub struct CreateRequest {
    pub active_until: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub create_keycard: bool,
    pub departments: Option<Vec<Uuid>>,
    pub other_rooms: Option<String>,
    pub rooms: Option<Vec<Uuid>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
struct QueryResult {
    pub room_id: Uuid,
    pub door_id: Uuid,
}
async fn query(db: &DatabaseConnection) -> Result<Vec<QueryResult>, CrudError> {
    let query_result: Vec<QueryResult> = QueryResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        select tbl_room.room_id, door_id from tbl_room join tbl_door on tbl_room.room_id = tbl_door.room_id
        "#,
        vec![],
    ))
    .all(db)
    .await?;
    Ok(query_result)
}

pub async fn create_request(
    db: &DatabaseConnection,
    user_id: &Uuid,
    request: &CreateRequest,
    sercurity_level: SecurityLevel,
) -> Result<(), CrudError> {
    // if department or other_rooms is something than it is a tempcard
    let is_temp_card =
        (request.departments.is_some() || request.other_rooms.is_some() || request.rooms.is_some())
            && request.create_keycard;
    // non user can assign rooms
    let is_allowed_to_have_rooms =
        match sercurity_level.has_high_enough_security_level(SecurityLevel::Worker) {
            Ok(_) => true,
            _ => false,
        };
    // default request we skip here the proposal request. it is the default case
    let mut db_request =
        create_default_request(db, user_id, request, is_temp_card, &sercurity_level).await?;
    create_log_message(
        user_id,
        &format!(
            "{}: {} \n {}",
            crud::log::CREATE_REQUEST,
            db_request.request_id.to_string(),
            serde_json::to_string_pretty(&db_request).unwrap()
        ),
    )
    .insert(db)
    .await?;

    // add keycard and update request with keycard info
    if request.create_keycard {
        let keycard = tbl_keycard::ActiveModel {
            request_id: Set(Some(db_request.request_id)),
            user_id: Set(user_id.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;
        create_log_message(
            user_id,
            &format!(
                "{}: keycard {} in request {}",
                crud::log::CREATE_KEYCARD,
                keycard.keycard_id.to_string(),
                db_request.request_id.to_string()
            ),
        )
        .insert(db)
        .await?;
        let mut model = db_request.into_active_model();
        model.keycard_id = Set(Some(keycard.keycard_id.to_owned()));
        db_request = model.update(db).await?;
    }
    // add rooms and doors
    if let Some(departments) = &request.departments {
        let db_request = &db_request;
        let departments: Vec<_> = departments
            .iter()
            .map(|department| tbl_request_department::ActiveModel {
                department_id: Set(department.to_owned()),
                request_id: Set(db_request.request_id.to_owned()),
            })
            .collect();
        for department in departments {
            let res = department.insert(db).await?;
            create_log_message(
                user_id,
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
    if is_allowed_to_have_rooms {
        //  here we are workers
        if let Some(rooms) = &request.rooms {
            let all_doors = query(db).await?;
            let doors: HashSet<_> = rooms
                .iter()
                .map(|room| {
                    all_doors
                        .iter()
                        .filter(|f| &f.room_id == room)
                        .map(|f| f.door_id)
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            let doors: Vec<_> = doors
                .iter()
                .map(|door| tbl_door_to_request::ActiveModel {
                    door_id: Set(door.to_owned()),
                    request_id: Set(db_request.request_id.to_owned()),
                })
                .collect();
            for door in doors {
                let res = door.insert(db).await?;
                let history = create_door_to_request_history(
                    db,
                    &Add,
                    user_id,
                    &res.door_id,
                    &res.request_id,
                )
                .await?;
                let mut log = create_log_message(
                    user_id,
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
    let user = tbl_user::Entity::find_by_id(user_id.to_owned())
        .one(db)
        .await?;
    if let Some(user) = user {
        send_mail(
            Email {
                email_to: user.email.to_string(),
                message: format!("Your Request has been created"),
                subject: format!("{}", "Create Request"),
            },
        )?;
    }
    Ok(())
}
async fn create_default_request(
    db: &DatabaseConnection,
    user_id: &Uuid,
    request: &CreateRequest,
    is_temp_card: bool,
    security_level: &SecurityLevel,
) -> Result<tbl_request::Model, CrudError> {
    let mut is_leader = false;
    if security_level == &SecurityLevel::Leader {
        is_leader = true;
    }
    let model = tbl_request::ActiveModel {
        requester_id: Set(user_id.to_owned()),
        active_until: Set(request.active_until.map(|f| f.naive_utc())),
        description: Set(request.description.to_owned()),
        active: Set(true),
        accept: Set(is_leader),
        reject: Set(false),
        pending: Set(!is_leader),
        keycard_id: Set(None),
        additional_rooms: Set(request.other_rooms.to_owned()),
        payed: match is_temp_card {
            // if it is temp card you need to pay but the payment is not finished
            true => Set(Some(false)),
            // if it is not a temp card you don't need to pay
            false => Set(None),
        },
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(model)
}
