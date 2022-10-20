use sqlx::{self, postgres::PgPoolOptions, Executor};
pub async fn run_migration(db_connection_string: &str, db_name: &str) -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&format!("{}/{}", db_connection_string, db_name))
        .await?;
    sqlx::migrate!("../migrations").run(&pool).await?;
    Ok(())
}
pub async fn drop_database(db_connection_string: &str, db_name: &str) -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(db_connection_string)
        .await?;
    pool.execute(format!("DROP DATABASE IF EXISTS {}", db_name).as_str())
        .await?;
    Ok(())
}
pub async fn create_database(db_connection_string: &str, db_name: &str) -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(db_connection_string)
        .await?;
    pool.execute(format!("CREATE DATABASE {}", db_name).as_str())
        .await?;
    Ok(())
}
pub fn split_connection_string(con_str: &str) -> (&str, &str) {
    con_str.rsplit_once("/").unwrap()
}
