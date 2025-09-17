pub const WORKSPACE_CARGO_TOML: &'static str = r#"
[package]
name = "udfs"
version = "0.1.0"
edition = "2024"

# This is the Cargo.toml associated with your workspace
# DO NOT REMOVE THE sentinel-macros crate.

[dependencies]
sentinel-macros = { path = "../sentinel-macros" }
"#;

pub const WORKSPACE_MANIFEST_TOML: &'static str = r#"
# This is your pipline manifest where you define your pipeline as a DAG
# THis is just an example. Use the docs to figure out how to build a DAG
# exactly how you want it.

[[operators]]
id = "read_input"
udf_name = "source_udf"
type = "source"

[operators.config]
type = "path"
path = "input.csv"


[[operators]]
id = "transform_data"
udf_name = "transform_udf"
depends = ["read_input"]
type = "transform"


[[operators]]
id = "write_output"
udf_name = "sink_udf"
depends = ["transform_data"]
type = "sink"

[operators.config]
type = "path"
path = "output.csv"

"#;

pub const WORKSPACE_LIB_RS: &'static str = r#"
use sentinel_macros::sentinel_udf;
use arrow::record_batch::RecordBatch;

// This is your

#[sentinel_udf]
async fn my_source_udf(batch: RecordBatch) -> RecordBatch {
    // Your source logic here...
}

#[sentinel_udf]
async fn my_transform_udf(batch: RecordBatch) -> RecordBatch {
    // Your transform logic here...
}

#[sentinel_udf]
async fn sink(batch: RecordBatch) -> RecordBatch {
    // Your sink logic here...
}
"#;
