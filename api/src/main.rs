pub mod api;
pub mod crud;
pub mod util;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

use dotenv;
use log::info;
use paperclip::{
    actix::{
        web::{self},
        // extension trait for actix_web::App and proc-macro attributes
        OpenApiExt,
    },
    v2::models::DefaultApiRaw,
};
use sea_orm::Database;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Error)
        .is_test(true)
        .init();
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
            .wrap_api_with_spec(DefaultApiRaw {
                info: paperclip::v2::models::Info {
                    version: "0.2".to_string(),
                    title: "KeyPa API Spec".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .service(
                web::scope("/api/v1")
                    .wrap(util::middleware::Auth)
                    //user services
                    .service(api::user::get_users)
                    .service(api::user::add_user)
                    .service(api::user::delete_user)
                    .service(api::user::update_user)
                    .service(api::user::get_single_user)
                    .service(api::user::get_self)
                    //login services
                    .service(api::auth::login)
                    .service(api::auth::register)
                    //woker services
                    .service(api::worker::add_worker)
                    .service(api::worker::update_worker)
                    .service(api::worker::delete_worker)
                    .service(api::worker::get_worker)
                    .service(api::worker::get_self),
            )
            .with_json_spec_at("/api/spec/v2/spec.json")
            .build()
            .app_data(web::Data::new(db.clone()))
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
