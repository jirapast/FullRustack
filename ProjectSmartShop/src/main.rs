use tokio;
use serde_json;
use actix_web::web::block;
use futures::executor::block_on;

use sea_orm::*;
use async_graphql::*;
use async_graphql::{Schema, EmptyMutation, EmptySubscription, Object};


use std::env;
use std::sync::Arc;

mod entities;
use entities::{prelude::*, *};
// use entities::demo_items::Model; // Add this import
use entities::demo_items::{Model}; // Add this import

use async_graphql::SimpleObject;

// mod services {
//     pub mod sea_orm_ark;
// }
// use services::sea_orm_ark::connect_db;

// /--------------------------------const DATABASE_URL: &str = "sqlite://db/mydb.sqlite";

const DATABASE_URL: &str = "sqlite://db/mydb.sqlite";

const DB_NAME: &str = "tmp_table";


#[derive(SimpleObject)]
pub struct ModelObject {
    pub id: i32,
    pub name: String,
}

impl From<crate::entities::demo_items::Model> for ModelObject {
    fn from(model: crate::entities::demo_items::Model) -> Self {
        ModelObject {
            id: model.id,
            name: model.name,
        }
    }
}



pub async fn connect_db() -> DatabaseConnection {
    let db = Database::connect(DATABASE_URL)
        .await
        .expect("Database connection failed");
    db
}
// use crate::entities::demo_items::{Model};

pub async fn find_row(id: i32, db: &DatabaseConnection) -> Option<Model> {
    Some(Model {
        id: 0,
        name: "test".to_string(),
    })
    // Book {
    //     title: "title4".to_string(),
    //     author: "author4".to_string(),
    // }
    // let r = DemoItems::find_by_id(id)
    //     .one(db)
    //     .await
    //     .unwrap();
    // r 
}
// /--------------------------------const DATABASE_URL: &str = "sqlite://db/mydb.sqlite";
// /--------------------------------const DATABASE_URL: &str = "sqlite://db/mydb.sqlite";
// /--------------------------------const DATABASE_URL: &str = "sqlite://db/mydb.sqlite";

// #[derive(SimpleObject)]
// struct Book {
//     title: String,
//     author: String,
// }
// #[derive(Default)]
// struct Model {
//     title: String,
//     author: String,
// }

struct ModelService;

impl ModelService {
    // async fn get_book(db: &Arc<DatabaseConnection>) -> Model {
    async fn get_book(db: &Arc<DatabaseConnection>) -> Option<ModelObject> {
        println!(".............<<<<<<<<<<<<<<<<>>>>>>>>>>>>>>>>");
        // Model {
        //     title: "title4".to_string(),
        //     author: "author4".to_string(),
        // }
        // let row: Option<Model> = find_row(1, &db).await;
        // row
        let row: Option<crate::entities::demo_items::Model> = find_row(1, db).await;
        row.map(|m| m.into()) // convert into ModelObject
        // match row {
        //     Some(row) => row,
        //     None => Model { title: "default".to_string(), author: "default".to_string() }
        // }
    }
    // async fn get_books(db: &Arc<DatabaseConnection>) -> Vec<Model> {
    //     vec![
    //         Model {
    //             title: "title1".to_string(),
    //             author: "author1".to_string(),
    //         },
    //         Model {
    //             title: "title2".to_string(),
    //             author: "author2".to_string(),
    //         },
    //     ]
    // }
}

#[derive(Default)]
struct Query;

#[Object]
impl Query {
    // async fn book<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Model, async_graphql::Error> {
    async fn book<'ctx>(&self, ctx: &'ctx Context<'_>) -> Result<ModelObject> {
        let db: &Arc<DatabaseConnection> = ctx.data_unchecked();
        let res = ModelService::get_book(&db).await;
        match res {
            Some(res) => Ok(res),
            None => Err("Book not found".into()),
        }
    }
    // async fn book(&self) -> Book {
    //     BookService::get_book().await
    // }
    // async fn books(&self) -> Vec<Model> {
        // ModelService::get_books(&db).await
    // }
    // async fn books(&self) -> Vec<Book> {
    //     BookService::get_books().await
    // }
}


type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

fn build_schema(db: Arc<DatabaseConnection>) -> AppSchema {
    // let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    // Schema::build(Query, EmptyMutation, EmptySubscription)
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}

#[tokio::main]
async fn main() {
    // sea_orm - connect
    // let db: DatabaseConnection = connect_db().await;
    let db: DatabaseConnection = connect_db().await;
    let arc_db: Arc<DatabaseConnection> = Arc::new(db);
    let clone_arc_db: Arc<DatabaseConnection> = arc_db.clone();
    println!("-----");

    let schema: AppSchema = build_schema(clone_arc_db);
    println!("{}", schema.sdl());


    let res = schema.execute("{ book }").await;
    let json = serde_json::to_string(&res);
    println!(">> {:?}", json);
}
