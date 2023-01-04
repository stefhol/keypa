use entities::model::tbl_room;
use sea_orm::{DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::util::error::CrudError;

use super::building::GetBuilding;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetRoom {
    pub room_id: Uuid,
    pub name: String,
    pub floor: i32,
    pub is_sensitive: Option<bool>,
    pub building: Option<GetBuilding>,
    pub building_id: Uuid,
}
impl From<&tbl_room::Model> for GetRoom {
    fn from(room: &tbl_room::Model) -> Self {
        Self {
            name: room.name.clone(),
            room_id: room.room_id,
            floor: room.floor,
            is_sensitive: room.is_sensitive,
            building_id: room.building_id,
            building: None,
        }
    }
}
pub async fn get_door_from_room_id(
    room_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<GetRoom, CrudError> {
    let model = tbl_room::Entity::find_by_id(room_id.clone())
        .one(db)
        .await?;
    match model {
        Some(model) => Ok((&model).into()),
        None => Err(CrudError::NotFound),
    }
}
pub async fn get_rooms_id_sensitive(db: &DatabaseConnection) -> Result<Vec<Uuid>, CrudError> {
    #[derive(FromQueryResult)]
    struct QueryResult {
        room_id: Uuid,
    }
    let query_result: Vec<QueryResult> =
        QueryResult::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
        select room_id from tbl_room
        where is_sensitive = true;
        "#,
            vec![],
        ))
        .all(db)
        .await?;
    Ok(query_result.iter().map(|f| f.room_id).collect())
}
