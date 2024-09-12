use std::error::Error;
use tokio_postgres::Client;
use crate::model::{Order, Delivery, Payment, Item};
use log::info;

pub async fn add_order_to_db(order: &Order, client: &Client) -> Result<(), Box<dyn Error>> {
    info!("Starting to add order to the database: {:?}", order.order_uid);

    let delivery_id = insert_delivery_to_db(&order.delivery, client).await?;
    insert_payment_to_db(&order.payment, client).await?;
    insert_order_to_db(&order, client, delivery_id).await?;

    for item in &order.items {
        insert_item_to_db(&item, client).await?;
        insert_order_item_to_db(&order, &item, client).await?;
    }

    info!("Order successfully added to the database: {:?}", order.order_uid);
    Ok(())
}

async fn insert_delivery_to_db(delivery: &Delivery, client: &Client) -> Result<i64, Box<dyn Error>> {
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

    let row = client.query_one(query, &[
        &delivery.name, 
        &delivery.phone, 
        &delivery.zip, 
        &delivery.city, 
        &delivery.address, 
        &delivery.region, 
        &delivery.email
    ]).await?;

    let delivery_id: i64 = row.get(0);

    info!("Delivery inserted with ID: {}", delivery_id);
    Ok(delivery_id)
}

async fn insert_payment_to_db(payment: &Payment, client: &Client) -> Result<(), Box<dyn Error>> {
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

    client.execute(query, &[
        &payment.transaction,
        &payment.request_id,
        &payment.currency,
        &payment.provider,
        &payment.amount,
        &payment.payment_dt,
        &payment.bank,
        &payment.delivery_cost,
        &payment.goods_total,
        &payment.custom_fee,
    ]).await?;

    info!("Payment inserted: {}", payment.transaction);
    Ok(())
}

async fn insert_order_to_db(order: &Order, client: &Client, delivery_id: i64) -> Result<(), Box<dyn Error>> {
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

    client.execute(query, &[
        &order.order_uid,
        &order.track_number,
        &order.entry,
        &delivery_id,
        &order.payment.transaction,
        &order.locale,
        &order.internal_signature,
        &order.customer_id,
        &order.delivery_service,
        &order.shardkey,
        &order.sm_id,
        &order.date_created,
        &order.oof_shard,
    ]).await?;

    info!("Order inserted: {:?}", order.order_uid);
    Ok(())
}

async fn insert_item_to_db(item: &Item, client: &Client) -> Result<(), Box<dyn Error>> {
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

    client.execute(query, &[
        &item.chrt_id,
        &item.track_number,
        &item.price,
        &item.rid,
        &item.name,
        &item.sale,
        &item.size,
        &item.total_price,
        &item.nm_id,
        &item.brand,
        &item.status,
    ]).await?;

    info!("Item inserted: {:?}", item.chrt_id);
    Ok(())
}

async fn insert_order_item_to_db(order: &Order, item: &Item, client: &Client) -> Result<(), Box<dyn Error>> {
    info!("Inserting order_item: order_uid: {:?}, item_chrt_id: {:?}", order.order_uid, item.chrt_id);

    let query = r#"INSERT INTO order_item (order_uid, item_chrt_id) VALUES ($1, $2)"#;

    client.execute(query, &[&order.order_uid, &item.chrt_id]).await?;

    info!("Order item inserted: order_uid: {:?}, item_chrt_id: {:?}", order.order_uid, item.chrt_id);
    Ok(())
}
