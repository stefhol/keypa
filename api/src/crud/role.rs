use entities::model::tbl_role;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct GetRole {
    pub role_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
impl From<&tbl_role::Model> for GetRole {
    fn from(role: &tbl_role::Model) -> Self {
        Self {
            role_id: role.role_id,
            name: role.name.to_owned(),
            description: role.description.to_owned(),
        }
    }
}
