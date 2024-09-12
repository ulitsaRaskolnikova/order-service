use std::sync::Arc;
use axum::{Router, routing::{get, post}, extract::State, Json, response::IntoResponse, http::StatusCode};
use tokio::sync::RwLock;
use serde_json::json;
use tokio_postgres::{NoTls, Client};
use dotenvy::dotenv;
use log::info;

mod database;
mod model;
use model::Order;

#[tokio::main]
async fn main() {
    // Инициализируем dotenv
    dotenv().expect("Unable to access .env file");

    // Инициализация log4rs
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    // Логируем запуск сервера
    info!("Starting server...");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8081".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    // Настраиваем подключение к базе данных через tokio_postgres
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Can't connect to database");

    // Запускаем соединение с базой данных в фоновом режиме
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("Database connection error: {}", e);
        }
    });

    let app = Router::new()
        .route("/add_order", post(add_order))
        .route("/get_orders", get(get_orders))
        .with_state(Arc::new(RwLock::new(OrdersState { orders: Vec::new(), client })));

    info!("Listening on {}", server_address);

    axum_server::bind(server_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type OrdersStateType = Arc<RwLock<OrdersState>>;
pub struct OrdersState {
    pub orders: Vec<Order>,
    pub client: Client,
}

pub async fn add_order(
    State(state): State<OrdersStateType>, 
    Json(order): Json<Order>
) -> impl IntoResponse {
    let mut state = state.write().await;
    state.orders.push(order.clone());

    match database::add_order_to_db(&order, &state.client).await {
        Ok(_) => {
            info!("Order added successfully: {:?}", order);
            let pretty_json_order = serde_json::to_string_pretty(&order).unwrap();
            (StatusCode::OK, pretty_json_order)
        }
        Err(e) => {
            let error_response = json!({
                "success": false,
                "message": e.to_string(),
            });
            log::error!("Failed to add order: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, error_response.to_string())
        }
    }
}

async fn get_orders(State(state): State<OrdersStateType>) -> impl IntoResponse {
    let pretty_json_orders = serde_json::to_string_pretty(
        &state.read().await.orders
    ).unwrap();
    info!("Fetched all orders");
    (StatusCode::OK, pretty_json_orders)
}
