use crate::diesel::{prelude::*, result::Error};

use crate::ledger::database::model::*;
use crate::ledger::Upsert;
use crate::utils::CommonRespose;
use crate::{ledger::MysqlDbConn, utils::CommonPageRespose};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::ledger::database::schema::goods::dsl::*;
use crate::ledger::DBMethod::*;

#[get("/list?<page>&<limit>")]
pub async fn goods_list(
    page: i32,
    limit: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonPageRespose<Vec<Goods>>>) {
    let exec_result = db
        .run(move |conn| {
            goods
                .offset((page * limit).into())
                .limit(limit.into())
                .load::<Goods>(conn)
        })
        .await;
    let total_count: Result<usize, Error> = db.run(move |conn| goods.count().execute(conn)).await;
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

#[get("/query-by-name?<item_name>")]
pub async fn goods_fetch_by_name(
    item_name: String,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<Vec<Goods>>>) {
    let pattern = format!("%{}%", item_name);
    let exec_result: Result<Vec<Goods>, Error> = db
        .run(move |conn| goods.filter(name.like(pattern)).load::<Goods>(conn))
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

#[get("/query-by-id?<item_id>")]
pub async fn goods_fetch_by_id(
    item_id: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<Vec<Goods>>>) {
    let exec_result: Result<Vec<Goods>, Error> = db
        .run(move |conn| goods.filter(id.eq(item_id)).load::<Goods>(conn))
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

#[post("/update", data = "<input>", format = "json")]
pub async fn goods_update(
    input: Json<Goods4Update>,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<usize>>) {
    let update_body = input.into_inner();
    let exec_result: Result<usize, Error> = match update_body.method() {
        Insert => {
            db.run(move |conn| {
                diesel::insert_into(goods)
                    .values(vec![update_body])
                    .execute(conn)
            })
            .await
        }
        Update => {
            db.run(move |conn| {
                diesel::update(goods)
                    .filter(id.eq(update_body.id))
                    .set((
                        name.eq(update_body.name),
                        cls1.eq(update_body.cls1),
                        cls2.eq(update_body.cls2),
                        cls3.eq(update_body.cls3),
                    ))
                    .execute(conn)
            })
            .await
        }
    };
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

#[get("/drop-by?<item_id>")]
pub async fn goods_delete(item_id: i32, db: MysqlDbConn) -> (Status, Json<CommonRespose<usize>>) {
    let exec_result = db
        .run(move |conn| diesel::delete(goods.filter(id.eq(item_id))).execute(conn))
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}
