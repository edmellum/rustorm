#[cfg(feature = "db-auth")]
use crate::db_auth::{
    Role,
    User,
};
use crate::{
    table::SchemaContent,
    DbError,
    Rows,
    Table,
    TableName,
    Value,
};
use rustorm_codegen::FromDao;
use serde::Serialize;


/// The current database name and its comment
#[derive(Serialize, FromDao)]
pub struct DatabaseName {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}



pub trait Database {
    fn execute_sql_with_return(&mut self, sql: &str, param: &[&Value]) -> Result<Rows, DbError>;

    fn get_table(&mut self, table_name: &TableName) -> Result<Table, DbError>;

    fn get_all_tables(&mut self) -> Result<Vec<Table>, DbError>;

    fn get_grouped_tables(&mut self) -> Result<Vec<SchemaContent>, DbError>;

    fn get_database_name(&mut self) -> Result<Option<DatabaseName>, DbError>;

    #[cfg(feature = "db-auth")]
    fn get_users(&mut self) -> Result<Vec<User>, DbError>;

    #[cfg(feature = "db-auth")]
    fn get_user_detail(&mut self, username: &str) -> Result<Vec<User>, DbError>;

    #[cfg(feature = "db-auth")]
    fn get_roles(&mut self, username: &str) -> Result<Vec<Role>, DbError>;
}
