

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