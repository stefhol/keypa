use std::collections::HashMap;

use chrono::{DateTime, Utc};
use entities::model::{tbl_request, tbl_request_comment};
use sea_orm::{prelude::DateTimeUtc, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::user::GetUser;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct GetRequestWithComments {
    pub request_id: Uuid,
    pub requester_id: Uuid,
    pub door_group_id: Uuid,
    pub created_at: DateTimeUtc,
    pub changed_at: DateTimeUtc,
    pub description: Option<String>,
    pub accept: bool,
    pub reject: bool,
    pub pending: bool,
    pub comments: Vec<GetComments>,
}
impl From<(&tbl_request::Model, &Vec<GetComments>)> for GetRequestWithComments {
    fn from((request, comments): (&tbl_request::Model, &Vec<GetComments>)) -> Self {
        Self {
            request_id: request.request_id.clone(),
            requester_id: request.requester_id.clone(),
            door_group_id: request.door_group_id.clone(),
            created_at: DateTime::from_local(request.created_at.clone(), Utc),
            changed_at: DateTime::from_local(request.changed_at.clone(), Utc),
            description: request.description.clone(),
            accept: request.accept.clone(),
            reject: request.reject.clone(),
            pending: request.pending.clone(),
            comments: comments.clone(),
        }
    }
}
impl From<&tbl_request::Model> for GetRequestWithComments {
    fn from(request: &tbl_request::Model) -> Self {
        Self {
            request_id: request.request_id.clone(),
            requester_id: request.requester_id.clone(),
            door_group_id: request.door_group_id.clone(),
            created_at: DateTime::from_local(request.created_at.clone(), Utc),
            changed_at: DateTime::from_local(request.changed_at.clone(), Utc),
            description: request.description.clone(),
            accept: request.accept.clone(),
            reject: request.reject.clone(),
            pending: request.pending.clone(),
            comments: vec![],
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
impl From<(&tbl_request_comment::Model, &HashMap<Uuid, GetUser>)> for GetComments {
    fn from((comment, user_map): (&tbl_request_comment::Model, &HashMap<Uuid, GetUser>)) -> Self {
        GetComments {
            comment_id: comment.comment_id.clone(),
            request_id: comment.request_id.clone(),
            user_id: comment.user_id.clone(),
            user: user_map.get(&comment.user_id).cloned(),
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

    Ok(model.iter().map(|f| f.into()).collect())
}
pub async fn get_request_from_user_id_and_request_id(
    user_id: &Uuid,
    request_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetRequestWithComments, CrudError> {
    let model = tbl_request::Entity::find_by_id(request_id.clone())
        .filter(tbl_request::Column::RequesterId.eq(user_id.clone()))
        .one(db)
        .await?;
    if let Some(model) = model {
        let comments = tbl_request_comment::Entity::find()
            .filter(tbl_request_comment::Column::RequestId.eq(request_id.clone()))
            .all(db)
            .await?;
        let mut user_map: HashMap<Uuid, GetUser> = HashMap::new();
        //get all users
        for comment in &comments {
            if !user_map.contains_key(&comment.user_id) {
                let user = super::user::get_single_user(db, &comment.user_id).await?;
                user_map.insert(comment.user_id.clone(), user);
            }
        }
        let comments: Vec<GetComments> = comments.iter().map(|f| (f, &user_map).into()).collect();
        return Ok((&model, &comments).into());
    } else {
        return Err(CrudError::NotFound);
    }
}
