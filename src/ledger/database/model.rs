//! 项目内的数据库模型
use crate::ledger::database::schema::*;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

/// 消费场所的映射结构
///
/// 用法: 数据库 -> Ledger
#[derive(Queryable, Debug, Serialize)]
pub struct MarketPlace {
    pub id: Option<i32>,
    pub gmt_create: Option<NaiveDateTime>,
    pub gmt_modified: Option<NaiveDateTime>,
    pub name: String,
    pub lng: Option<String>,
    pub lat: Option<String>,
    pub location_desc: Option<String>,
    pub adcode: Option<String>,
}

/// 消费场所的映射结构
///
/// 用法: 数据库 <- Ledger
#[derive(Insertable, Deserialize, Debug)]
#[table_name = "market_place"]
pub struct MarketPlace4Update {
    pub id: Option<i32>,
    pub name: String,                  // 商家名称
    pub lng: Option<String>,           // 经度
    pub lat: Option<String>,           // 维度
    pub location_desc: Option<String>, // 位置描述
    pub adcode: Option<String>,        // 商家所在位置行政编码
}

/// 消费记录的映射结构
///
/// 用法: 数据库 -> Ledger
#[derive(Queryable, Debug, Serialize)]
pub struct ShoppingRecords {
    pub id: Option<i32>,
    pub gmt_create: Option<NaiveDateTime>,
    pub gmt_modified: Option<NaiveDateTime>,
    pub place_id: i32,           // 位置关联
    pub comment: Option<String>, // 消费总体评论
}

/// 消费记录的映射结构
///
/// 用法: 数据库 <- Ledger
#[derive(Insertable, Deserialize, Debug)]
#[table_name = "shopping_records"]
pub struct ShoppingRecords4Update {
    pub id: Option<i32>,
    pub place_id: i32,           // 位置关联
    pub comment: Option<String>, // 消费总体评论
}

/// 消费明细的映射结构
///
/// 用法: 数据库 -> Ledger
#[derive(Queryable, Debug, Serialize)]
pub struct CostDetail {
    pub id: Option<i32>,
    pub gmt_create: Option<NaiveDateTime>,
    pub gmt_modified: Option<NaiveDateTime>,
    pub shopping_id: i32,        // 消费关联
    pub good_id: i32,            // 商品关联
    pub cost: i32,               // 金额（分）
    pub count: i32,              // 数量（个）
    pub comment: Option<String>, // 消费评价
}

/// 消费明细的映射结构
///
/// 用法: 数据库 <- Ledger
#[derive(Insertable, Deserialize, Debug)]
#[table_name = "cost_detail"]
pub struct CostDetail4Update {
    pub id: Option<i32>,
    pub shopping_id: i32,        // 消费关联
    pub good_id: i32,            // 商品关联
    pub cost: i32,               // 金额（分）
    pub count: i32,              // 数量（个）
    pub comment: Option<String>, // 消费评价
}
/// 消费商品的映射结构
///
/// 用法: 数据库 -> Ledger
#[derive(Queryable, Debug, Serialize)]
pub struct Goods {
    pub id: Option<i32>,
    pub gmt_create: Option<NaiveDateTime>,
    pub gmt_modified: Option<NaiveDateTime>,
    pub name: String,         // 消费项名称
    pub cls1: Option<String>, // 一级类目
    pub cls2: Option<String>, // 二级类目
    pub cls3: Option<String>, // 三级类目
}

/// 消费商品的映射结构
///
/// 用法: 数据库 <- Ledger
#[derive(Insertable, Deserialize, Debug)]
#[table_name = "goods"]
pub struct Goods4Update {
    pub id: Option<i32>,
    pub name: String,         // 消费项名称
    pub cls1: Option<String>, // 一级类目
    pub cls2: Option<String>, // 二级类目
    pub cls3: Option<String>, // 三级类目
}
