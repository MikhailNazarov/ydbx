use crate::error::YdbxError;

use super::Ydb;


impl Ydb{
    pub(crate) async fn execute_schema_query(&self, query: impl Into<String>)->Result<(), YdbxError>{
        self
            .table_client()
            .retry_execute_scheme_query(query)
            .await?;
        Ok(())
    }
}