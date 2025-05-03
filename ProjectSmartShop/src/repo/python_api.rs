use std::sync::Arc;
use sea_orm::{Database, DatabaseConnection};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};

use crate::services::sea_orm_ark;
use crate::domain::model::{MyObject, MyGraphQLObject};


// pub async fn repository_get_item() -> MyObject {
    // MyGraphQLObject::get_item().await
// }



// pub async fn repository_get_items() -> Vec<MyObject> {
    // MyGraphQLObject::get_items().await
// }

pub async fn repository_get_item_from_python_api() -> MyObject {
    // let db: &'ctx Arc<DatabaseConnection> = ctx.data_unchecked();
    // let row: Option<crate::entities::demo_items::Model> = sea_orm_ark::find_row(0, &db).await;
    // let (id, name) = match row {
    //     Some(crate::entities::demo_items::Model {id, name}) => (id, name),
    //     None => return MyObject {
    //         id: 0,
    //         name: "default".to_string()
    //     }
    // };
    MyObject { id: 0, name: "ddd".to_string() }
}

