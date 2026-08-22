import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { FileItem, FileList } from "../types/printerfile";
import { PrinterStatus } from "../types/printerstatus";
import { VersionInfo } from "../types/versioninfo";
import { PrintJob } from "../types/printjobinfo";
import { TemperatureReading } from "../types/temperature";


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
                 console.log("Updated file list:", updatedFiles!.children);
                setFiles(updatedFiles!.children ?? []);
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

        const temperatureReading: TemperatureReading = {
          timestamp: Date.now(),

          bedTarget: printer.target_bed ?? 0,
          bedTemp: printer.temp_bed ?? 0,

          nozzleTarget: printer.target_nozzle ?? 0,
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
                 console.log("Updated file list:", updatedFiles!.children);
                setFiles(updatedFiles!.children ?? []);
            });
        }

        setupListener();    


    // async function setup() {

    //     unlisten = await listen<ThumbnailEvent>(
    //         "file-image-updated",

    //         (event) => {

    //             const thumbnail = event.payload;

    //             setFiles(oldFiles =>

    //                 oldFiles.map(file => {

    //                    console.log("Updating file:", file.thumbnail_path, "with thumbnail path:", thumbnail.path);
    //                     if (file.thumbnail_path !== thumbnail.path)
    //                         return file;

    //                     return {
    //                         ...file,
    //                         image: thumbnail.image
    //                     };
    //                 })
    //             );
    //         }
    //     );
    // }

    //setup();

    return () => {
        unlisten?.();
    };

}, []);


}