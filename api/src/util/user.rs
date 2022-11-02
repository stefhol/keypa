use entities::model::tbl_user;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

use crate::crud::user::{ChangeUser, CreateUser};

use super::error::CrudError;

pub trait UpdateActiveModel<T> {
    fn convert(&self, model: T) -> Result<T, CrudError>
    where
        T: ActiveModelTrait;
}
pub trait CreateActiveModel<T> {
    fn convert(&self) -> Result<T, CrudError>
    where
        T: ActiveModelTrait;
}
impl UpdateActiveModel<tbl_user::ActiveModel> for ChangeUser {
    fn convert(
        &self,
        mut model: tbl_user::ActiveModel,
    ) -> Result<tbl_user::ActiveModel, CrudError> {
        if let Some(email) = &self.email {
            model.email = Set(email.to_string());
        }
        if let Some(name) = &self.name {
            model.name = Set(name.to_string());
        }
        if let Some(role) = &self.role {
            if let Some(role_id) = &role {
                model.role_id = Set(Some(Uuid::parse_str(&role_id)?));
            } else {
                model.role_id = Set(None);
            }
        }
        Ok(model)
    }
}
impl CreateActiveModel<tbl_user::ActiveModel> for CreateUser {
    fn convert(&self) -> Result<tbl_user::ActiveModel, CrudError>
    where
        tbl_user::ActiveModel: ActiveModelTrait,
    {
        Ok(tbl_user::ActiveModel {
            name: Set(self.name.to_string()),
            email: Set(self.email.to_string()),
            password: Set(self.password.to_string()),
            ..Default::default()
        })
    }
}
