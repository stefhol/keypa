use super::{
    log::{create_log_message, CHANGE_KEYCARD, DEACTIVE_KEYCARD},
    request::get::RequestType,
};
use crate::crud;
use crate::util::error::CrudError;
use chrono::{DateTime, Utc};
use entities::model::{tbl_keycard, tbl_keycard_archive, tbl_request_log};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbBackend, EntityTrait, IntoActiveModel, Set, Statement,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GetKeycard {
    pub keycard_id: Uuid,
    pub user_id: Uuid,
    pub is_lost: bool,
    pub is_locked: bool,
    pub is_deactivated: bool,
    pub is_given_back: bool,
    pub request_id: Option<Uuid>,
    pub given_out: Option<DateTime<Utc>>,
    pub keycard_type: Option<RequestType>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChangeKeyboard {
    pub is_lost: Option<bool>,
    pub is_locked: Option<bool>,
    pub is_deactivated: Option<bool>,
    pub is_given_back: Option<bool>,
    pub is_given_out: Option<bool>,
}
impl From<&tbl_keycard::Model> for GetKeycard {
    fn from(model: &tbl_keycard::Model) -> Self {
        let keycard = model.clone();
        Self {
            keycard_id: keycard.keycard_id,
            is_lost: keycard.is_lost,
            is_locked: keycard.is_locked,
            is_deactivated: keycard.is_deactivated,
            is_given_back: keycard.is_given_back,
            request_id: keycard.request_id,
            user_id: keycard.user_id,
            given_out: keycard.given_out.map(|f| DateTime::from_utc(f, Utc)),
            keycard_type: None,
        }
    }
}
async fn get_keycard_user_id_query(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<Vec<GetKeycard>, CrudError> {
    let requests = crud::request::get::get_all_requests(db).await?;
    let values = tbl_keycard::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select (tk.*) from tbl_user
            join tbl_request tr on tbl_user.user_id = tr.requester_id
            join tbl_keycard tk on tr.keycard_id = tk.keycard_id
            where tbl_user.user_id = $1
            and tr.accept = true
            and tr.active = true
            "#,
            vec![user_id.clone().into()],
        ))
        .all(db)
        .await?;
    let mut keycards: Vec<GetKeycard> = values.iter().map(|f| f.into()).collect();
    keycards.iter_mut().for_each(|keycard| {
        keycard.keycard_type = requests
            .iter()
            .find(|reqeuest| Some(reqeuest.request_id.to_owned()) == keycard.request_id.to_owned())
            .map(|f| f.request_type.to_owned());
    });
    Ok(keycards)
}
async fn get_single_keycard_keycard_id_query(
    db: &DatabaseConnection,
    keycard_id: &Uuid,
) -> Result<Option<GetKeycard>, CrudError> {
    let keycard = tbl_keycard::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            select (tk.*) from tbl_request as tr
            join tbl_keycard tk on tr.keycard_id = tk.keycard_id
            where tr.accept = true
            and tr.active = true
            and tr.keycard_id = $1;
            "#,
            vec![keycard_id.clone().into()],
        ))
        .one(db)
        .await?;
    let mut keycard: Option<GetKeycard> = keycard.map(|f| (&f).into());
    if let Some(card) = &keycard {
        if let Some(request_id) = &card.request_id {
            let request = crud::request::get::get_single_request(db, request_id).await?;
            keycard = keycard.map(|keycard| {
                let mut keycard = keycard.clone();
                keycard.keycard_type = Some(request.request_type);
                keycard
            })
        }
    }
    Ok(keycard)
}

