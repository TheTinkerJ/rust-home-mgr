//! 账本模块,独立模块
//! - 接口层
//! - 数据适配层
//! - 数据层

pub mod controller;
pub mod database;

use rocket_sync_db_pools::database;

/// 配置项，用于注册数据库连接
#[database("mysql_database")]
pub struct MysqlDbConn(diesel::MysqlConnection);

pub enum DBMethod {
    Update,
    Insert,
}

pub trait Upsert {
    fn method(&self) -> DBMethod;
}

impl Upsert for database::model::MarketPlace4Update {
    fn method(&self) -> DBMethod {
        match self.id {
            Some(_) => DBMethod::Update,
            None => DBMethod::Insert,
        }
    }
}

impl Upsert for database::model::ShoppingRecords4Update {
    fn method(&self) -> DBMethod {
        match self.id {
            Some(_) => DBMethod::Update,
            None => DBMethod::Insert,
        }
    }
}

impl Upsert for database::model::Goods4Update {
    fn method(&self) -> DBMethod {
        match self.id {
            Some(_) => DBMethod::Update,
            None => DBMethod::Insert,
        }
    }
}

impl Upsert for database::model::CostDetail4Update {
    fn method(&self) -> DBMethod {
        match self.id {
            Some(_) => DBMethod::Update,
            None => DBMethod::Insert,
        }
    }
}
