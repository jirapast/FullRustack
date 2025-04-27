// pub mod prelude;
// pub mod user;
// use crate::users:ActiveModel as UserActiveModel;


// #[derive(Debug, Serialize)]
// struct User {
//     id: String,
//     name: String
// }

// async fn insert_new_user(
//     conn: &DatabaseConnection,
//     user_name: String,
// ) -> Result<User, sea_orm::DbErr> {

// }

// #[actix_web::main]
// async fn main() -> io::Result<()> {
//     let database_url = "sqlite:app.db";
//     let conn = Database::connect(database_url)
//         .await
//         .expect("Failed to connect to database");
//     HttpServer::new(move || {
//         App:new()
//             .app_data(web::Data::new(conn.clone()))
//             .route("/{name}", web::get().to(index))
//     })
//     .bind(("127.0.0.1", 8888))?
//     .run()
//     .await
// }