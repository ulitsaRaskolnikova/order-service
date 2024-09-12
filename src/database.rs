use std::error::Error;
use sqlx::{self, postgres::PgPool};
use crate::model::{Order, Delivery, Payment, Item};
use log::info;

pub async fn add_order_to_db(order: &Order, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    info!("Starting to add order to the database: {:?}", order.order_uid);
    let delivery_id = insert_delivery_to_db(&order.delivery, &pool).await?;
    insert_payment_to_db(&order.payment, &pool).await?;
    insert_order_to_db(&order, &pool, delivery_id).await?;
    
    for item in &order.items {
        insert_item_to_db(&item, &pool).await?;
        insert_order_item_to_db(&order, &item, &pool).await?;
    }

    info!("Order successfully added to the database: {:?}", order.order_uid);
    Ok(())
}

async fn insert_delivery_to_db(delivery: &Delivery, pool: &PgPool) -> Result<i64, Box<dyn Error>> {
    info!("Inserting delivery: {:?}", delivery);
    let query = r#"
        INSERT INTO delivery (
            name,
            phone,
            zip,
            city,
            address,
            region,
            email
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING delivery_id
    "#;
    let delivery_id: (i64,) = sqlx::query_as(query) 
        .bind(&delivery.name)
        .bind(&delivery.phone)
        .bind(&delivery.zip)
        .bind(&delivery.city)
        .bind(&delivery.address)
        .bind(&delivery.region)
        .bind(&delivery.email)
        .fetch_one(pool)
        .await?;

    info!("Delivery inserted with ID: {}", delivery_id.0);
    Ok(delivery_id.0)
}

async fn insert_payment_to_db(payment: &Payment, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    info!("Inserting payment: {:?}", payment.transaction);
    let query = r#"
        INSERT INTO payment (
            transaction,
            request_id,
            currency,
            provider,
            amount,
            payment_dt,
            bank,
            delivery_cost,
            goods_total,
            custom_fee
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
    "#;
    sqlx::query(query)
        .bind(&payment.transaction)
        .bind(&payment.request_id)
        .bind(&payment.currency)        
        .bind(&payment.provider)
        .bind(payment.amount)
        .bind(payment.payment_dt)
        .bind(&payment.bank)
        .bind(payment.delivery_cost)
        .bind(payment.goods_total)
        .bind(payment.custom_fee)
        .execute(pool)
        .await?;

    info!("Payment inserted: {}", payment.transaction);
    Ok(())
}

async fn insert_order_to_db(order: &Order, pool: &PgPool, delivery_id: i64) -> Result<(), Box<dyn Error>> {
    info!("Inserting order: {:?}", order.order_uid);
    let query = r#"
        INSERT INTO order_info (
            order_uid,
            track_number,
            entry,
            delivery_id,
            payment_transaction,
            locale,
            internal_signature,
            customer_id,
            delivery_service,
            shardkey,
            sm_id,
            date_created,
            oof_shard
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
    "#;
    sqlx::query(query)
        .bind(&order.order_uid)
        .bind(&order.track_number)
        .bind(&order.entry)
        .bind(delivery_id)
        .bind(&order.payment.transaction)
        .bind(&order.locale)
        .bind(&order.internal_signature)
        .bind(&order.customer_id)
        .bind(&order.delivery_service)
        .bind(&order.shardkey)
        .bind(order.sm_id)
        .bind(&order.date_created)
        .bind(&order.oof_shard)
        .execute(pool)
        .await?;

    info!("Order inserted: {:?}", order.order_uid);
    Ok(())
}

async fn insert_item_to_db(item: &Item, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    info!("Inserting item: {:?}", item.chrt_id);
    let query = r#"
        INSERT INTO item (
            chrt_id,
            track_number,
            price,
            rid,
            name,
            sale,
            size,
            total_price,
            nm_id,
            brand,
            status
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
    "#;
    sqlx::query(query)
        .bind(item.chrt_id)
        .bind(&item.track_number)
        .bind(item.price)
        .bind(&item.rid)
        .bind(&item.name)
        .bind(item.sale)
        .bind(&item.size)
        .bind(item.total_price)
        .bind(item.nm_id)
        .bind(&item.brand)
        .bind(item.status)
        .execute(pool)
        .await?;

    info!("Item inserted: {:?}", item.chrt_id);
    Ok(())
}

async fn insert_order_item_to_db(order: &Order, item: &Item, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    info!("Inserting order_item: order_uid: {:?}, item_chrt_id: {:?}", order.order_uid, item.chrt_id);
    let query = r#"INSERT INTO order_item (order_uid, item_chrt_id) VALUES ($1, $2) "#;
    sqlx::query(query)
        .bind(&order.order_uid)
        .bind(item.chrt_id)
        .execute(pool)
        .await?;

    info!("Order item inserted: order_uid: {:?}, item_chrt_id: {:?}", order.order_uid, item.chrt_id);
    Ok(())
}
