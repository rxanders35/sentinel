
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
