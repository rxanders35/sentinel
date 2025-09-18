use arrow::record_batch::RecordBatch;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum SourceConnectorError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Arrow Error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),
}

#[async_trait]
pub trait SourceConnector: Send + Sync {
    /// Produces the first RecordBatch for the pipeline via a SourceConnector
    async fn produce(&mut self) -> Option<Result<RecordBatch, SourceConnectorError>>;
}
