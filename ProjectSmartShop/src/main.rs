// use actix_web::web::block;
use futures::executor::block_on;
use sea_orm::{Database, DatabaseConnection};
use sea_orm::{ConnectionTrait, DbBackend, DbErr, Statement};


const DATABASE_URL: &str = "sqlite://db/mydb.sqlite?mode=ro";
const DB_NAME = "tmp_table";

async fn run() -> Result<(), DbErr> {
    let db: DatabaseConnection = Database::connect(DATABASE_URL).await?;
    println!("Database connected successfully!");

    // 
    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
+           db.execute(Statement::from_string(
+               db.get_database_backend(),
+               format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
+           ))
+           .await?;
+
+           let url = format!("{}/{}", DATABASE_URL, DB_NAME);
+           Database::connect(&url).await?
+       }

        DbBackend::Sqlite => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            )).await?;
        }
        let url = format!("{}/{}", DATABASE_URL, DB_NAME);
        Database::connect(&url).await?

// +       DbBackend::Postgres => {
// +           db.execute(Statement::from_string(
// +               db.get_database_backend(),
// +               format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
// +           ))
// +           .await?;
// +           db.execute(Statement::from_string(
// +               db.get_database_backend(),
// +               format!("CREATE DATABASE \"{}\";", DB_NAME),
// +           ))
// +           .await?;
// +
// +           let url = format!("{}/{}", DATABASE_URL, DB_NAME);
// +           Database::connect(&url).await?

        // 

+       }
        BDBackedend::Sqlite => db
    };

    // 

    Ok(())
}

// #[derive(Iden)]
enum Item {
    id,
    name,
}

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
