use utoipa::{OpenApi};


use crate::{crud, api};
#[derive(OpenApi)]
#[openapi(
    
    paths(
        //login
        api::auth::login,
        api::auth::logout,
        //user
        api::user::get_users,
        api::user::get_single_user,
        api::user::get_self,
        //key
        api::door::get_self_door,
        api::door::get_user_authorized_doors,
        api::door::get_doors_of_door_group,
        //keycard
        api::keycard::get_self_keycard,
        api::keycard::get_user_keycard,
        
        //requests
        api::request::get_self_requests,
        api::request::get_requests_from_user,
        api::request::get_single_requests_from_user,
        api::request::get_self_requests_from_request_id,
        api::request::get_all_pending_requests,
        api::request::get_single_requests,
        api::request::create_requests,
        //buildings
        api::building::get_buldings,
        //department
        api::department::get_departments,
    ),
    components(schemas(
        api::auth::Login,
        crud::role::GetRole,

        crud::door::GetDoor,
        crud::room::GetRoom,
        crud::building::GetBuilding,
        crud::user::GetUser,
        crud::keycard::GetKeycard,

        crud::request::get::GetRequestWithComments,
        crud::request::get::GetComments,
        crud::request::create::CreateRequest,
        crud::request::create::IndividualRooms,
        crud::access::GetCompleteBuilding,
        crud::access::GetCompleteRoom,
        crud::access::GetCompleteDoor,
        crud::department::GetDepartment,
    ))
)]
pub struct ApiDoc;
