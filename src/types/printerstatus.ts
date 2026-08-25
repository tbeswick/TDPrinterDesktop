

export interface PrinterStatus  {
  printer: {
    connected?: boolean;
    state?: string;

    temp_nozzle?: number;
    target_nozzle?: number;

    temp_bed?: number;
    target_bed?: number;

    axis_z?: number;
    axis_x?: number;
    axis_y?: number;

    flow?: number;
    speed?: number;

    fan_hotend?: number;
    fan_print?: number;
  };
  job: {
    id?:number;
    progress?: number;
    progress_abs?:number;
    time_remaining?: number;
    time_remaining_str?:string,
    time_printing?:number
    time_print_string?: string
  };
};