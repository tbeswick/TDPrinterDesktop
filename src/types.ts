

export interface FileItem {
    name: string;
    display_name: string;
    file_type: string;
    last_modified_timestamp: number;
}


export interface FileList {
    children?: FileItem[];
}


export interface PrinterStatus  {
  printer: {
    connected?: boolean;
    state?: string;
    temperature?: number;
    temp_nozzle?: number;
    bed_temp?: number;
    progress?: number;
    time_remaining?: number;
  }
};