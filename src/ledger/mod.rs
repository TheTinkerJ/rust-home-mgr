//! 账本模块,独立模块
//! - 接口层 controller
//! - 数据层 database

pub mod controller;
pub mod database;

use rocket_sync_db_pools::database;

/// 配置项，用于注册数据库连接
#[database("mysql_database")]
pub struct MysqlDbConn(diesel::MysqlConnection);

/// 数据库操作美枚举,插入 or 更新
/// - Update,数据更新操作
/// - Insert,数据插入操作
pub enum DBMethod {
    Update,
    Insert,
}

/// 定义一个通用行为
///
/// 根据输入参数是否包含id字段来判断是否为更新
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
