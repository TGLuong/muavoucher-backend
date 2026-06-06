use sea_orm::DatabaseConnection;

use crate::storage::repository::{area::AreaRepository, cookie::CookieRepository, user::UserRepository};

pub mod area;
pub mod cookie;
pub mod user;

pub type CenterDatabase = CenterRepository<UserRepository, CookieRepository, AreaRepository>;

#[derive(Debug, Clone)]
pub struct CenterRepository<USER, COOKIE, AREA> {
    pub user_repository: USER,
    pub cookie_repository: COOKIE,
    pub area_repository: AREA,
}

pub fn init_repository(conn: DatabaseConnection) -> CenterDatabase {
    CenterRepository {
        user_repository: UserRepository::new(conn.clone()),
        cookie_repository: CookieRepository::new(conn.clone()),
        area_repository: AreaRepository::new(conn.clone()),
    }
}
