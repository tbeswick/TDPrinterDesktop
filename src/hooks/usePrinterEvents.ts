import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { FileItem, FileList } from "../types/printerfile";
import { PrinterStatus } from "../types/printerstatus";
import { VersionInfo } from "../types/versioninfo";


export function usePrinterEvents(
    setStatus: React.Dispatch<React.SetStateAction<PrinterStatus | null>>,
    setFiles: React.Dispatch<React.SetStateAction<FileItem[]>>,
    setVersion: React.Dispatch<React.SetStateAction<VersionInfo | null>>
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
      console.log("Received printer status update:", event.payload.printer);
      setStatus(event.payload || null);
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