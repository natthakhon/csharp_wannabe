use pgsql_provider::PgSqlRandomRepo;
pub struct AppState{
    pub repo: PgSqlRandomRepo,
}