use entities::model::tbl_door;
use sea_orm::{DatabaseConnection, DbBackend, EntityTrait, Statement};

use uuid::Uuid;

use crate::util::error::CrudError;

use super::door::GetDoor;

pub async fn get_keys_of_user_id(
    user_id: &Uuid,
    db: &DatabaseConnection,
) -> Result<Vec<GetDoor>, CrudError> {
    let values = tbl_door::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select (tbl_door.*) from tbl_user
            join tbl_door_group on tbl_user.user_id = tbl_door_group.owner_id
            join tbl_door_to_group_door on tbl_door_group.door_group_id = tbl_door_to_group_door.door_group_id
            join tbl_door on tbl_door_to_group_door.door_id = tbl_door.door_id
            where user_id = $1"#,
            vec![user_id.clone().into()],
        ))
        .all(db)
        .await?;

    Ok(values.iter().map(|f| f.into()).collect())
}
