use std::env;
use std::path::Path;
use std::str::FromStr;

use anyhow::Result;
use once_cell::sync::OnceCell;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, SqlitePool};
use tokio::io::AsyncReadExt;

use crate::bl3_save::Bl3Save;

mod bl3_save;
mod conduit;
mod error;
mod models;
mod parser;
mod protos;
mod util;

static DB_POOL: OnceCell<SqlitePool> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    env::set_var("RUST_LOG", "INFO");

    tracing_subscriber::fmt::init();

    let migrations = sqlx::migrate::Migrator::new(Path::new("./migrations")).await?;

    let database_url = env::var("DATABASE_URL").expect("Could not read 'DATABASE_URL'");

    let mut connect_options = SqliteConnectOptions::from_str(&database_url)?;
    connect_options.disable_statement_logging();

    let pool = SqlitePoolOptions::new()
        .max_connections(15)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to Sqlite Database");

    migrations.run(&pool).await?;

    DB_POOL.set(pool).expect("Failed to set DB_POOL");

    let mut save_file = tokio::fs::File::open("./test_files/19.sav").await?;
    let mut save_file_data = Vec::with_capacity(save_file.metadata().await?.len() as usize);

    save_file.read_to_end(&mut save_file_data).await?;

    let bl3_save = Bl3Save::from_data(&mut save_file_data).await?;

    println!("{}", bl3_save);

    Ok(())
}
