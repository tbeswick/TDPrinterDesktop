use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThumbnailState {
    NotStarted,
    Downloading,
    Ready,
    Failed,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmState{
    Connect,
    Info,
    Files,
    Idle,
    Status,
    UploadFile,
     DeleteFile,
     SendPrintJob,
     NewJob,     
    // StopPrintJob,
    // PausePrintJob,
    // ResumePrintJob
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmRequest{
    Continue,
    Idle,
    Restart,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterStatus {
    pub printer: Option<PrinterTelemetry>,
    pub job: Option<JobStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterTelemetry {
    pub state: Option<String>,
    pub temp_bed: Option<f64>,
    pub target_bed: Option<f64>,
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
pub struct JobStatus {
    pub id: Option<u64>,
    pub progress: Option<f64>,
    pub time_remaining: Option<u64>,
    pub time_printing: Option<u64>,
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
    pub min_extrusion_temp: Option<i64>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileList {
    #[serde(rename = "children")]
    pub children: Option<Vec<FileItem>>,    
}



#[derive(Debug, Clone, Serialize, Deserialize)]



pub struct FileItem {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "display_name")]
    pub display_name: String,

    #[serde(rename = "type")]
    pub file_type: Option<String>,

    #[serde(rename = "m_timestamp")]
    pub last_modified_timestamp: i64,

    #[serde(rename = "refs")]
    pub refs: Option<FileRefs>,

    //#[serde(rename = "thumbnail_image")]
    pub thumbnail_image: Option<Vec<u8>>,

    #[serde(rename = "thumbnail_downloaded")]
    pub thumbnail_state: Option<ThumbnailState>,


    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRefs {
    pub icon: Option<String>,    
    pub thumbnail: Option<String>,
    pub download: Option<String>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct PrintFile {
    pub refs: FileRefs,
    pub name: Option<String>,
    pub display_name: String,
    pub path: Option<String>,
    pub size: Option<i64>,
    pub m_timestamp: i64,
}





#[derive(Debug, Serialize, Deserialize)]
pub struct PrintJob{        
    pub id: i32,        
    pub state: Option<String>,    
    pub progress: f64,
    pub time_remaining: i32,        
    pub time_printing: i32,
    pub file: PrintFile,
}