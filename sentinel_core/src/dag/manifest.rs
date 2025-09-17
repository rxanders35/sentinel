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

    // This field now clearly holds the operator's type and its specific config.
    // The `flatten` attribute here is what allows the `type` and `config` keys
    // to exist directly within the `[[operators]]` table in the TOML.
    #[serde(flatten)]
    pub kind: OperatorKind,
}

#[derive(Deserialize, Debug)]
// This is the key: `serde` will look for a TOML key named "type" and use its
// value ("source", "sink", or "transform") to decide which variant to pick.
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OperatorKind {
    // If type = "source", it expects a table named `config` containing a SourceConfig.
    Source { config: SourceConfig },
    // If type = "sink", it expects a table named `config` containing a SinkConfig.
    Sink { config: SinkConfig },
    // If type = "transform", it expects no other fields.
    Transform,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SourceConfig {
    Path { path: PathBuf },
}

#[derive(Debug, Deserialize)]
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
