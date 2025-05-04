use tokio;
use actix_web::web::block;
use futures::executor::block_on;

use sea_orm::*;
use async_graphql::{Schema, EmptyMutation, EmptySubscription, Object, SimpleObject};
use std::sync::Arc;

mod entities;
use entities::{prelude::*, *};
use entities::demo_items::Model; // Add this import


mod domain;
mod repo;
mod graphql;


use graphql::graphql::{graphql_build_schema, graphql_build_schema_local_orm, GraphQLSchema };

mod services;
use crate::services::sea_orm_ark;


#[tokio::main]
async fn main() {
}


#[cfg(test)]
mod tests {
    use serde::de::Expected;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_graphql_simple_0() {
        let schema: GraphQLSchema = graphql_build_schema();
        let res = schema.execute("{ item { id name } }").await;
        let data = res.data.into_json().unwrap();
        let item = &data["item"];
        assert_eq!(item["id"], 0);
        assert_eq!(item["name"], "item_name");
    }

    #[tokio::test]
    async fn test_graphql_simple_1() {
        let schema: GraphQLSchema = graphql_build_schema();
        let res = schema.execute("{ items { id name } }").await;
        let data = res.data.into_json().unwrap();
        let item = &data["items"];
        let expected_result = json!([
            { "id": 0, "name": "item_0_name" },
            { "id": 1, "name": "item_1_name" }
        ]);
        assert_eq!(*item, expected_result);
    }

    #[tokio::test]
    async fn test_graphql_local_db_orm_0() {
        let db: DatabaseConnection = sea_orm_ark::connect_db().await;
        let arc_db: Arc<DatabaseConnection> = Arc::new(db);
        let clone_arc_db: Arc<DatabaseConnection> = arc_db.clone();

        let schema: GraphQLSchema = graphql_build_schema_local_orm(clone_arc_db);
        let res = schema.execute("{ local_orm_item {id name} }").await;

        let data = res.data.into_json().unwrap();
        let item = &data["local_orm_item"];
        let expected_result = json!({
            "id": 0,
            "name": "new_name",
        });
        
        assert_eq!(*item, expected_result);
    }

    #[tokio::test]
    async fn test_graphql_local_db_orm_1() {
        let db: DatabaseConnection = sea_orm_ark::connect_db().await;
        let arc_db: Arc<DatabaseConnection> = Arc::new(db);
        let clone_arc_db: Arc<DatabaseConnection> = arc_db.clone();

        let schema: GraphQLSchema = graphql_build_schema_local_orm(clone_arc_db);
        let res = schema.execute("{ local_orm_items {id name} }").await;

        let data = res.data.into_json().unwrap();
        let item = &data["local_orm_items"];

        let expected_result = json!([
            { "id": 0, "name": "new_name", },
            { "id": 0, "name": "new_name", }
        ]);
        
        assert_eq!(*item, expected_result);
    }

}
// 