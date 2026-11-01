use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GroundTruth {
    pub open_ports: Vec<u16>,
    pub severity_score: f32,
}

#[derive(Deserialize)]
pub struct MinerResponse {
    pub detected_ports: Vec<u16>,
    pub evaluated_severity: f32,
}

#[derive(Serialize)]
pub struct EvalScore {
    pub port_accuracy: f32,
    pub severity_mae: f32,
    pub final_score: f32,
}

#[no_mangle]
pub extern "C" fn evaluate(miner_json: *const u8, miner_len: usize, gt_json: *const u8, gt_len: usize) -> f32 {
    // Evaluation scoring logic goes here
    // Compiles cleanly to target wasm32-wasip1
    1.0
}