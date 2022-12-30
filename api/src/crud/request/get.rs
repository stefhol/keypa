use crate::crud;
use crate::util::error::CrudError;
use chrono::{DateTime, Utc};
use entities::model::{tbl_request, tbl_request_comment};
use sea_orm::{
    prelude::DateTimeUtc, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult,
    QueryFilter, QueryOrder, Statement,
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
pub struct GetRequestWithComments {
    pub request_id: Uuid,
    pub requester_id: Uuid,
    pub requester: Option<GetUser>,
    pub created_at: DateTime<Utc>,
    pub changed_at: DateTime<Utc>,
    pub description: Option<String>,
    pub accept: bool,
    pub reject: bool,
    pub pending: bool,
    pub comments: Vec<GetComments>,
    pub active_until: Option<DateTime<Utc>>,
    pub active: bool,
    pub keycard_id: Option<Uuid>,
    pub request_type: RequestType,
}

impl From<(&tbl_request::Model, &Vec<Uuid>)> for GetRequestWithComments {
    fn from((request, requests_with_doors): (&tbl_request::Model, &Vec<Uuid>)) -> Self {
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
            requester: None,
            created_at: DateTime::from_local(request.created_at.clone(), Utc),
            changed_at: DateTime::from_local(request.changed_at.clone(), Utc),
            description: request.description.clone(),
            accept: request.accept.clone(),
            reject: request.reject.clone(),
            pending: request.pending.clone(),
            comments: vec![],
            active_until: request
                .active_until
                .map(|active_until| DateTime::from_local(active_until.clone(), Utc)),
            active: request.active,
            keycard_id: request.keycard_id,
            request_type,
        }
    }
}
impl From<(&tbl_request::Model, &Vec<GetUser>, &Vec<Uuid>)> for GetRequestWithComments {
    fn from(
        (request, user, requests_with_doors): (&tbl_request::Model, &Vec<GetUser>, &Vec<Uuid>),
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
            comments: vec![],
            active_until: request
                .active_until
                .map(|active_until| DateTime::from_local(active_until.clone(), Utc)),
            active: request.active,
            keycard_id: request.keycard_id,
            request_type,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct GetComments {
    pub comment_id: Uuid,
    pub request_id: Uuid,
    pub user_id: Uuid,
    pub user: Option<GetUser>,
    pub comment: String,
    pub written_at: DateTime<Utc>,
}
impl From<(&tbl_request_comment::Model, &Vec<GetUser>)> for GetComments {
    fn from((comment, user_map): (&tbl_request_comment::Model, &Vec<GetUser>)) -> Self {
        GetComments {
            comment_id: comment.comment_id.clone(),
            request_id: comment.request_id.clone(),
            user_id: comment.user_id.clone(),
            user: user_map
                .iter()
                .find(|f| &f.user_id == &comment.user_id)
                .cloned(),
            comment: comment.comment.clone(),
            written_at: DateTimeUtc::from_utc(comment.written_at.clone(), Utc),
        }
    }
}

pub async fn get_requests_id_with_departments_rooms(
    db: &DatabaseConnection,
) -> Result<Vec<Uuid>, CrudError> {
    #[derive(Debug, Clone, FromQueryResult)]
    struct Temp {
        request_id: Uuid,
    }
    let query_result: Vec<Temp> = Temp::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        "select distinct tbl_request_department.request_id from tbl_request_department
        left outer join tbl_door_to_request tdtr on tbl_request_department.request_id = tdtr.request_id;",
        vec![],
    ))
    .all(db)
    .await?;
    Ok(query_result.iter().map(|f| f.request_id).collect())
}
pub async fn get_request_from_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetRequestWithComments>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::RequesterId.eq(user_id.clone()))
        .all(db)
        .await?;
    let request_with_doors = get_requests_id_with_departments_rooms(db).await?;
    Ok(model
        .iter()
        .map(|f| GetRequestWithComments::from((f, &request_with_doors)))
        .collect())
}
pub async fn get_request_from_user_id_and_request_id(
    user_id: &Uuid,
    request_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetRequestWithComments, CrudError> {
    let request = get_single_request(db, request_id).await?;
    if &request.requester_id == user_id {
        return Ok(request);
    }
    Err(CrudError::NotFound)
}
pub async fn get_all_requests(
    db: &DatabaseConnection,
) -> Result<Vec<GetRequestWithComments>, CrudError> {
    let model = tbl_request::Entity::find().all(db).await?;
    let user_vec = crud::user::get_all_user(db).await?;
    let request_with_doors = get_requests_id_with_departments_rooms(db).await?;

    Ok(model
        .iter()
        .map(|f| GetRequestWithComments::from((f, &user_vec, &request_with_doors)))
        .collect())
}
pub async fn get_all_pending_requests(
    db: &DatabaseConnection,
) -> Result<Vec<GetRequestWithComments>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::Pending.eq(true))
        .all(db)
        .await?;
    let user_vec = crud::user::get_all_user(db).await?;
    let request_with_doors = get_requests_id_with_departments_rooms(db).await?;

    Ok(model
        .iter()
        .map(|f| GetRequestWithComments::from((f, &user_vec, &request_with_doors)))
        .collect())
}
pub async fn get_all_reject_requests(
    db: &DatabaseConnection,
) -> Result<Vec<GetRequestWithComments>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::Reject.eq(true))
        .all(db)
        .await?;
    let user_vec = crud::user::get_all_user(db).await?;
    let request_with_doors = get_requests_id_with_departments_rooms(db).await?;

    Ok(model
        .iter()
        .map(|f| GetRequestWithComments::from((f, &user_vec, &request_with_doors)))
        .collect())
}
pub async fn get_all_accepted_requests(
    db: &DatabaseConnection,
) -> Result<Vec<GetRequestWithComments>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::Accept.eq(true))
        .all(db)
        .await?;
    let user_vec = crud::user::get_all_user(db).await?;
    let request_with_doors = get_requests_id_with_departments_rooms(db).await?;

    Ok(model
        .iter()
        .map(|f| GetRequestWithComments::from((f, &user_vec, &request_with_doors)))
        .collect())
}
pub async fn get_single_request(
    db: &DatabaseConnection,
    request_id: &Uuid,
) -> Result<GetRequestWithComments, CrudError> {
    let model = tbl_request::Entity::find_by_id(request_id.clone())
        .one(db)
        .await?;
    match &model {
        Some(request) => {
            let comments = tbl_request_comment::Entity::find()
                .filter(tbl_request_comment::Column::RequestId.eq(request.request_id.clone()))
                .order_by_asc(tbl_request_comment::Column::WrittenAt)
                .all(db)
                .await?;
            let user_vec = crud::user::get_all_user(db).await?;
            let request_with_doors = get_requests_id_with_departments_rooms(db).await?;

            let mut request =
                GetRequestWithComments::from((request, &user_vec, &request_with_doors));
            request.comments = comments.iter().map(|f| (f, &user_vec).into()).collect();
            Ok(request)
        }
        None => Err(CrudError::NotFound),
    }
}
