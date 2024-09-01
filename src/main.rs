use axum::{
    routing::{
        get,
        post,
    },
    Router,
    Json,
    response::IntoResponse,
    http::StatusCode,
};

use serde::{
    Serialize,
    Deserialize,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/add_order", post(add_order));
    axum::Server::bind(&"127.0.0.1:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn add_order(Json(order): Json<Order>) -> impl IntoResponse {
    let pretty_json_order = serde_json::to_string_pretty(&order).unwrap();
    (StatusCode::OK, pretty_json_order)
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: u32,
    payment_dt: u64,
    bank: String,
    delivery_cost: u32,
    goods_total: u32,
    custom_fee: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Item {
    chrt_id: u64,
    track_number: String,
    price: u32,
    rid: String,
    name: String,
    sale: u32,
    size: String,
    total_price: u32,
    nm_id: u64,
    brand: String,
    status: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: u64,
    date_created: String,
    oof_shard: String,
}
