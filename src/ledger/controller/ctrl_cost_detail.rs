//! 消费明细请求响应模块
use crate::diesel::{prelude::*, result::Error};

use crate::ledger::database::model::*;
use crate::ledger::Upsert;
use crate::utils::CommonRespose;
use crate::{ledger::MysqlDbConn, utils::CommonPageRespose};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::ledger::database::schema::cost_detail::dsl::*;
use crate::ledger::DBMethod::*;

/// 消费明细列表
///
/// 支持分页,目前需要从 page 0 开始计数.
///
/// 页的计算方式 {start: page*limit, end: page*limit+limit}
#[get("/list?<page>&<limit>")]
pub async fn cost_detail_list(
    page: i32,
    limit: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonPageRespose<Vec<CostDetail>>>) {
    let exec_result = db
        .run(move |conn| {
            cost_detail
                .offset((page * limit).into())
                .limit(limit.into())
                .load::<CostDetail>(conn)
        })
        .await;
    let total_count: Result<usize, Error> =
        db.run(move |conn| cost_detail.count().execute(conn)).await;
    let total = total_count.ok().map(|u| u as i32);
    (
        Status::Accepted,
        Json(CommonPageRespose::build_page(
            exec_result,
            total,
            Some(page),
            Some(limit),
        )),
    )
}

/// 消费明细获取
///
/// 根据记录的id获取
#[get("/query-by?<cost_detail_id>")]
pub async fn cost_detail_fetch(
    cost_detail_id: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<Vec<CostDetail>>>) {
    let exec_result: Result<Vec<CostDetail>, Error> = db
        .run(move |conn| {
            cost_detail
                .filter(id.eq(cost_detail_id))
                .load::<CostDetail>(conn)
        })
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

/// 消费明细获取
///
/// 根据关联的消费记录获取
#[get("/related-to?<ref_shopping_id>")]
pub async fn cost_detail_related(
    ref_shopping_id: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<Vec<CostDetail>>>) {
    let exec_result: Result<Vec<CostDetail>, Error> = db
        .run(move |conn| {
            cost_detail
                .filter(shopping_id.eq(ref_shopping_id))
                .load::<CostDetail>(conn)
        })
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

/// 消费明细更新
///
/// 需要传入 json, 以下仅表示类似结构,引号内值表示需要填入的类型,Option表示可选
/// ```json
/// {
///    "id": "Oprion<i32>,自身id",  
///    "shopping_id": "i32,消费关联",  
///    "good_id": "i32,商品关联",            
///    "cost": "i32,金额（分）",               
///    "count": "i32,数量（个）",              
///    "comment": "Option<String>,消费评价 ",
/// }
///```
#[post("/update", data = "<input>", format = "json")]
pub async fn cost_detail_update(
    input: Json<CostDetail4Update>,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<usize>>) {
    let update_body = input.into_inner();
    let exec_result: Result<usize, Error> = match update_body.method() {
        Insert => {
            db.run(move |conn| {
                diesel::insert_into(cost_detail)
                    .values(vec![update_body])
                    .execute(conn)
            })
            .await
        }
        Update => {
            db.run(move |conn| {
                diesel::update(cost_detail)
                    .filter(id.eq(update_body.id))
                    .set((
                        shopping_id.eq(update_body.shopping_id),
                        good_id.eq(update_body.good_id),
                        cost.eq(update_body.cost),
                        count.eq(update_body.count),
                        comment.eq(update_body.comment),
                    ))
                    .execute(conn)
            })
            .await
        }
    };
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

/// 消费明细删除
///
/// 根据消费明细的ID删除
#[get("/drop-by?<cost_detail_id>")]
pub async fn cost_detail_delete(
    cost_detail_id: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<usize>>) {
    let exec_result = db
        .run(move |conn| diesel::delete(cost_detail.filter(id.eq(cost_detail_id))).execute(conn))
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}
