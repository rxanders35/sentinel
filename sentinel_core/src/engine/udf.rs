#[async_trait::async_trait]
pub trait Udf {
    /// Returns the name of the user-defined function to be stored in the inventory.
    fn name(&self) -> &'static str;

    /// Wraps the execution of a user-defined function.
    /// All user-defined functions must take in a RecordBatch and return a RecordBatch accross green threads.
    async fn execute(
        &self,
        record_batch: arrow::record_batch::RecordBatch,
    ) -> Result<arrow::record_batch::RecordBatch, Box<dyn std::error::Error + Send + Sync>>;
}
