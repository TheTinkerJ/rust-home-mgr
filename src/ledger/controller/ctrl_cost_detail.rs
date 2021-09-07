use crate::diesel::{prelude::*, result::Error};

use crate::ledger::database::model::*;
use crate::ledger::Upsert;
use crate::utils::CommonRespose;
use crate::{ledger::MysqlDbConn, utils::CommonPageRespose};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::ledger::database::schema::cost_detail::dsl::*;
use crate::ledger::DBMethod::*;

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
