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

pub fn routes_shopping_records() -> Vec<rocket::Route> {
    routes![
        ctrl_shopping_records::shopping_records_list,
        ctrl_shopping_records::shopping_records_fetch,
        ctrl_shopping_records::shopping_records_update,
        ctrl_shopping_records::shopping_records_delete,
    ]
}

pub fn routes_cost_detail() -> Vec<rocket::Route> {
    routes![
        ctrl_cost_detail::cost_detail_list,
        ctrl_cost_detail::cost_detail_fetch,
        ctrl_cost_detail::cost_detail_related,
        ctrl_cost_detail::cost_detail_update,
        ctrl_cost_detail::cost_detail_delete,
    ]
}

pub fn routes_goods() -> Vec<rocket::Route> {
    routes![
        ctrl_goods::goods_list,
        ctrl_goods::goods_fetch_by_name,
        ctrl_goods::goods_fetch_by_id,
        ctrl_goods::goods_update,
        ctrl_goods::goods_delete,
    ]
}
