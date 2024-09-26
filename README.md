# Rust library for YDB

This is experimental library wrapper for [YDB rust sdk](https://github.com/ydb-platform/ydb-rs-sdk) for more convenient usage of YDB in Rust.

**!!!! WARNING !!!!**
It's in heavy development. Please do not use it in production.

## Checklist

- [ ] Connect to ydb
- [ ] Default credentials (using fromEnv)
- [ ] Custom credentials with options
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
