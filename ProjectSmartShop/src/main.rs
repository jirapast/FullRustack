use actix_web::web::block;
use futures::executor::block_on;
use sea_orm::{ActiveValue, Database, DatabaseConnection};
use sea_orm::{ConnectionTrait, DbBackend, DbErr, Statement};
use sea_orm::*;
use std::env;
use crate::entities::demo_items::Model; // Add this import

mod entities;
use entities::{prelude::*, *};

const DATABASE_URL: &str = "sqlite://db/mydb.sqlite";
const DB_NAME: &str = "tmp_table";


async fn connect_db() -> DatabaseConnection {
    let db = Database::connect(DATABASE_URL)
        .await
        .expect("Database connection failed");
    db
}

async fn find_row(id: i32, db: &DatabaseConnection) -> Option<Model> {
    let r= DemoItems::find_by_id(id)
        .one(db)
        .await
        .unwrap();
    r 
}

async fn insert_row(row_id: i32, db: &DatabaseConnection) -> Result<(), DbErr> {
    let row = find_row(row_id, &db).await;
    match row {
        Some(_) => {
            println!("the key already exists; cannot insert into key {}", row_id) ;
        }
        None => {
            let to_update= demo_items::ActiveModel {
                id: Set(row_id),
                name: Set("val".to_owned()),
            };
            println!("the key is not exists; can insert into key {:?}", to_update) ;
            to_update.insert(db).await?;
            println!("Successfully inserted key");
        }
    }
    Ok(())
}

async fn delete_row(row_id: i32, db: &DatabaseConnection) -> Result<(), DbErr> {
    let row = find_row(row_id, &db).await;
    match row {
        Some(_) => {
            println!("the key already exists; can delete into key {}", row_id) ;
            let to_update= demo_items::ActiveModel {
                id: Set(row_id),
                name: Set("val".to_owned()),
            };
            to_update.delete(db).await?;
            println!("Successfully deleted key");
        }
        None => {
            println!("the key is not exists; cannot delete into key {:?}", row_id) ;
        }
    }
    Ok(())
}

async fn update_row(row_id: i32, val: &str, db: &DatabaseConnection) -> Result<(), DbErr> {
    let row = find_row(row_id, &db).await;
    match row {
        Some(_) => {
            println!("the key already exists; canupdate into key {}", row_id) ;
            let to_update= demo_items::ActiveModel {
                id: Set(row_id),
                name: Set(val.to_owned()),
            };
            to_update.update(db).await?;
            println!("Successfully updateed key");
        }
        None => {
            println!("the key is not exists; cannot update into key {:?}", row_id) ;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // connect
    let db = connect_db().await;
        
    // select
    let row = find_row(5, &db).await;
    println!("row {:?}", row);

    // insert
    insert_row(5, &db).await;

    // delete
    delete_row(5, &db).await;

    // update
    update_row(3, "<><><>", &db).await;
}
