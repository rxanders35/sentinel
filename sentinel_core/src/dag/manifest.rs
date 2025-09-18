use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub operators: Vec<Operator>,
}

#[derive(Deserialize, Debug)]
pub struct Operator {
    /// name of the operator
    pub id: String,
    /// name of the rust udf
    pub udf_name: String,
    /// what other operators does this operator depend on?
    #[serde(default)]
    pub depends: Vec<String>,
    #[serde(flatten)]
    pub kind: OperatorKind,
}
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OperatorKind {
    Source { config: SourceConfig },
    Sink { config: SinkConfig },
    Transform,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SourceConfig {
    Path { path: PathBuf },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SinkConfig {
    Path { path: PathBuf },
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
