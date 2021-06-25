use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::gi_mission::GiMission;

pub async fn all(pool: &SqlitePool) -> Result<Vec<GiMission>> {
    let all_gi_mission = sqlx::query_as!(GiMission, "SELECT * FROM gi_mission").fetch_all(pool).await?;

    Ok(all_gi_mission)
}
