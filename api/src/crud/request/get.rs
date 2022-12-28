use crate::crud;
use crate::util::error::CrudError;
use chrono::{DateTime, Utc};
use entities::model::{tbl_request, tbl_request_comment};
use sea_orm::{
    prelude::DateTimeUtc, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::crud::user::GetUser;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct GetRequestWithComments {
    pub request_id: Uuid,
    pub requester_id: Uuid,
    pub requester: Option<GetUser>,
    pub created_at: DateTimeUtc,
    pub changed_at: DateTimeUtc,
    pub description: Option<String>,
    pub accept: bool,
    pub reject: bool,
    pub pending: bool,
    pub comments: Vec<GetComments>,
    pub active_until: Option<DateTimeUtc>,
    pub is_proposal: bool,
    pub active: bool,
    pub keycard_id: Option<Uuid>,
}
impl From<(&tbl_request::Model, &Vec<GetComments>)> for GetRequestWithComments {
    fn from((request, comments): (&tbl_request::Model, &Vec<GetComments>)) -> Self {
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
            comments: comments.clone(),
            active_until: request
                .active_until
                .map(|active_until| DateTime::from_local(active_until.clone(), Utc)),
            is_proposal: request.is_proposal,
            active: request.active,
            keycard_id: request.keycard_id,
        }
    }
}
impl From<&tbl_request::Model> for GetRequestWithComments {
    fn from(request: &tbl_request::Model) -> Self {
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
            is_proposal: request.is_proposal,
            active: request.active,
            keycard_id: request.keycard_id,
        }
    }
}
impl From<(&tbl_request::Model, &Vec<GetUser>)> for GetRequestWithComments {
    fn from((request, user): (&tbl_request::Model, &Vec<GetUser>)) -> Self {
        let user = user
            .iter()
            .find(|f| f.user_id == request.requester_id)
            .cloned();
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
            is_proposal: request.is_proposal,
            active: request.active,
            keycard_id: request.keycard_id,
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
    pub written_at: DateTimeUtc,
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

pub async fn get_request_from_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetRequestWithComments>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::RequesterId.eq(user_id.clone()))
        .all(db)
        .await?;

    Ok(model
        .iter()
        .map(|f| GetRequestWithComments::from(f))
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
pub async fn get_all_open_requests(
    db: &DatabaseConnection,
) -> Result<Vec<GetRequestWithComments>, CrudError> {
    let model = tbl_request::Entity::find()
        .filter(tbl_request::Column::Pending.eq(true))
        .all(db)
        .await?;
    let user_vec = crud::user::get_all_user(db).await?;
    Ok(model
        .iter()
        .map(|f| GetRequestWithComments::from((f, &user_vec)))
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
            let mut request = GetRequestWithComments::from((request, &user_vec));
            request.comments = comments.iter().map(|f| (f, &user_vec).into()).collect();
            Ok(request)
        }
        None => Err(CrudError::NotFound),
    }
}
