use sea_orm::DatabaseConnection;

use crate::storage::repository::{
    area::AreaRepository, cookie::CookieRepository, product::ProductRepository, shop::ShopRepository, user::UserRepository,
};

pub mod area;
pub mod cookie;
pub mod product;
pub mod shop;
pub mod user;

pub type CenterDatabase = CenterRepository<UserRepository, CookieRepository, AreaRepository, ShopRepository, ProductRepository>;

#[derive(Debug, Clone)]
pub struct CenterRepository<USER, COOKIE, AREA, SHOP, PRODUCT> {
    pub user_repository: USER,
    pub cookie_repository: COOKIE,
    pub area_repository: AREA,
    pub shop_repository: SHOP,
    pub product_repository: PRODUCT,
}

pub fn init_repository(conn: DatabaseConnection) -> CenterDatabase {
    CenterRepository {
        user_repository: UserRepository::new(conn.clone()),
        cookie_repository: CookieRepository::new(conn.clone()),
        area_repository: AreaRepository::new(conn.clone()),
        shop_repository: ShopRepository::new(conn.clone()),
        product_repository: ProductRepository::new(conn.clone()),
    }
}
