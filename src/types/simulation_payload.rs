use crate::types::cell_configuration::CellConfiguration;

#[derive(Default)]
pub struct SimulationPayload {
    pub cconf: Option<CellConfiguration>,
}

impl SimulationPayload {
    pub fn new(cconf: Option<CellConfiguration>) -> Self {
        Self { cconf }
    }
}
