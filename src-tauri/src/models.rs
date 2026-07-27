use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterStatus {
    pub printer: Option<PrinterTelemetry>,
    pub job: Option<JobInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterTelemetry {
    pub state: Option<String>,
    pub temp_bed: Option<f64>,
    pub temp_nozzle: Option<f64>,
    pub target_nozzle: Option<f64>,
    pub axis_z: Option<f64>,
    pub axis_x: Option<f64>,
    pub axis_y: Option<f64>,
    pub flow: Option<f64>,
    pub speed: Option<f64>,
    pub fan_hotend: Option<f64>,
    pub fan_print: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobInfo {
    pub progress: Option<f64>,
    pub time_remaining: Option<u64>,
    pub file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapInfo{
    pub upload_by_pc: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    pub api: Option<String>,
    pub server: Option<String>,
    pub nozzle_diameter: Option<f64>,
    pub text: Option<String>,
    pub hostname: Option<String>,
    pub firmware: Option<String>,
    pub printer: Option<String>,
    pub capabilities: Option<CapInfo>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterInfo {
    pub nozzle_diameter: Option<f64>,
    pub mmu: Option<bool>,
    pub serial: Option<String>,
    pub hostname: Option<String>,
    pub min_extrution_temp: Option<i64>,
}