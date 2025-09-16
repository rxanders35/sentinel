use crate::plan::manifest::{Manifest, Operator, PlanError};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

pub fn plan_dag(manifest: &Manifest) -> Result<Vec<&Operator>, PlanError> {
    let mut graph = petgraph::graph::DiGraph::new();
    let mut op_ids: HashMap<&str, NodeIndex> = HashMap::new();

    for op in &manifest.operators {
        let node_idx = graph.add_node(op);
        op_ids.insert(op.id.as_str(), node_idx);
    }

    for op in &manifest.operators {
        let dest_idx = op_ids[op.id.as_str()];
        for dep in &op.depends {
            let src_idx = op_ids[dep.as_str()];
            graph.add_edge(src_idx, dest_idx, ());
        }
    }

    match petgraph::algo::toposort(&graph, None) {
        Ok(sorted) => {
            let ordered_ops: Vec<&Operator> = sorted
                .into_iter()
                .map(|index| *graph.node_weight(index).unwrap())
                .collect();
            Ok(ordered_ops)
        }
        Err(cycle) => {
            let cycle_node = cycle.node_id();
            let op = graph.node_weight(cycle_node).unwrap();
            Err(PlanError::CycleDetected(op.id.clone()))
        }
    }
}
