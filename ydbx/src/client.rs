use tonic::transport::Channel;
use ydbx_grpc::ydb::{operations::OperationParams, table::{v1::table_service_client::TableServiceClient, CreateSessionRequest}};

use crate::error::YdbxError;

#[derive(Clone, Debug)]
pub struct Ydbx {
    pub(crate) table_client: TableServiceClient<Channel>,
}

impl Ydbx{
    pub async fn connect(connection_string: String) -> Result<Self, YdbxError> {
        let channel = Channel::from_shared(connection_string)
            .map_err(|e| YdbxError::ConnectionError(e.to_string()))?
            .connect().await
            .map_err(|e| YdbxError::ConnectionError(e.to_string()))?;
        let table_client = TableServiceClient::new(channel);
        Ok(Self { table_client })
    }

    pub async fn create_session(&self)-> Result<YdbxSession, YdbxError>{

        let mut client = self.clone();
        client.table_client.create_session(CreateSessionRequest::default()).await.map_err(|e| YdbxError::SessionError(e.to_string()))?;

        Ok(YdbxSession::new(client))
    }
}

pub struct YdbxSession{
    ydbx: Ydbx
}

impl YdbxSession{

    pub fn new(ydbx: Ydbx) -> Self {
        Self { ydbx }
    }
}