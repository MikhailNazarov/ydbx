
#[cfg(test)]
mod prelude{
    pub use std::str::FromStr;
    pub use ydbx::*;
    pub use std::env;
}


#[tokio::test]
pub async fn test_connect_from_str(){
    use prelude::*;

    let connection_string = env::var("YDB_CONNECTION_STRING").expect("YDB_CONNECTION_STRING not set");
    let ydb = Ydb::connect(connection_string).await;
    assert!(ydb.is_ok());    
}



#[tokio::test]
pub async fn test_connect_from_opts(){
    use prelude::*;

    let connection_string = std::env::var("YDB_CONNECTION_STRING").expect("YDB_CONNECTION_STRING not set");
    let opts = ydbx::connection::YdbConnectOptions::from_str(&connection_string);
    assert!(opts.is_ok());
    let opts = opts.unwrap();

    let ydb = ydbx::Ydb::connect(opts).await;
    assert!(ydb.is_ok());    
}