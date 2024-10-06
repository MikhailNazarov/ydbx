const PROTO_FILES: &[&str] = &[
    "proto/ydb_auth_v1.proto",
    "proto/ydb_coordination_v1.proto",
    "proto/ydb_discovery_v1.proto",
    "proto/ydb_scheme_v1.proto",
    "proto/ydb_table_v1.proto",
    "proto/ydb_topic_v1.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/generated")
        .include_file("mod.rs")
        .compile_protos(PROTO_FILES, &["proto", "proto/protos"])?;
    Ok(())
}
