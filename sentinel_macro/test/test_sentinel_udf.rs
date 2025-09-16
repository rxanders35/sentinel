use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use sentinel_core::engine::udf::{Udf, UdfRegistration};
use sentinel_macro::sentinel_udf;
use std::sync::Arc;

#[sentinel_udf]
async fn my_test_udf(batch: RecordBatch) -> RecordBatch {
    println!("My test UDF was executed!");
    batch // Just return the batch as-is.
}

#[tokio::test]
async fn test_sentinel_udf_macro() {
    let registration = inventory::iter::<UdfRegistration>()
        .find(|reg| {
            let udf = (reg.instantiate_udf)();
            udf.name() == "my_test_udf"
        })
        .expect("Failed to find 'my_test_udf' in inventory.");

    let udf = (registration.instantiate_udf)();

    assert_eq!(udf.name(), "my_test_udf");

    let schema = Arc::new(Schema::new(vec![Field::new("a", DataType::Int32, false)]));
    let batch = RecordBatch::new_empty(schema);

    let result = udf.execute(batch).await;
    assert!(result.is_ok(), "The UDF execution should succeed.");

    let processed_batch = result.unwrap();
    assert_eq!(
        processed_batch.num_columns(),
        1,
        "The processed batch should have the correct schema."
    );
}
