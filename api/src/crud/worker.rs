use entities::model::{tbl_leader, tbl_worker};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::{
    role::GetRole,
    user::{get_single_user, GetUser},
};
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct GetWorker {
    pub user_id: Uuid,
    pub name: String,
    pub role: Option<GetRole>,
    pub email: String,
    #[serde(skip_serializing)]
    //only for internal use
    boss_id: Option<Uuid>,

    pub boss: Option<GetUser>,
}

pub async fn get_worker_by_user_id(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<GetWorker, CrudError> {
    let temp_worker = __get_worker_by_user_id(db, user_id).await?;
    if let Some(boss_id) = &temp_worker.boss_id {
        let boss_id = boss_id.clone();
        let boss_model = tbl_leader::Entity::find_by_id(boss_id).one(db).await?;
        let mut boss = None;
        if let Some(boss_model) = boss_model {
            let user_boss_model = __get_worker_by_user_id(db, &boss_model.user_id).await?;

            boss = Some(GetUser {
                user_id: user_boss_model.user_id.clone(),
                email: user_boss_model.email,
                name: user_boss_model.name,
                role: user_boss_model.role,
            });
        }
        return Ok(GetWorker {
            boss,
            ..temp_worker
        });
    }
    Ok(temp_worker)
}
/// Private function that gets a Worker without Boss struct
async fn __get_worker_by_user_id(
    db: &DatabaseConnection,
    user_id: &Uuid,
) -> Result<GetWorker, CrudError> {
    let user_model = get_single_user(db, user_id).await?;
    let worker_model = tbl_worker::Entity::find_by_id(user_id.clone())
        .one(db)
        .await?;

    if let Some(worker_model) = worker_model {
        let worker = GetWorker {
            email: user_model.email,
            role: user_model.role,
            name: user_model.name,
            user_id: user_model.user_id,
            boss_id: worker_model.boss_user_id.map(|f| f.clone()),
            boss: None,
        };
        return Ok(worker);
    }
    Err(CrudError::NotFound)
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
