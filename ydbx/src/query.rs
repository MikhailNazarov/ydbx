use rustring_builder::StringBuilder;

use crate::{error::YdbxError, Ydb};

pub struct YdbQuery<'q> {
    ydb: &'q Ydb,
    query: StringBuilder,
}

impl<'q> YdbQuery<'q> {
    pub(crate) fn new(ydb: &'q Ydb, init: impl AsRef<str>) -> Self {
        Self {
            ydb,
            query: StringBuilder::new_with_value(init.as_ref()),
        }
    }

    pub async fn schema_execute(self)->Result<(), YdbxError>{
        self.ydb
            .execute_schema_query(self.query.to_string())
            .await?;
        Ok(())
    }
}


pub struct YdbCreateTableQuery<'q>{
    ydb: &'q Ydb,
    query: StringBuilder
}

impl<'q> YdbCreateTableQuery<'q>{
    pub(crate) fn new(ydb: &'q Ydb, table: impl AsRef<str>) -> Self {
        Self {
            ydb,
            query: StringBuilder::new_with_value(format!("CREATE TABLE \"{}\" (",table.as_ref())),
        }
    }

    pub async fn schema_execute(mut self)->Result<(), YdbxError>{
        self.query.append(" )");
        self.ydb
            .execute_schema_query(self.query.to_string())
            .await?;
        Ok(())
    }
}