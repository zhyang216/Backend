pub use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres_db")]
pub struct AccountsDb(rocket_db_pools::diesel::PgPool);