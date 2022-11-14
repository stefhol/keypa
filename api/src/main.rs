pub mod api;
pub mod crud;
pub mod util;

use std::net::Ipv4Addr;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use utoipa_swagger_ui::SwaggerUi;

use dotenv;
use log::info;

use sea_orm::Database;

use utoipa::{OpenApi, openapi::{Server, Info}};
#[derive(OpenApi)]
#[openapi(
    
    paths(
        //login
        api::auth::login,
        api::auth::logout,
        //user
        api::user::get_users,
        api::user::add_user,
        api::user::delete_user,
        api::user::update_user,
        api::user::get_single_user,
        api::user::get_self,
        //worker
        api::worker::add_worker,
        api::worker::update_worker,
        api::worker::delete_worker,
        //key
        api::door::get_self_door,
        api::door::get_user_authorized_doors,
        api::door::get_doors_of_door_group,
        //keycard
        api::keycard::get_self_keycard,
        api::keycard::get_keycard_of_user_id,
        api::keycard::get_single_keycard,
        
        //key_group
        api::door_group::add_key_into_key_group,
        api::door_group::delete_key_from_key_group,
        api::door_group::get_key_group,
        api::door_group::get_keys_of_key_group,
        api::door_group::get_self_key_group,
        api::door_group::add_key_group,
        api::door_group::upate_key_group,
        api::request::get_self_requests,
        api::request::get_requests_from_user,
        api::request::get_single_requests_from_user,
        api::request::get_self_requests_from_request_id,
        api::building::get_buldings,
        api::request::get_all_pending_requests,
        api::request::get_single_requests,
        
    ),
    components(schemas(
        api::auth::Login,
        crud::role::GetRole,
        crud::user::CreateUser,
        crud::user::ChangeUser,
        crud::door::GetDoor,
        crud::room::GetRoom,
        crud::building::GetBuilding,
        crud::user::GetUser,
        crud::user::GetUserSmall,
        crud::worker::GetSmallWorker,
        crud::worker::CreateWorker,
        crud::door_group::CreateKeyGroup,
        crud::door_group::ChangeKeyGroup,
        crud::door_group::GetKeyGroup,
        crud::request::GetRequestWithComments,
        crud::request::GetComments,
        crud::building::GetCompleteBuilding,
        crud::building::GetCompleteRoom,
        crud::building::GetCompleteDoor,
        crud::keycard::GetKeycard,
    ))
)]
struct ApiDoc;
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Error)
        .is_test(true)
        .init();
    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let mut openapi = ApiDoc::openapi();
    
    openapi.servers = Some(vec![
       Server::new(format!("{}:{}",Ipv4Addr::UNSPECIFIED, 8080))
    ]);
    openapi.info = Info::new("KeyPa", "0.0.1");

    let database_url = dotenv::var("DATABASE_URL")?;
    let (database_url, db_name) = migration_helper::split_connection_string(&database_url);
    let err = migration_helper::create_database(database_url, db_name).await;
    if let Err(err) = err {
        info!("{}", err.to_string());
    }
    let err = migration_helper::run_migration(database_url, db_name).await;
    if let Err(err) = err {
        info!("{}", err.to_string());
    }

    let db = Database::connect(format!("{}/{}", database_url, db_name)).await?;
    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
            .service(
                web::scope("/api/v1")
                    .wrap(util::middleware::Auth)
                    //login services
                    .service(api::auth::login)
                    .service(api::auth::logout)
                    //user services
                    .service(api::user::get_users)
                    .service(api::user::add_user)
                    .service(api::user::delete_user)
                    .service(api::user::update_user)
                    .service(api::user::get_single_user)
                    .service(api::user::get_self)
                    //woker services
                    .service(api::worker::add_worker)
                    .service(api::worker::update_worker)
                    .service(api::worker::delete_worker)
                    //key
                    .service(api::door::get_self_door)
                    .service(api::door::get_user_authorized_doors)
                    .service(api::door::get_doors_of_door_group)
                    //keycard
                    .service(api::keycard::get_self_keycard)
                    .service(api::keycard::get_keycard_of_user_id)
                    .service(api::keycard::get_single_keycard)
                    
                    //key_group
                    .service(api::door_group::add_key_into_key_group)
                    .service(api::door_group::delete_key_from_key_group)
                    .service(api::door_group::get_key_group)
                    .service(api::door_group::get_self_key_group)
                    .service(api::door_group::get_keys_of_key_group)
                    .service(api::door_group::add_key_group)
                    .service(api::door_group::upate_key_group)
                    //request
                    .service(api::request::get_self_requests)
                    .service(api::request::get_requests_from_user)
                    .service(api::request::get_single_requests_from_user)
                    .service(api::request::get_self_requests_from_request_id)
                    .service(api::request::get_all_pending_requests)
                    .service(api::request::get_single_requests)
                    // building
                    .service(api::building::get_buldings)
                ,
            )
            .app_data(web::Data::new(db.clone()))
            .wrap(Cors::permissive())
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await?;
    Ok(())
}
