use crate::crud::{self};
use crate::util::error::CrudError;
use chrono::{DateTime, Utc};
use entities::model::{tbl_door_to_request, tbl_request, tbl_request_department};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, QueryFilter,
    Statement,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::crud::user::GetUser;
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RequestType {
    Keycard,
    Temp,
    Room,
    None,
}
#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct GetRequest {
    pub request_id: Uuid,
    pub requester_id: Uuid,
    pub requester: Option<GetUser>,
    pub created_at: DateTime<Utc>,
    pub changed_at: DateTime<Utc>,
    pub description: Option<String>,
    pub accept: bool,
    pub reject: bool,
    pub pending: bool,
    pub active_until: Option<DateTime<Utc>>,
    pub active: bool,
    pub keycard_id: Option<Uuid>,
    pub request_type: RequestType,
    pub additional_rooms: Option<String>,
    pub departments: Option<Vec<Uuid>>,
    pub doors: Option<Vec<Uuid>>,
    pub is_sensitive: Option<bool>,
}

#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
struct QueryResult {
    request_id: Uuid,
    is_sensitive: bool,
}

async fn query_is_request_sensitive(
    db: &DatabaseConnection,
) -> Result<Vec<QueryResult>, CrudError> {
    let  query_result: Vec<QueryResult> = QueryResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        select distinct is_sensitive, sub.request_id from tbl_room
        join
        (
        select room_id, tbl_request.request_id from tbl_request
        join tbl_door_to_request on tbl_request.request_id = tbl_door_to_request.request_id
        join tbl_door td on tbl_door_to_request.door_id = td.door_id
        union
        select room_id, tbl_request.request_id from tbl_request
        join tbl_request_department on tbl_request.request_id = tbl_request_department.request_id
        join tbl_room_department on tbl_room_department.department_id = tbl_request_department.department_id
        )as sub on sub.room_id = tbl_room.room_id
        where is_sensitive = true
        "#,
        vec![],
    ))
    .all(db)
    .await?;
    Ok(query_result)
}

impl
    From<(
        &tbl_request::Model,
        &Vec<GetUser>,
        &Vec<Uuid>,
        &Vec<QueryResult>,
    )> for GetRequest
{
    fn from(
        (request, user, requests_with_doors, query_sens): (
            &tbl_request::Model,
            &Vec<GetUser>,
            &Vec<Uuid>,
            &Vec<QueryResult>,
        ),
    ) -> Self {
        let user = user
            .iter()
            .find(|f| f.user_id == request.requester_id)
            .cloned();
        let request_has_doors = requests_with_doors.contains(&request.request_id);
        let request_type = match request.keycard_id {
            Some(_) => match request_has_doors {
                true => RequestType::Temp,
                false => RequestType::Keycard,
            },
            None => match request_has_doors {
                true => RequestType::Room,
                false => RequestType::None,
            },
        };
        Self {
            request_id: request.request_id.clone(),
            requester_id: request.requester_id.clone(),
            requester: user,
            created_at: DateTime::from_local(request.created_at.clone(), Utc),
            changed_at: DateTime::from_local(request.changed_at.clone(), Utc),
            description: request.description.clone(),
            accept: request.accept.clone(),
            reject: request.reject.clone(),
            pending: request.pending.clone(),
            active_until: request
                .active_until
                .map(|active_until| DateTime::from_local(active_until.clone(), Utc)),
            active: request.active,
            keycard_id: request.keycard_id,
            request_type,
            departments: None,
            doors: None,
            additional_rooms: request.additional_rooms.to_owned(),
            is_sensitive: Some(
                query_sens
                    .iter()
                    .any(|f| f.request_id.to_owned() == request.request_id && f.is_sensitive),
            ),
        }
    }
}

