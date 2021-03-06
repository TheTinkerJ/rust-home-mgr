use crate::diesel::{prelude::*, result::Error};

use crate::ledger::database::model::*;
use crate::ledger::Upsert;
use crate::utils::CommonRespose;
use crate::{ledger::MysqlDbConn, utils::CommonPageRespose};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::ledger::database::schema::shopping_records::dsl::*;
use crate::ledger::DBMethod::*;

/// 消费记录列表
///
/// 支持分页,目前需要从 page 0 开始计数.
///
/// 页的计算方式 {start: page*limit, end: page*limit+limit}
#[get("/list?<page>&<limit>")]
pub async fn shopping_records_list(
    page: i32,
    limit: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonPageRespose<Vec<ShoppingRecords>>>) {
    let exec_result = db
        .run(move |conn| {
            shopping_records
                .offset((page * limit).into())
                .limit(limit.into())
                .load::<ShoppingRecords>(conn)
        })
        .await;
    let total_count: Result<usize, Error> = db
        .run(move |conn| shopping_records.count().execute(conn))
        .await;
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

/// 消费记录获取
///
/// 根据记录的ID获取
#[get("/shopping-records/query-by?<shopping_id>")]
pub async fn shopping_records_fetch(
    shopping_id: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<ShoppingRecords>>) {
    let exec_result = db
        .run(move |conn| {
            shopping_records
                .filter(id.eq(shopping_id))
                .first::<ShoppingRecords>(conn)
        })
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

/// 消费记录更新
///
/// 需要传入 json, 以下仅表示类似结构,引号内值表示需要填入的类型,Option表示可选
/// ```json
/// {
///    "id": "Oprion<i32>,自身id",  
///    "place_id": "i32",  
///    "comment": "Option<String>",            
/// }
///```
#[post("/update", data = "<input>", format = "json")]
pub async fn shopping_records_update(
    input: Json<ShoppingRecords4Update>,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<usize>>) {
    let update_body = input.into_inner();
    let exec_result: Result<usize, Error> = match update_body.method() {
        Insert => {
            db.run(move |conn| {
                diesel::insert_into(shopping_records)
                    .values(vec![update_body])
                    .execute(conn)
            })
            .await
        }
        Update => {
            db.run(move |conn| {
                diesel::update(shopping_records)
                    .filter(id.eq(update_body.id))
                    .set((
                        place_id.eq(update_body.place_id),
                        comment.eq(update_body.comment),
                    ))
                    .execute(conn)
            })
            .await
        }
    };
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

/// 消费记录删除
///
/// 根据记录的ID删除
#[get("/drop-by?<shopping_id>")]
pub async fn shopping_records_delete(
    shopping_id: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<usize>>) {
    let exec_result = db
        .run(move |conn| diesel::delete(shopping_records.filter(id.eq(shopping_id))).execute(conn))
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}
