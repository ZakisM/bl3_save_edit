use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::gi_fast_travel::GiFastTravel;

pub async fn all(pool: &SqlitePool) -> Result<Vec<GiFastTravel>> {
    let all_gi_fast_travel = sqlx::query_as!(GiFastTravel, "SELECT * FROM gi_fast_travel").fetch_all(pool).await?;

    Ok(all_gi_fast_travel)
}
