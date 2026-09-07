import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { FileItem, FileList } from "../types/printerfile";
import { PrinterStatus } from "../types/printerstatus";
import { VersionInfo } from "../types/versioninfo";
import { PrintJob } from "../types/printjobinfo";
import { TemperatureReading } from "../types/temperature";
import { format } from "../components/stringUtils"
import { convertFileSrc } from "@tauri-apps/api/core";


export function usePrinterEvents(
    setStatus: React.Dispatch<React.SetStateAction<PrinterStatus | null>>,
    setFiles: React.Dispatch<React.SetStateAction<FileItem[]>>,
    setVersion: React.Dispatch<React.SetStateAction<VersionInfo | null>>,
    setShowWarning: React.Dispatch<React.SetStateAction<boolean | null>>,
    setJobInfo: React.Dispatch<React.SetStateAction<PrintJob | null>>,
    setTemperatureReadings: React.Dispatch<React.SetStateAction<TemperatureReading[]>
>    
) {

    useEffect(() => {

        let unlisten: (() => void) | undefined;

        async function setupListener() {

            unlisten = await listen("file-list-updated", async () => {

                const updatedFiles = await invoke<FileList | null>("get_file_list");
                  if (!updatedFiles?.children) {
                      setFiles([]);
                      return;
                  }                 

                  const filesWithImages = updatedFiles.children.map((file) => ({
                      ...file,
                      imageSrc: file.thumbnail_path
                          ? convertFileSrc(file.thumbnail_path)
                          : undefined,
                  }));              
                setFiles(filesWithImages);
            });
        }

        setupListener();

        return () => {
            if (unlisten) {
                unlisten();
            }
        };

    }, []);


    useEffect(() => {

        let unlisten: (() => void) | undefined;

        async function setupListener() {

            unlisten = await listen("job-stopped", async () => {
                console.log("job-stopped event");
                setJobInfo(null);
            });
        }

        setupListener();

        return () => {
            if (unlisten) {
                unlisten();
            }
        };

    }, []);    




  useEffect(() => {
    let unlisten: (() => void) | undefined;


    listen<VersionInfo>("printer-version-updated", (event) => {
      console.log("Received printer version update:", event.payload);
      setVersion(event.payload || null);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);


  useEffect(() => {
    let unlisten: (() => void) | undefined;

    listen<PrinterStatus>("printer-status-updated", (event) => {
        console.log(
          "Received printer status update:",
          event.payload.printer
        );

        // Keep your existing status state
        setStatus(event.payload || null);

        // Extract printer temperature data
        const printer = event.payload.printer;

        var job = event.payload.job;
        // format the time strings for display here
        if(job != undefined){
          if(job.time_remaining != undefined){
            job.time_remaining_str= format(" {0} mins", parseInt((job.time_remaining/60).toFixed(2)))
          }
          if(job.time_printing != undefined){
            var str = format(" {0} min {1} secs",  (job.time_printing/60).toFixed(0), job.time_printing % 60);
            job.time_print_string =   str;
          }
          if(job.progress != undefined){
            job.progress_abs = parseFloat((job.progress/100).toFixed(2))
            console.log("prg {}, prg1 {}",job.progress,job.progress_abs)
          }
        }

        const temperatureReading: TemperatureReading = {
          timestamp: Date.now(),
          // set traget to temp levels if target has not been set (0) for cleaner graph
          bedTarget:printer.target_bed === 0 ? printer.temp_bed ?? 0 : printer.target_bed ?? 0,
          bedTemp: printer.temp_bed ?? 0,
          nozzleTarget: printer.target_nozzle === 0 ? printer.temp_nozzle ?? 0: printer.target_nozzle ?? 0,
          nozzleTemp: printer.temp_nozzle ?? 0,
        };

        // Add the new reading to the history.
        // Keep the most recent 150 readings.
        setTemperatureReadings((previous) => [
          ...previous,
          temperatureReading,
        ].slice(-150));
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);  


  useEffect(() => {
    let unlisten: (() => void) | undefined;

    listen<PrintJob>("new-job-info", (event) => {
      console.log("Received printer job info update:", event.payload);
      setJobInfo(event.payload || null);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);    




  useEffect(() => {
    let unlisten: (() => void) | undefined;

    listen<String>("delete-file-contention", (event) => {
      console.log("delete-file-contention update:", event.payload);
      setShowWarning(true);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);   



useEffect(() => {

    let unlisten: (() => void) | undefined;


        async function setupListener() {

            unlisten = await listen("file-image-updated", async () => {

                const updatedFiles = await invoke<FileList | null>("get_file_list");

                  if (!updatedFiles?.children) {
                      setFiles([]);
                      return;
                  }                 

                  const filesWithImages = updatedFiles.children.map((file) => ({
                      ...file,
                      imageSrc: file.thumbnail_path
                          ? convertFileSrc(file.thumbnail_path)
                          : undefined,
                  }));              

                setFiles(filesWithImages);
            });
        }

        setupListener();    



    return () => {
        unlisten?.();
    };

}, []);


}