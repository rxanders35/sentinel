use crate::dag::manifest::SourceConfig;
use crate::dag::manifest::{Manifest, Operator, OperatorKind};
use crate::engine::udf::{Udf, UdfRegistration};
use crate::source_connectors::path_source::PathSource;
use crate::source_connectors::source_connector::SourceConnector;
use arrow::record_batch::RecordBatch;
use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::task::JoinHandle;

#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error("Execution error in operator '{0}': {1}")]
    ExecutionError(String, String),

    #[error("Source Connector Error: {0}")]
    SourceConnector(#[from] crate::source_connectors::source_connector::SourceConnectorError),
}

pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Engine {}
    }

    pub async fn run(&self, plan: &Manifest) -> Result<(), RuntimeError> {
        let sorted_dag = crate::dag::plan::plan_dag(plan).unwrap();
        let mut udfs: HashMap<&'static str, Box<dyn Udf>> = HashMap::new();
        for registration in inventory::iter::<UdfRegistration>() {
            let udf_instance = (registration.instantiate_udf)();
            let name = udf_instance.name();
            udfs.insert(name, udf_instance);
        }

        let mut senders: HashMap<String, Sender<RecordBatch>> = HashMap::new();
        let mut receivers: HashMap<String, Receiver<RecordBatch>> = HashMap::new();
        for operator in &sorted_dag {
            let (tx, rx) = channel(100);
            senders.insert(operator.id.clone(), tx);
            receivers.insert(operator.id.clone(), rx);
        }
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for operator in sorted_dag {
            let udf = udfs.remove(operator.udf_name.as_str()).unwrap();
            let mut input_rxs = Vec::new();
            for dependency_id in &operator.depends {
                let rx = receivers.remove(dependency_id).unwrap();
                input_rxs.push(rx);
            }
            let output_tx = senders.remove(&operator.id).unwrap();
            let operator_id = operator.id.clone();
            let operator_kind = operator.kind.clone();

            let handle = tokio::spawn(async move {
                match operator_kind {
                    OperatorKind::Source { config } => {
                        let mut source_connector: Box<dyn SourceConnector> = match config {
                            SourceConfig::Path { path } => {
                                Box::new(PathSource::new().with_path(path).unwrap())
                            }
                        };
                        while let Some(result) = source_connector.produce().await {
                            let batch = result.unwrap();
                            let transformed_batch = udf.execute(batch).await.unwrap();
                            if output_tx.send(transformed_batch).await.is_err() {
                                eprintln!("Channel closed for source operator {}", operator_id);
                                break;
                            }
                        }
                    }
                    OperatorKind::Transform => {
                        let mut input_rx = input_rxs.pop().unwrap();

                        while let Some(batch) = input_rx.recv().await {
                            let result_batch = udf.execute(batch).await.unwrap();
                            if output_tx.send(result_batch).await.is_err() {
                                break;
                            }
                        }
                    }
                    OperatorKind::Sink { config } => {
                        let mut input_rx = input_rxs.pop().unwrap();
                        while let Some(batch) = input_rx.recv().await {
                            udf.execute(batch).await.unwrap();
                        }
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        Ok(())
    }
}
