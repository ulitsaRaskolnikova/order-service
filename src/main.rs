use std::sync::Arc;
use axum::{Router, routing::{get, post}, extract::State, Json, response::IntoResponse, http::StatusCode};
use tokio::sync::RwLock;
use serde_json::json;
use tokio_postgres::{NoTls, Client};
use log::{info, error};

mod database;
mod model;
use model::Order;
use clap::{self, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Your server host
    #[clap(long, env)]
    server_host: String,
    /// Your host port
    #[clap(long, env)]
    server_port: u16,
    /// Database user
    #[clap(long, env)]
    db_user: String,
    /// Database user's password
    #[clap(long, env)]
    db_password: String,
    /// Name of database
    #[clap(long, env)]
    db_name: String,
    /// Your database host
    #[clap(long, env)]
    db_host: String,
    /// Your database port
    #[clap(long, env)]
    db_port: u16
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Инициализация log4rs
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    // Логируем запуск сервера
    info!("Starting server...");

    let args = Args::parse();

    let server_address = format!("{}:{}", args.server_host, args.server_port);
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        args.db_user, args.db_password, args.db_host, args.db_port, args.db_name
    );
    // Настраиваем подключение к базе данных через tokio_postgres
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Can't connect to database");

    // Запускаем соединение с базой данных в фоновом режиме
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Database connection error: {}", e);
        }
    });

    let app = Router::new()
    .route("/add_order", post(add_order))
    .route("/get_orders", get(get_orders))
    .with_state(Arc::new(RwLock::new(
        OrdersState { 
            orders: match database::get_all_orders(&client).await {
                Ok(orders) => {
                    info!("Successfully get orders: {:?}", orders);
                    orders
                }
                Err(e) => {
                    error!("Failed to get orders: {:?}", e);
                    Vec::new() // Возвращаем пустой вектор в случае ошибки
                }
            },
            client
        }
    )));

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
    
    match database::save_order(&order, &state.client).await {
        Ok(_) => {
            info!("Order added successfully: {:?}", order);
            state.orders.push(order.clone());
            let pretty_json_order = serde_json::to_string_pretty(&order).unwrap();
            (StatusCode::OK, pretty_json_order)
        }
        Err(e) => {
            let error_response = json!({
                "success": false,
                "message": e.to_string(),
            });
            error!("Failed to add order: {:?}", e);
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
