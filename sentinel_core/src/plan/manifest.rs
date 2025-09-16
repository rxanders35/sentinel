use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Manifest {
    pub operators: Vec<Operator>, // all user-defined operators
}

#[derive(Deserialize)]
pub struct Operator {
    /// name of the operator
    pub id: String,
    /// name of the rust udf
    pub udf_name: String,
    /// what other operators does this operator depend on?
    #[serde(default)]
    pub depends: Vec<String>,
    /// configures the operator to be a source, sink, or neither
    #[serde(flatten)]
    pub config: Option<Config>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Config {
    /// config for an external source at the "extract" stage
    Source(SourceConfig),
    /// config for an external source at the "load" stage
    Sink(SinkConfig),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SourceConfig {
    Path { path: PathBuf },
    // S3 { bucket: String, key_prefix: String },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SinkConfig {
    Path { path: PathBuf },
    // Postgres { url: String, table: String },
}

#[derive(Debug, thiserror::Error)]
pub enum PlanError {
    #[error("Error finding manifest.toml: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error parsing manifest.toml: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Cycle detected in DAG at operator: {0}")]
    CycleDetected(String),
    // ...
    // add more once I figure out suitable errors
}
