
use tokio;
use serde_json;
use async_graphql::*;

use std::sync::Arc;
use sea_orm_ark::connect_db;


#[derive(SimpleObject)]
struct Book {
    title: String,
    author: String,
}

struct BookService;

impl BookService {
    async fn get_book() -> Book {
        Book {
            title: "title4".to_string(),
            author: "author4".to_string(),
        }
    }
    async fn get_books() -> Vec<Book> {
        vec![
            Book {
                title: "title1".to_string(),
                author: "author1".to_string(),
            },
            Book {
                title: "title2".to_string(),
                author: "author2".to_string(),
            },
        ]
    }
}

struct Query;

#[Object]
impl Query {
    async fn book(&self) -> Book {
        BookService::get_book().await
    }
    async fn books(&self) -> Vec<Book> {
        BookService::get_books().await
    }
}


type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

fn build_schema() -> AppSchema {
    // sea_orm - connect
    let db = connect_db().await;
    let arc_db = Arc::new(db);
    let clone_arc_db = arc_db.clone();

    // let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(clone_arc_db)
        .finish()
}