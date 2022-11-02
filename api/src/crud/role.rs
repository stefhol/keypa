use entities::model::tbl_role;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Apiv2Schema)]
pub struct GetRole {
    pub role_id: String,
    pub name: String,
    pub description: Option<String>,
}
impl From<&tbl_role::Model> for GetRole {
    fn from(role: &tbl_role::Model) -> Self {
        Self {
            role_id: role.role_id.to_string(),
            name: role.name.to_owned(),
            description: role.description.to_owned(),
        }
    }
}
