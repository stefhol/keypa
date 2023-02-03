use chrono::{DateTime, Utc};
use entities::model::{tbl_request_comment, tbl_user};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::{error::CrudError, mail::{send_mail, Email}};

use super::{
    user::{get_all_user, GetUser},
};
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct GetComment {
    pub comment_id: Uuid,
    pub request_id: Uuid,
    pub user: Option<GetUser>,
    pub comment: String,
    pub written_at: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct InsertComment {
    pub comment: String,
}
impl From<(&tbl_request_comment::Model, &Vec<GetUser>)> for GetComment {
    fn from((value, user_vec): (&tbl_request_comment::Model, &Vec<GetUser>)) -> Self {
        let value = value.clone();
        Self {
            comment_id: value.comment_id,
            request_id: value.request_id,
            user: user_vec
                .iter()
                .cloned()
                .find(|f| f.user_id == value.user_id)
                .to_owned(),
            comment: value.comment.to_owned(),
            written_at: DateTime::from_utc(value.written_at, Utc),
        }
    }
}
/// get all comments from the given request id
pub async fn get_comments_of_request_id(
    db: &DatabaseConnection,
    request_id: &Uuid,
) -> Result<Vec<GetComment>, CrudError> {
    let user = get_all_user(db).await?;
    let model = tbl_request_comment::Entity::find()
        .filter(tbl_request_comment::Column::RequestId.eq(request_id.to_owned()))
        .all(db)
        .await?;
    Ok(model.iter().map(|f| GetComment::from((f, &user))).collect())
}
/// insert a comment into the request and send the email
pub async fn insert_comment_into_request_id(
    db: &DatabaseConnection,
    user_id: &Uuid,
    request_id: &Uuid,
    comment: &InsertComment,
) -> Result<(), CrudError> {
    let user = tbl_user::Entity::find_by_id(user_id.to_owned())
        .one(db)
        .await?;
    if let Some(user) = user {
        send_mail(
            Email {
                email_to: user.email.to_string(),
                message: format!("Nutzer {} hat in Antrag {} einen Kommentar verfasst", user.name, request_id),
                subject: format!("Neuer Kommentar in Antrag"),
            },
        )?;
    }
    tbl_request_comment::ActiveModel {
        comment: ActiveValue::Set(comment.comment.to_owned()),
        user_id: ActiveValue::Set(user_id.to_owned()),
        request_id: ActiveValue::Set(request_id.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(())
}
