use sea_orm::DatabaseConnection;

use crate::storage::repository::user::UserRepository;

pub mod user;

pub type CenterDatabase = CenterRepository<UserRepository>;

#[derive(Debug, Clone)]
pub struct CenterRepository<USER> {
    pub user_repository: USER,
}

pub fn init_repository(conn: DatabaseConnection) -> CenterDatabase {
    CenterRepository {
        user_repository: UserRepository::new(conn.clone()),
    }
}