pub async fn get_requests_id_with_departments_or_rooms(
    db: &DatabaseConnection,
) -> Result<Vec<Uuid>, CrudError> {
    #[derive(Debug, Clone, FromQueryResult)]
    struct Temp {
        request_id: Uuid,
    }
    let query_result: Vec<Temp> = Temp::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        "SELECT DISTINCT request_id FROM tbl_request_department
        UNION
        SELECT DISTINCT request_id FROM tbl_door_to_request",
        vec![],
    ))
    .all(db)
    .await?;
    Ok(query_result.iter().map(|f| f.request_id).collect())
}
pub async fn get_request_from_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetRequest>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::RequesterId.eq(user_id.clone()))
        .all(db)
        .await?;
    let requests_sens = query_is_request_sensitive(db).await?;
    let user_vec = vec![crud::user::get_single_user(user_id, db).await?];

    let request_with_doors = get_requests_id_with_departments_or_rooms(db).await?;
    Ok(model
        .iter()
        .map(|f| GetRequest::from((f, &user_vec, &request_with_doors, &requests_sens)))
        .collect())
}
pub async fn get_request_from_user_id_and_request_id(
    user_id: &Uuid,
    request_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetRequest, CrudError> {
    let request = get_single_request(db, request_id).await?;
    if &request.requester_id == user_id {
        return Ok(request);
    }
    Err(CrudError::NotFound)
}
pub async fn get_all_requests(db: &DatabaseConnection) -> Result<Vec<GetRequest>, CrudError> {
    let model = tbl_request::Entity::find().all(db).await?;
    let requests_sens = query_is_request_sensitive(db).await?;
    let user_vec = crud::user::get_all_user(db).await?;
    let request_with_doors = get_requests_id_with_departments_or_rooms(db).await?;

    Ok(model
        .iter()
        .map(|f| GetRequest::from((f, &user_vec, &request_with_doors, &requests_sens)))
        .collect())
}
pub async fn get_all_pending_requests(
    db: &DatabaseConnection,
) -> Result<Vec<GetRequest>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::Pending.eq(true))
        .all(db)
        .await?;
    let requests_sens = query_is_request_sensitive(db).await?;
    let user_vec = crud::user::get_all_user(db).await?;
    let request_with_doors = get_requests_id_with_departments_or_rooms(db).await?;

    Ok(model
        .iter()
        .map(|f| GetRequest::from((f, &user_vec, &request_with_doors, &requests_sens)))
        .collect())
}
pub async fn get_all_reject_requests(
    db: &DatabaseConnection,
) -> Result<Vec<GetRequest>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::Reject.eq(true))
        .all(db)
        .await?;
    let user_vec = crud::user::get_all_user(db).await?;
    let request_with_doors = get_requests_id_with_departments_or_rooms(db).await?;
    let requests_sens = query_is_request_sensitive(db).await?;

    Ok(model
        .iter()
        .map(|f| GetRequest::from((f, &user_vec, &request_with_doors, &requests_sens)))
        .collect())
}
pub async fn get_all_accepted_requests(
    db: &DatabaseConnection,
) -> Result<Vec<GetRequest>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::Accept.eq(true))
        .all(db)
        .await?;
    let user_vec = crud::user::get_all_user(db).await?;
    let request_with_doors = get_requests_id_with_departments_or_rooms(db).await?;
    let requests_sens = query_is_request_sensitive(db).await?;

    Ok(model
        .iter()
        .map(|f| GetRequest::from((f, &user_vec, &request_with_doors, &requests_sens)))
        .collect())
}
pub async fn get_single_request(
    db: &DatabaseConnection,
    request_id: &Uuid,
) -> Result<GetRequest, CrudError> {
    let model = tbl_request::Entity::find_by_id(request_id.clone())
        .one(db)
        .await?;
    match &model {
        Some(request) => {
            let user_vec = crud::user::get_all_user(db).await?;
            let request_with_doors = get_requests_id_with_departments_or_rooms(db).await?;
            let requests_sens = query_is_request_sensitive(db).await?;

            let mut request =
                GetRequest::from((request, &user_vec, &request_with_doors, &requests_sens));
            let request_department = tbl_request_department::Entity::find()
                .filter(tbl_request_department::Column::RequestId.eq(request.request_id))
                .all(db)
                .await?;
            let request_door = tbl_door_to_request::Entity::find()
                .filter(tbl_door_to_request::Column::RequestId.eq(request.request_id))
                .all(db)
                .await?;
            request.departments =
                Some(request_department.iter().map(|f| f.department_id).collect());
            request.doors = Some(request_door.iter().map(|f| f.door_id).collect());

            Ok(request)
        }
        None => Err(CrudError::NotFound),
    }
}
