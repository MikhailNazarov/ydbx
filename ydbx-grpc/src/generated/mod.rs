// This file is @generated by prost-build.
pub mod ydb {
    include!("ydb.rs");
    pub mod auth {
        include!("ydb.auth.rs");
        pub mod v1 {
            include!("ydb.auth.v1.rs");
        }
    }
    pub mod coordination {
        include!("ydb.coordination.rs");
        pub mod v1 {
            include!("ydb.coordination.v1.rs");
        }
    }
    pub mod discovery {
        include!("ydb.discovery.rs");
        pub mod v1 {
            include!("ydb.discovery.v1.rs");
        }
    }
    pub mod formats {
        include!("ydb.formats.rs");
    }
    pub mod issue {
        include!("ydb.issue.rs");
    }
    pub mod operations {
        include!("ydb.operations.rs");
    }
    pub mod scheme {
        include!("ydb.scheme.rs");
        pub mod v1 {
            include!("ydb.scheme.v1.rs");
        }
    }
    pub mod table {
        include!("ydb.table.rs");
        pub mod v1 {
            include!("ydb.table.v1.rs");
        }
    }
    pub mod table_stats {
        include!("ydb.table_stats.rs");
    }
    pub mod topic {
        include!("ydb.topic.rs");
        pub mod v1 {
            include!("ydb.topic.v1.rs");
        }
    }
}
