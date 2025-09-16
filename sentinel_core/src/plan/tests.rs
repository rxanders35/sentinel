#[cfg(test)]
use crate::plan::manifest::Manifest;
use crate::plan::manifest::Operator;
use crate::plan::manifest::PlanError;
use crate::plan::planner::plan_dag;

#[test]
fn test_parse_manifest() {
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

fn op(id: &str, depends: &[&str]) -> Operator {
    Operator {
        id: id.to_string(),
        udf_name: format!("{}_udf", id),
        depends: depends.iter().map(|s| s.to_string()).collect(),
        config: None,
    }
}

#[test]
fn test_plan_valid_dag_returns_sorted_operators() {
    let manifest = Manifest {
        operators: vec![
            op("D", &["B", "C"]),
            op("C", &["A"]),
            op("B", &["A"]),
            op("A", &[]),
        ],
    };

    let sorted_ops = plan_dag(&manifest).unwrap();
    let sorted_ids: Vec<&str> = sorted_ops.iter().map(|op| op.id.as_str()).collect();

    assert_eq!(sorted_ids.len(), 4);

    let pos_a = sorted_ids.iter().position(|&id| id == "A").unwrap();
    let pos_b = sorted_ids.iter().position(|&id| id == "B").unwrap();
    let pos_c = sorted_ids.iter().position(|&id| id == "C").unwrap();
    assert!(pos_a < pos_b);
    assert!(pos_a < pos_c);

    let pos_d = sorted_ids.iter().position(|&id| id == "D").unwrap();
    assert!(pos_b < pos_d);
    assert!(pos_c < pos_d);
}

#[test]
fn test_plan_with_cycle_returns_cycle_detected_error() {
    let manifest = Manifest {
        operators: vec![op("A", &["C"]), op("B", &["A"]), op("C", &["B"])],
    };

    let result = plan_dag(&manifest);

    match result {
        Err(PlanError::CycleDetected(node_id)) => {
            assert!(["A", "B", "C"].contains(&node_id.as_str()));
        }
        Ok(_) => panic!("Expected a CycleDetected error, but got Ok"),
        Err(_) => panic!("Expected a CycleDetected error, but got a different error"),
    }
}
