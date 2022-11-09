use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn init_database(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(db_url).await?;

    Migrator::up(&db, None).await?;
    Ok(db)
}
