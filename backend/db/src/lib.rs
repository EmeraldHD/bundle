use anyhow::{anyhow, Result};
use diesel::{migration::MigrationVersion, Connection, ConnectionResult, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn connect() -> ConnectionResult<Self> {
        SqliteConnection::establish("bundle.db").map(|connection| Self { connection })
    }

    pub fn migrate(&mut self) -> Result<Vec<MigrationVersion>> {
        match self.connection.run_pending_migrations(MIGRATIONS) {
            Ok(migrations) => Ok(migrations),
            Err(error) => Err(anyhow!(error)),
        }
    }

    pub fn migrate_verbose(&mut self) -> Result<Vec<MigrationVersion>> {
        let migrations = self.migrate()?;
        print!("Database up to date ");
        if migrations.is_empty() {
            println!("(no migrations to run)");
        } else {
            println!("(ran {} migrations)", migrations.len());
        }
        Ok(migrations)
    }
}
