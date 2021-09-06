use crate::diesel::{prelude::*, result::Error};

use crate::ledger::database::model::*;
use crate::ledger::Upsert;
use crate::utils::CommonRespose;
use crate::{ledger::MysqlDbConn, utils::CommonPageRespose};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::ledger::database::schema::market_place::dsl::*;
use crate::ledger::DBMethod::*;

#[get("/list?<page>&<limit>")]
pub async fn market_place_list(
    page: i32,
    limit: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonPageRespose<Vec<MarketPlace>>>) {
    let exec_result = db
        .run(move |conn| {
            market_place
                .offset((page * limit).into())
                .limit(limit.into())
                .load::<MarketPlace>(conn)
        })
        .await;
    let total_count: Result<usize, Error> =
        db.run(move |conn| market_place.count().execute(conn)).await;
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

#[get("/query-by?<market_name>")]
pub async fn market_place_fetch(
    market_name: String,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<Vec<MarketPlace>>>) {
    let pattern = format!("%{}%", market_name);
    let exec_result: Result<Vec<MarketPlace>, Error> = db
        .run(move |conn| {
            market_place
                .filter(name.like(pattern))
                .load::<MarketPlace>(conn)
        })
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

#[post("/update", data = "<input>", format = "json")]
pub async fn market_place_update(
    input: Json<MarketPlace4Update>,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<usize>>) {
    let update_body = input.into_inner();
    let exec_result: Result<usize, Error> = match update_body.method() {
        Insert => {
            db.run(move |conn| {
                diesel::insert_into(market_place)
                    .values(vec![update_body])
                    .execute(conn)
            })
            .await
        }
        Update => {
            db.run(move |conn| {
                diesel::update(market_place)
                    .filter(id.eq(update_body.id))
                    .set((
                        name.eq(update_body.name),
                        lng.eq(update_body.lng),
                        lat.eq(update_body.lat),
                        location_desc.eq(update_body.location_desc),
                        adcode.eq(update_body.adcode),
                    ))
                    .execute(conn)
            })
            .await
        }
    };
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}

#[get("/drop-by?<market_id>")]
pub async fn market_place_delete(
    market_id: i32,
    db: MysqlDbConn,
) -> (Status, Json<CommonRespose<usize>>) {
    let exec_result = db
        .run(move |conn| diesel::delete(market_place.filter(id.eq(market_id))).execute(conn))
        .await;
    (Status::Accepted, Json(CommonRespose::build(exec_result)))
}
