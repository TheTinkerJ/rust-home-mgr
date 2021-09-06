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
        .attach(ledger::MysqlDbConn::fairing())
}