pub async fn get_keycards_from_user(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<Vec<GetKeycard>, CrudError> {
    Ok(get_keycard_user_id_query(db, user_id).await?)
}
pub async fn get_single_keycard(
    db: &DatabaseConnection,
    keycard_id: &Uuid,
) -> Result<Option<GetKeycard>, CrudError> {
    Ok(get_single_keycard_keycard_id_query(db, keycard_id).await?)
}

pub async fn change_keycard(
    worker_id: &Uuid,
    db: &DatabaseConnection,
    keycard_id: &Uuid,
    change_keycard: &ChangeKeyboard,
) -> Result<(), CrudError> {
    let keycard_model = tbl_keycard::Entity::find_by_id(keycard_id.to_owned())
        .one(db)
        .await?;
    let mut log_vec: Vec<tbl_request_log::ActiveModel> = vec![];

    if let Some(keycard_model) = keycard_model {
        let mut keycard = keycard_model.clone().into_active_model();
        if let Some(is_deactivated) = change_keycard.is_deactivated {
            if is_deactivated == true {
                keycard.is_deactivated = Set(is_deactivated);
                log_vec.push(create_log_message(
                    worker_id,
                    &format!(
                        "{}: {} is_deactivated = true",
                        CHANGE_KEYCARD,
                        keycard_id.to_string()
                    ),
                ));
            }
        }
        if let Some(is_given_back) = change_keycard.is_given_back {
            if is_given_back {
                keycard.is_given_back = Set(is_given_back);
                log_vec.push(create_log_message(
                    worker_id,
                    &format!(
                        "{}: {} is_given_back = true",
                        CHANGE_KEYCARD,
                        keycard_id.to_string()
                    ),
                ));
                keycard.is_deactivated = Set(is_given_back);
                log_vec.push(create_log_message(
                    worker_id,
                    &format!(
                        "{}: {} is_deactivated = true",
                        CHANGE_KEYCARD,
                        keycard_id.to_string()
                    ),
                ));
            }
        }
        if let Some(is_given_out) = change_keycard.is_given_out {
            if is_given_out {
                keycard.given_out = Set(Some(Utc::now().naive_local()));
                log_vec.push(create_log_message(
                    worker_id,
                    &format!(
                        "{}: {} given_out = true",
                        CHANGE_KEYCARD,
                        keycard_id.to_string()
                    ),
                ));
            }
        }
        if let Some(is_locked) = change_keycard.is_locked {
            if is_locked == false {
                keycard.is_lost = Set(false);
                log_vec.push(create_log_message(
                    worker_id,
                    &format!(
                        "{}: {} is_lost = false",
                        CHANGE_KEYCARD,
                        keycard_id.to_string()
                    ),
                ));
            }
            keycard.is_locked = Set(is_locked);
            log_vec.push(create_log_message(
                worker_id,
                &format!(
                    "{}: {} is_locked = {}",
                    CHANGE_KEYCARD,
                    keycard_id.to_string(),
                    is_locked,
                ),
            ));
        }
        if let Some(is_lost) = change_keycard.is_lost {
            keycard.is_lost = Set(is_lost);
            log_vec.push(create_log_message(
                worker_id,
                &format!(
                    "{}: {} is_lost = {}",
                    CHANGE_KEYCARD,
                    keycard_id.to_string(),
                    is_lost,
                ),
            ));
            keycard.is_locked = Set(true);
            log_vec.push(create_log_message(
                worker_id,
                &format!(
                    "{}: {} is_locked = {}",
                    CHANGE_KEYCARD,
                    keycard_id.to_string(),
                    true,
                ),
            ));
        }
        let keycard_model = keycard.update(db).await?;
        tbl_request_log::Entity::insert_many(log_vec)
            .exec(db)
            .await?;
        if keycard_model.is_deactivated {
            move_to_archive(db, worker_id, &keycard_id).await?;
            if let Some(request_id) = &keycard_model.request_id {
                crud::request::change::move_to_archive(worker_id, db, request_id).await?;
            }
        }
    }

    Ok(())
}
pub async fn move_to_archive(
    db: &DatabaseConnection,
    worker_id: &Uuid,
    keycard_id: &Uuid,
) -> Result<(), CrudError> {
    let keycard_model = tbl_keycard::Entity::find_by_id(keycard_id.to_owned())
        .one(db)
        .await?;
    if let Some(keycard_model) = keycard_model {
        tbl_keycard_archive::ActiveModel {
            keycard_id: Set(keycard_model.keycard_id),
            user_id: Set(keycard_model.user_id),
            is_lost: Set(keycard_model.is_lost),
            is_locked: Set(keycard_model.is_locked),
            is_deactivated: Set(keycard_model.is_deactivated),
            is_given_back: Set(keycard_model.is_given_back),
            given_out: Set(keycard_model.given_out),
        }
        .insert(db)
        .await?;
        tbl_keycard::Entity::delete_by_id(keycard_model.keycard_id)
            .exec(db)
            .await?;
        create_log_message(
            worker_id,
            &format!("{}: {}", DEACTIVE_KEYCARD, keycard_id.to_string()),
        )
        .insert(db)
        .await?;
        tbl_keycard::Entity::delete_by_id(keycard_id.to_owned())
            .exec(db)
            .await?;
    }
    Ok(())
}
