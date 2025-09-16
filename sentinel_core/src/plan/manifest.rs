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

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_parse_valid_manifest_successfully() {
        let sample_manifest = r#"
            [[operators]]
            id = "read_csv"
            udf_name = "csv_source_udf"
            # 'depends' is omitted to test the #[serde(default)]
            [operators.source]
            type = "path"
            path = "/data/raw/input.csv"

            [[operators]]
            id = "transform_data"
            udf_name = "clean_and_enrich_udf"
            depends = ["read_csv"]
            # This operator has no source or sink config

            [[operators]]
            id = "write_results"
            udf_name = "parquet_sink_udf"
            depends = ["transform_data"]
            [operators.sink]
            type = "path"
            path = "/data/processed/output.parquet"
        "#;

        let manifest: Manifest = toml::from_str(sample_manifest).unwrap();

        assert_eq!(manifest.operators.len(), 3);

        let source_op = &manifest.operators[0];
        assert_eq!(source_op.id, "read_csv");
        assert_eq!(source_op.udf_name, "csv_source_udf");
        assert!(
            source_op.depends.is_empty(),
            "Source operator should have no dependencies"
        );
        assert!(
            source_op.config.is_some(),
            "Source operator should have a config"
        );

        let transform_op = &manifest.operators[1];
        assert_eq!(transform_op.id, "transform_data");
        assert_eq!(transform_op.depends, vec!["read_csv"]);
        assert!(
            transform_op.config.is_none(),
            "Transform operator should not have a config"
        );

        let sink_op = &manifest.operators[2];
        assert_eq!(sink_op.id, "write_results");
        assert_eq!(sink_op.depends, vec!["transform_data"]);
        assert!(
            sink_op.config.is_some(),
            "Sink operator should have a config"
        );
    }
}
