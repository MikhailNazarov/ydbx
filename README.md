# Rust library for YDB

This is experimental library wrapper for [YDB rust sdk](https://github.com/ydb-platform/ydb-rs-sdk) for more convenient usage of YDB in Rust.

**!!!! WARNING !!!!**
It's in heavy development. Please do not use it in production.

## Connect to database
```rust
let connection_string = env::var("YDB_CONNECTION_STRING")
        .unwrap_or_else(|_| "grpc://localhost:2136?database=/local".to_string());
let ydb: Ydb = Ydb::connect(connection_string).await
    .expect("Error connecting to YDB");

```

## Execute schema queries

```rust
    
    ydb.query(r#"
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
```

## Checklist

- [x] Connect to ydb
- [x] Default credentials (using fromEnv)
- [x] Custom credentials with options
- [ ] Basic query
- [ ] Binding parameters
- Support types
    - Numeric
        - [ ] Bool	
        - [ ] Int8 	
        - [ ] Int16 	
        - [ ] Int32 	
        - [ ] Int64 	
        - [ ] Uint8 	
        - [ ] Uint16 	
        - [ ] Uint32 	
        - [ ] Uint64
        - [ ] Float 
        - [ ] Double 	
        - [ ] Decimal 
    - String types
        - [ ] String
        - [ ] Utf8
        - [ ] Json
        - [ ] JsonDocument
        - [ ] Yson
        - [ ] Uuid
    - Date and time
        - [ ] Date
        - [ ] Datetime
        - [ ] Timestamp
        - [ ] Interval
    - [ ] Optional
- [ ] Prepared statements
- [ ] Transactions
- [ ] Compile-type checked queries
- [ ] Migrations
- [ ] Log Statements
