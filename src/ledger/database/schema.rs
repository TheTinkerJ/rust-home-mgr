table! {
  /// 消费场所
  market_place(id){
    id->Nullable<Integer>,
    gmt_create->Nullable<Timestamp>,
    gmt_modified->Nullable<Timestamp>,
    name->Varchar,
    lng->Nullable<Varchar>,
    lat->Nullable<Varchar>,
    location_desc->Nullable<Varchar>,
    adcode->Nullable<Varchar>,
  }
}

table! {
  /// 消费记录
  shopping_records(id){
    id->Nullable<Integer>,
    gmt_create->Nullable<Timestamp>,
    gmt_modified->Nullable<Timestamp>,
    place_id->Integer,
    comment->Nullable<Varchar>,
  }
}

table! {
  /// 消费明细
  cost_detail(id){
    id->Nullable<Integer>,
    gmt_create->Nullable<Timestamp>,
    gmt_modified->Nullable<Timestamp>,
    shopping_id->Integer,
    good_id->Integer,
    cost->Integer,
    count->Integer,
    comment->Nullable<Varchar>,
  }
}

table! {
  /// 消费条目
  goods(id){
    id->Nullable<Integer>,
    gmt_create->Nullable<Timestamp>,
    gmt_modified->Nullable<Timestamp>,
    name->Varchar,
    cls1->Nullable<Varchar>,
    cls2->Nullable<Varchar>,
    cls3->Nullable<Varchar>,
  }
}
