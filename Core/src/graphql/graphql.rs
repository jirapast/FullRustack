use std::sync::Arc;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use sea_orm::{Database, DatabaseConnection};

use crate::domain::model::MyObject;
use crate::repo::local_db_orm::{repository_get_item, repository_get_items, repository_get_item_from_local_orm, repository_get_items_from_local_orm};


// GraphQL - Data Model >> domain - models




// GraphQL - Schema

pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn graphql_build_schema() -> GraphQLSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish()
}

pub fn graphql_build_schema_local_orm(db: Arc<DatabaseConnection>) -> GraphQLSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}


// GraphQL - Query

#[derive(Default)]
pub struct Query;


#[Object]
impl Query {
    #[graphql(name = "item")]
    pub async fn item<'ctx>(&self, ctx: &Context<'ctx>) -> MyObject {
        let item = repository_get_item().await;
        item
    }

    #[graphql(name = "items")]
    pub async fn items<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<MyObject> {
        let items = repository_get_items().await;
        items
    }

    #[graphql(name = "local_orm_item")]
    pub async fn local_orm_item <'ctx>(&self, ctx: &Context<'ctx>) -> MyObject {
        let item = repository_get_item_from_local_orm(ctx).await;
        item
    }

    #[graphql(name = "local_orm_items")]
    pub async fn local_orm_items<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<MyObject> {
        let items = repository_get_items_from_local_orm(ctx).await;
        items
    }
}
