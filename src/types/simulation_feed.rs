use crate::types::simulation_payload::SimulationPayload;
use crossbeam::atomic::AtomicCell;
use std::sync::Arc;

pub type SimulationFeed = Arc<AtomicCell<Arc<SimulationPayload>>>;
