use crate::crud;
use crate::util::error::CrudError;
use entities::model::{tbl_leader, tbl_worker};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::user::GetUserSmall;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct GetSmallWorker {
    user_id: Uuid,
    pub boss: Option<GetUserSmall>,
}
impl From<&tbl_worker::Model> for GetSmallWorker {
    fn from(worker: &tbl_worker::Model) -> Self {
        Self {
            user_id: worker.user_id,
            boss: None,
        }
    }
}
pub async fn get_worker_by_user_id(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<GetSmallWorker, CrudError> {
    let worker = tbl_worker::Entity::find_by_id(user_id.clone())
        .one(db)
        .await?;
    if let Some(worker) = worker {
        return match worker.boss_user_id {
            Some(boss_id) => {
                let leader_user = get_leader_by_user_id(db, &boss_id).await?;
                Ok(GetSmallWorker {
                    boss: Some(leader_user),
                    ..(&worker).into()
                })
            }
            None => Ok((&worker).into()),
        };
    }
    Err(CrudError::NotFound)
}
pub async fn get_leader_by_user_id(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<GetUserSmall, CrudError> {
    let user = GetUserSmall::from(&crud::user::get_single_user(db, user_id).await?);
    let is_admin = crud::user::is_admin_by_user_id(user_id, db).await?;
    let is_worker = crud::worker::is_worker_by_user_id(user_id, db).await?;
    Ok(GetUserSmall {
        is_leader: Some(true),
        is_admin: Some(is_admin),
        is_worker: Some(is_worker),
        ..user
    })
}

pub async fn is_leader_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let leader = tbl_leader::Entity::find_by_id(user_id.clone())
        .one(db)
        .await?;
    return Ok(leader.is_some());
}
pub async fn is_worker_by_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let worker = tbl_worker::Entity::find_by_id(user_id.clone())
        .one(db)
        .await?;
    return Ok(worker.is_some());
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateWorker {
    pub boss_user_id: Option<Uuid>,
}
pub async fn create_worker_from_user_id(
    user_id: &Uuid,
    woker: &CreateWorker,
    db: &DatabaseConnection,
) -> Result<(), CrudError> {
    let mut boss_id = None;
    if let Some(id) = &woker.boss_user_id {
        boss_id = Some(id.clone());
    }
    tbl_worker::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(user_id.clone()),
        boss_user_id: sea_orm::ActiveValue::Set(boss_id),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(())
}
pub async fn update_worker_with_user_id(
    user_id: &Uuid,
    woker: &CreateWorker,
    db: &DatabaseConnection,
) -> Result<(), CrudError> {
    let worker = tbl_worker::Entity::find_by_id(user_id.clone())
        .one(db)
        .await?;
    return if let Some(worker) = worker {
        let mut boss_id = None;
        if let Some(id) = &woker.boss_user_id {
            boss_id = Some(id.clone());
        }
        let mut worker: tbl_worker::ActiveModel = worker.into();
        worker.boss_user_id = ActiveValue::Set(boss_id);
        worker.update(db).await?;
        Ok(())
    } else {
        Err(CrudError::NotFound)
    };
}
pub async fn delete_worker_with_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<(), CrudError> {
    let worker = tbl_worker::Entity::find_by_id(user_id.clone())
        .one(db)
        .await?;
    return if let Some(worker) = worker {
        worker.delete(db).await?;
        Ok(())
    } else {
        Err(CrudError::NotFound)
    };
}
