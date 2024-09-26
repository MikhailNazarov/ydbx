use std::env;

use serde::{Deserialize, Serialize};
use tracing::error;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use ydbx::{connection::Ydb, error::YdbxError};

#[tokio::main]
async fn main() -> Result<(), YdbxError> {
    dotenvy::dotenv().ok();
    init_logs();

    if let Err(e) = run().await{
        error!("{:?}",e);
    }

    Ok(())
}

async fn run()->Result<(), YdbxError>{
    let connection_string = env::var("YDB_CONNECTION_STRING")
        .unwrap_or_else(|_| "grpc://localhost:2136?database=/local".to_string());
    let ydb: Ydb = Ydb::connect(connection_string).await
        .expect("Error connecting to YDB");

    // ydb
    //     .query("SELECT 1+1")
    //     .fetch_one()
    //     .await?;

    ydb
        .query(r#"
            CREATE TABLE test (
                id Uint64 NOT NULL,
                name Utf8,
                age UInt32 NOT NULL,
                description Utf8,
                PRIMARY KEY (id)
            );
        "#)
        .schema_execute()
        .await?;

    // ydb
    //     .create("test2")
    //     .schema_execute()
    //     .await?;

    // let test_user_info = UserInfo {
    //     id: 1,
    //     name: "test".to_string(),
    //     age: 33,
    //     description: None
    // };

    // ydb
    //     .query(
    //         r#"
    //             INSERT INTO test2 (id, name, age, description) 
    //             VALUES ( $arg_1, $arg_2, $age, $arg_3)
    //         "#)
    //     .bind(test_user_info.id)
    //     .bind(test_user_info.name)
    //     .bind_named("age", test_user_info.age)
    //     .bind(test_user_info.description)
    //     .execute()
    //     .await?;

    // let users: Vec<UserInfo> =
    //     ydb
    //         .query(
    //         r#"
    //             SELECT * FROM test2 WHERE age > $age AND age < $arg_1
    //         "#)
    //         .bind_named("age", 30)
    //         .bind(40)
    //         .fetch_all()
    //         .await?;

    // assert!(users.len() > 0);
    // info!("users found: {}", users.len());



    Ok(())
}


#[derive(Serialize, Deserialize)]
struct UserInfo {
    id: u64,
    name: String,
    age: u32,
    description: Option<String>,
}

pub fn init_logs() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}
