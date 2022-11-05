use entities::model::{tbl_leader, tbl_worker};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::error::CrudError;

use super::{role::GetRole, user::get_single_user};
#[derive(Serialize, Deserialize, Debug)]
pub struct Worker {
    pub user_id: String,
    pub worker_id: String,
    pub name: String,
    pub role: Option<GetRole>,
    pub email: String,
    #[serde(skip_serializing)]
    //only for internal use
    boss_id: Option<String>,

    pub boss: Option<Boss>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Boss {
    pub user_id: String,
    pub worker_id: String,
    pub leader_id: String,
    pub name: String,
    pub role: Option<GetRole>,
    pub email: String,
}

pub async fn get_worker_by_user_id(
    db: &DatabaseConnection,
    user_id: &str,
) -> Result<Option<Worker>, CrudError> {
    let temp_worker = __get_worker_by_user_id(db, user_id).await?;
    if let Some(boss_id) = &temp_worker.boss_id {
        let boss_id = Uuid::parse_str(boss_id)?;
        let boss_model = tbl_leader::Entity::find()
            .filter(tbl_leader::Column::LeaderId.eq(boss_id))
            .one(db)
            .await?;
        let mut boss = None;
        if let Some(boss_model) = boss_model {
            let user_boss_model = __get_worker_by_user_id(db, &boss_id.to_string()).await?;

            boss = Some(Boss {
                user_id: user_boss_model.user_id.to_string(),
                worker_id: boss_model.worker_id.to_string(),
                leader_id: boss_model.leader_id.to_string(),
                email: user_boss_model.email,
                name: user_boss_model.name,
                role: user_boss_model.role,
            });
        }
        return Ok(Some(Worker {
            boss,
            ..temp_worker
        }));
    }
    Ok(None)
}
/// Private function that gets a Worker without Boss struct
async fn __get_worker_by_user_id(
    db: &DatabaseConnection,
    user_id: &str,
) -> Result<Worker, CrudError> {
    let user_model = get_single_user(db, &user_id).await?;
    let worker_model = tbl_worker::Entity::find()
        .filter(tbl_worker::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    if let (Some(user_model), Some(worker_model)) = (user_model, worker_model) {
        let worker = Worker {
            worker_id: worker_model.worker_id.to_string(),
            email: user_model.email,
            role: user_model.role,
            name: user_model.name,
            user_id: user_model.user_id,
            boss_id: worker_model.boss_id.map(|f| f.to_string()),
            boss: None,
        };
        return Ok(worker);
    }
    Err(CrudError::NotFound)
}

pub async fn is_leader_by_worker_id(
    worker_id: &str,
    db: &DatabaseConnection,
) -> Result<bool, CrudError> {
    let worker_id = Uuid::parse_str(worker_id)?;
    let leader = tbl_leader::Entity::find()
        .filter(tbl_leader::Column::WorkerId.eq(worker_id))
        .one(db)
        .await?;
    return Ok(leader.is_some());
}
pub async fn get_worker_id_by_user_id(
    user_id: &str,
    db: &DatabaseConnection,
) -> Result<Option<Uuid>, CrudError> {
    let user_id = Uuid::parse_str(user_id)?;
    let worker = tbl_worker::Entity::find()
        .filter(tbl_worker::Column::UserId.eq(user_id))
        .select_only()
        .column(tbl_worker::Column::WorkerId)
        .one(db)
        .await?;
    if let Some(worker) = worker {
        return Ok(Some(worker.worker_id));
    }
    Ok(None)
}
