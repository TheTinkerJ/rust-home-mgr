mod ctrl_cost_detail;
mod ctrl_goods;
mod ctrl_market_place;
mod ctrl_shopping_records;

pub fn routes_market_place() -> Vec<rocket::Route> {
    routes![
        ctrl_market_place::market_place_list,
        ctrl_market_place::market_place_fetch,
        ctrl_market_place::market_place_update,
        ctrl_market_place::market_place_delete,
    ]
}
