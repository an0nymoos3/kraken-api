use anyhow::Result;

use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/// Runs initial migrations for database
pub fn run_migrations(conn: &mut SqliteConnection) -> Result<()> {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    Ok(())
}

/// Connect to local database
pub fn establish_connection() -> Result<SqliteConnection> {
    let database_url = env::var("DATABASE_URL")?;
    Ok(SqliteConnection::establish(&database_url)?) // TODO: Remove this weird thing
}
