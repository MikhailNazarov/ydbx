mod connection_impl;
//mod executor;
//pub mod schema_executor;

use std::fmt;
use std::ops::Deref;
use std::{str::FromStr, sync::Arc, time::Duration};

use crate::error::YdbxError;

//use self::schema_executor::YdbSchemaExecutor;



use url::Url;
use ydb::{AccessTokenCredentials, AnonymousCredentials, MetadataUrlCredentials, ServiceAccountCredentials, StaticCredentials};
use ydb::Credentials;

#[allow(unused)]
pub struct YdbConnection{
    client: ydb::Client,
    pub(crate) transaction: Option<Box<dyn ydb::Transaction>>
}
impl fmt::Debug for YdbConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("YdbConnection")
    }
}

impl Deref for YdbConnection {
    type Target = ydb::Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl YdbConnection{
    // pub fn schema(&self)->YdbSchemaExecutor{
    //     YdbSchemaExecutor::new(self.client.table_client())
    // }


    pub async fn close(self) -> Result<(), YdbxError> {
        Ok(())
    }


    pub async fn ping(&mut self) -> Result<(), YdbxError> {
        
        //todo: validate connection
        // Box::pin(async{
        //     self.client.table_client().keepalive().await
        //     .map_err(|_|sqlx_core::error::Error::PoolClosed)
            
        // })
        Ok(())
    }

    // fn begin(
    //     &mut self,
    // ) -> futures_util::future::BoxFuture<
    //     '_,
    //     Result<sqlx_core::transaction::Transaction<'_, Self::Database>, sqlx_core::Error>,
    // >
    // where
    //     Self: Sized,
    // {
    //     Transaction::begin(self)
    // }

}

#[allow(unused)]
#[derive(Clone, Debug, Default)]
pub struct YdbConnectOptions {
    connection_string: String,
    connection_timeout: Duration,
    credentials: Option<Arc<Box<dyn Credentials>>>,
    
}



impl YdbConnectOptions {

    pub fn from_url(url: &url::Url) -> Result<Self, YdbxError> {
        Self::from_str(url.as_str())
    }

    pub async fn connect(&self ) -> Result<YdbConnection, YdbxError>{
        let connection = YdbConnection::establish(&self).await?;
        Ok(connection)
    }

    pub fn log_statements(self, _level: tracing::log::LevelFilter) -> Self {
        todo!()
    }

    pub fn log_slow_statements(
        self,
        _level: tracing::log::LevelFilter,
        _duration: std::time::Duration,
    ) -> Self {
        todo!()
    }
}


impl FromStr for YdbConnectOptions {
    type Err = YdbxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut options = Self::default();
        options.connection_timeout = Duration::from_secs(2);
        
        let url = Url::parse(s)?;
        let mut user = None;
        let mut password = None;
        let mut database = None; 

        for (k,v) in url.query_pairs(){
            match k.as_ref(){
               
                "connection_timeout" => {
                    let timeout = v.parse::<u64>()?;
                    options.connection_timeout = Duration::from_secs(timeout);
                },
                "sa-key" => {
                    let sa = ServiceAccountCredentials::from_file(v.as_ref())?;
                    options.credentials = Some(Arc::new(Box::new(sa)));
                },
                "anonymous" =>{
                    let cred = AnonymousCredentials::new();
                    options.credentials = Some(Arc::new(Box::new(cred)));
                },
                "metadata" =>{
                    let cred = MetadataUrlCredentials::new();
                    options.credentials = Some(Arc::new(Box::new(cred)));
                }
                "token" =>{
                    let cred = AccessTokenCredentials::from(v.as_ref());
                    options.credentials = Some(Arc::new(Box::new(cred)));
                },
                "database" =>{
                    database = Some(v.to_owned());
                },
                "user" =>{
                    user = Some(v.to_owned());
                },
                "password"=>{
                    password = Some(v.to_owned());
                },
                _ => continue
            }
        }
        let database = database.unwrap_or("/".into()).to_string();
        
        let endpoint = format!("{}://{}:{}?database={}",url.scheme(),url.host().unwrap(),url.port().unwrap(),database);
        if let (Some(user), Some(password)) = (user, password) {
            let password = password.to_string();
            let uri = http::Uri::from_str(&endpoint).unwrap();
            let user = user.to_string();
            let cred = StaticCredentials::new(user, password, uri, database);
            options.credentials = Some(Arc::new(Box::new(cred)));
        }
        options.connection_string = endpoint;
        Ok(options)
    }

}

impl AsMut<YdbConnection> for YdbConnection {
    fn as_mut(&mut self) -> &mut YdbConnection {
        self
    }
}
