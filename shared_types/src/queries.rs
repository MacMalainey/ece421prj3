use diesel::sqlite::SqliteConnection;
use diesel::result::Error;

#[cfg(feature = "run_migrations")]
embed_migrations!();

#[cfg(feature = "run_migrations")]
pub fn run_migrations(conn: &SqliteConnection) -> Result<(), Error> {
    use diesel_migrations::RunMigrationsError;
    embedded_migrations::run_with_output(conn, &mut std::io::stdout()).map_err(
        |err| match err {
            RunMigrationsError::MigrationError(err) => panic!("DATABASE INITIALIZATION ERROR:\n{:?}", err),
            RunMigrationsError::QueryError(err) => err,
            RunMigrationsError::EmptyMigration => panic!("DATABASE INITIALIZATION ERROR:\n\tempty_migration"),
            _ => panic!("DATABASE INITIALIZATION ERROR:\n\tunknown error")
        }
    )
}