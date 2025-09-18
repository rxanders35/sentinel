use crate::source_connectors::source_connector::{SourceConnector, SourceConnectorError};
use arrow::record_batch::RecordBatch;
use arrow_csv::reader::{ReaderBuilder, infer_schema_from_files};
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

pub struct PathSource {
    reader: Option<arrow_csv::Reader<File>>,
}

impl PathSource {
    pub fn new() -> Self {
        Self { reader: None }
    }

    pub fn with_path(mut self, path: PathBuf) -> Result<Self, SourceConnectorError> {
        let file_path = path.display().to_string();
        let schema = infer_schema_from_files(&[file_path], b',', Some(100), true)
            .map_err(SourceConnectorError::from)?;
        let file = File::open(&path).map_err(SourceConnectorError::from)?;
        let reader = ReaderBuilder::new(Arc::new(schema))
            .with_header(true)
            .with_batch_size(1024)
            .build(file)
            .map_err(SourceConnectorError::from)?;
        self.reader = Some(reader);
        Ok(self)
    }
}

#[async_trait::async_trait]
impl SourceConnector for PathSource {
    async fn produce(&mut self) -> Option<Result<RecordBatch, SourceConnectorError>> {
        let reader = self.reader.as_mut()?;
        reader
            .next()
            .map(|result| result.map_err(SourceConnectorError::from))
    }
}
