use entities::model::{
    tbl_door_to_request, tbl_keycard, tbl_request, tbl_request_department, tbl_request_entrance,
};
use sea_orm::{
    prelude::DateTimeUtc, ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::{error::CrudError, middleware::SecurityLevel};
#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub struct IndividualRooms {
    pub building_id: Uuid,
    pub rooms: String,
}
#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub struct CreateRequest {
    pub active_until: Option<DateTimeUtc>,
    pub description: Option<String>,
    pub create_keycard: bool,
    pub departments: Option<Vec<Uuid>>,
    pub indivdual_rooms: Option<Vec<IndividualRooms>>,
    pub doors: Option<Vec<Uuid>>,
}
pub async fn create_request(
    db: &DatabaseConnection,
    user_id: &Uuid,
    request: &CreateRequest,
    sercurity_level: SecurityLevel,
) -> Result<(), CrudError> {
    // if department or individual_rooms is something than it is a tempcard
    let is_temp_card = (request.departments.is_some() || request.indivdual_rooms.is_some())
        && request.create_keycard;
    // non user can skip proposal step
    let skip_proposal_step =
        match sercurity_level.has_high_enough_security_level(SecurityLevel::Worker) {
            Ok(_) => true,
            _ => false,
        };
    // default request we skip here the proposal request. it is the default case
    let mut db_request = create_default_request(db, user_id, request, false, is_temp_card).await?;
    // add keycard and update request with keycard info
    if request.create_keycard {
        let keycard = tbl_keycard::ActiveModel {
            request_id: Set(db_request.request_id),
            ..Default::default()
        }
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
        tbl_request_department::Entity::insert_many(departments)
            .exec(db)
            .await?;
    }

    // special case here, we split the request if it is not temp and is not from worker
    if !skip_proposal_step {
        if let Some(individual_rooms) = &request.indivdual_rooms {
            // we need a new request if it has departments and individual rooms
            //special case if it is a temp card than it can have both
            let request_to_insert = if request.departments.is_some() && !is_temp_card {
                create_default_request(db, user_id, request, true, is_temp_card).await?
            } else {
                let mut model = db_request.clone().into_active_model();
                model.is_proposal = Set(true);
                db_request = model.update(db).await?;
                db_request.clone()
            };
            let individual_rooms: Vec<_> = individual_rooms
                .iter()
                .map(|individual_room| tbl_request_entrance::ActiveModel {
                    request_id: Set(request_to_insert.request_id.to_owned()),
                    building_id: Set(individual_room.building_id.to_owned()),
                    proposed_rooms: Set(individual_room.rooms.to_owned()),
                    ..Default::default()
                })
                .collect();
            tbl_request_entrance::Entity::insert_many(individual_rooms)
                .exec(db)
                .await?;
        }
    } else {
        //  here we are workers and don't have to make a special case so we just put the raw rooms and doors in one request and forward it
        if let Some(doors) = &request.doors {
            tbl_door_to_request::Entity::insert_many(doors.iter().map(|door| {
                tbl_door_to_request::ActiveModel {
                    door_id: Set(door.to_owned()),
                    request_id: Set(db_request.request_id.to_owned()),
                }
            }))
            .exec(db)
            .await?;
        }
    }
    Ok(())
}
async fn create_default_request(
    db: &DatabaseConnection,
    user_id: &Uuid,
    request: &CreateRequest,
    is_proposal: bool,
    is_temp_card: bool,
) -> Result<tbl_request::Model, CrudError> {
    Ok(tbl_request::ActiveModel {
        requester_id: Set(user_id.to_owned()),
        active_until: Set(request.active_until.map(|f| f.naive_utc())),
        description: Set(request.description.to_owned()),
        is_proposal: Set(is_proposal),
        active: Set(true),
        accept: Set(false),
        reject: Set(false),
        pending: Set(true),
        keycard_id: Set(None),
        payed: match is_temp_card {
            // if it is temp card you need to pay but the payment is not finished
            true => Set(Some(false)),
            // if it is not a temp card you don't need to pay
            false => Set(None),
        },
        ..Default::default()
    }
    .insert(db)
    .await?)
}
