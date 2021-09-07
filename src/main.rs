mod ledger;
mod utils;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/ledger/market-place",
            ledger::controller::routes_market_place(),
        )
        .mount(
            "/ledger/shopping-records",
            ledger::controller::routes_shopping_records(),
        )
        .mount(
            "/ledger/cost-detail",
            ledger::controller::routes_cost_detail(),
        )
        .mount("/ledger/goods", ledger::controller::routes_goods())
        .attach(ledger::MysqlDbConn::fairing())
}
