use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use sqlx::SqlitePool;

pub trait OnceCellExt {
    fn get_checked(&self) -> Result<&SqlitePool>;
}

impl OnceCellExt for OnceCell<SqlitePool> {
    fn get_checked(&self) -> Result<&SqlitePool> {
        self.get().context("Failed to get DB_POOL")
    }
}
