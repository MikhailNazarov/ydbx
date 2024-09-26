use std::{env, str::FromStr};

use ydbx::connection::{YdbConnectOptions, YdbConnection};



struct TestContext{
    connection: YdbConnection
}

impl TestContext{
    pub async fn new()->TestContext{
        let connection_string = env::var("YDB_CONNECTION_STRING").unwrap();
        let connection = YdbConnectOptions::from_str(&connection_string).expect("Error parsing connection string")
            .connect().await.unwrap();
        TestContext{connection}
    }

   

   
}



#[tokio::test]
pub async fn test_opt(){
    let ctx = TestContext::new().await;

    
    // sqlx::query(r#"
    //     CREATE TABLE IF NOT EXISTS test_opt(
    //         id Int32 NOT NULL,
    //         title Utf8,
    //         PRIMARY KEY (id)
    //     )
    // "#).execute(conn.schema()).await.unwrap();
   
    
    // sqlx::query(r#"
    //     insert into test_opt(id, title) values
    //     (1, 'title1'),
    //     (2, 'title2'),
    //     (3, 'title3')
    // "#).execute(&mut *conn).await.unwrap();
    
    // {
    // let row = sqlx::query_as::<_,(i32, Option<String>)>(r#"
    //     select * from test_opt where id = $id
    // "#).bind(with_name("id", 1))
    // .fetch_one(&mut *conn).await.unwrap();
    
    // assert_eq!(row.1, Some("title1".to_string()));
    //   }
}



#[tokio::test]
pub async fn test_explain(){
    let ctx = TestContext::new().await;

    // let mut tr = ctx.pool.begin().await.unwrap();
    // let conn = tr.acquire().await.unwrap();
   
    // sqlx::query(r#"
    //     CREATE TABLE IF NOT EXISTS test_opt(
    //         id Int32 NOT NULL,
    //         title Utf8,
    //         PRIMARY KEY (id)
    //     )
    // "#).execute(conn.schema()).await.unwrap();
   
    
    // sqlx::query(r#"
    //     insert into test_opt(id, title) values
    //     (1, 'title1'),
    //     (2, 'title2'),
    //     (3, 'title3')
    // "#).execute(&mut *conn).await.unwrap();
    
    // {
    //  let res = (&mut *conn).describe(r#"
    //     declare $id as Int64;
    //     select id, title from test_opt where id = $id;
    // "#).await;
   
    
    // // assert!(res.is_ok(),"{}",res.err().unwrap());

    // //let res = (&mut *conn).prepare("select id, title from test_opt where id = 1").await;
   
    
    // assert!(res.is_ok(),"{}",res.err().unwrap());
    
    
    // }

}
