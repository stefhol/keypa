pub mod api;
pub mod crud;
pub mod openapi;
pub mod sheduler;
pub mod util;
use std::net::Ipv4Addr;

use actix_cors::Cors;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use openapi::ApiDoc;
use tracing::{info, log};
use utoipa_swagger_ui::SwaggerUi;

use dotenv;

use sea_orm::Database;

use utoipa::{
    openapi::{Info, Server},
    OpenApi,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let wwwroot = dotenv::var("WWWROOT")?;
    // env::set_current_dir(&wwwroot)?;
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .init();
    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let mut openapi = ApiDoc::openapi();
    openapi.servers = Some(vec![Server::new(format!(
        "{}:{}",
        Ipv4Addr::UNSPECIFIED,
        8080
    ))]);
    openapi.info = Info::new("KeyPa", "1.0.0");

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
    sheduler::start(db.clone());

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/translations", "./translations").show_files_listing())
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
                    .service(api::user::get_single_user)
                    .service(api::user::get_self)
                    //door
                    .service(api::door::get_self_door)
                    .service(api::door::get_user_authorized_doors)
                    .service(api::door::get_self_door_keycard)
                    .service(api::door::get_user_authorized_doors_keycard)
                    .service(api::door::get_doors_of_door_group)
                    //keycard
                    .service(api::keycard::get_self_keycard)
                    .service(api::keycard::get_all_keycards)
                    .service(api::keycard::get_user_keycard)
                    .service(api::keycard::get_single_request_keycard)
                    .service(api::keycard::change_keycard)
                    //request
                    .service(api::request::get_self_requests)
                    .service(api::request::get_requests_from_user)
                    .service(api::request::get_single_requests_from_user)
                    .service(api::request::get_self_requests_from_request_id)
                    .service(api::request::get_all_requests)
                    .service(api::request::get_single_requests)
                    .service(api::request::create_requests)
                    .service(api::request::change_requests)
                    //demo keycard
                    .service(api::use_keycard::use_keycard)
                    // keycard usage history
                    .service(api::keycard_usage_history::get_keycard_usage_history)
                    .service(api::keycard_usage_history::get_csv_keycard_usage_history)
                    // building
                    .service(api::building::get_buldings)
                    //comment
                    .service(api::comment::get_comments)
                    .service(api::comment::insert_comment)
                    //logs
                    .service(api::log::get_logs)
                    .service(api::log::get_logs_as_csv)
                    //department
                    .service(api::department::get_departments_of_self)
                    .service(api::department::get_departments_of_user)
                    .service(api::department::get_departments_of_self_and_keycard)
                    .service(api::department::get_departments_of_user_and_keycard)
                    .service(api::department::get_departments),
            )
            .app_data(web::Data::new(db.clone()))
            .wrap(Cors::permissive())
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await?;
    Ok(())
}
