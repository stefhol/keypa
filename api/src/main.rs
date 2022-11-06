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
        api::auth::register,
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
        api::worker::get_worker,
        api::worker::get_self,
    ),
    components(schemas(
        api::auth::Login,
        crud::role::GetRole,
        crud::user::CreateUser,
        crud::user::ChangeUser,
        crud::user::GetUser,
        crud::worker::GetWorker,
        crud::worker::CreateWorker,
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
                    .service(api::auth::register)
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
                    .service(api::worker::get_worker)
                    .service(api::worker::get_self),
            )
            .app_data(web::Data::new(db.clone()))
            .wrap(Cors::permissive())
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await?;
    Ok(())
}
