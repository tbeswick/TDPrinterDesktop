import { ReactNode, useState } from "react";
import { PrinterStatus } from "../types/printerstatus";
import { VersionInfo } from "../types/versioninfo";
import { usePrinterEvents } from "../hooks/usePrinterEvents";
import {PrintJob} from "../types/printjobinfo"
import { FileItem } from "../types/printerfile";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./layout.css";



export default function DashboardLayout({
  children,
}: {
  children: ReactNode;
}) {

  const [status, setStatus] = useState<PrinterStatus | null>(null);
  const [files, setFiles] = useState<FileItem[]>([]); 
  const [version, setVersion] = useState<VersionInfo | null>(null);
  const [selectedFile, setSelectedFile] = useState<FileItem | null>(null);
  const [showWarning, setShowWarning] = useState<boolean | null>(null);
  const [jobInfo, setJobInfo] = useState<PrintJob | null>(null);



     usePrinterEvents(
         setStatus,
         setFiles,
         setVersion,
         setShowWarning,
         setJobInfo
     );


  async function handleCardClick(cardId: String) {
    const fileItem = await invoke<FileItem>("card_clicked", { cardId });
    setSelectedFile(fileItem);
  }     

  async function handleAddFileClick() {

    console.log("Add File button clicked");

    const selected = await open({
        multiple: false,
        filters: [
            {
                name: "G-code files",
                extensions: ["bgcode"],
            },
        ],
    });

    if (!selected) {
        return; // User cancelled
    }

    console.log("Selected:", selected);

    const result = await invoke<string>("send_gcode", {
        path: selected,
    });

    console.log(result);
}

  async function handleDeleteButtonClick() {
    await invoke<string>("deletebutton_clicked", {
      name: selectedFile?.name || "Unknown",
    });

    selectedFile && setSelectedFile(null); // Clear the selected file after deletion

    console.log("Delete button clicked for file: ", selectedFile?.name);
  }



  async function handlePrintButtonClick() {
    await invoke<string>("printbutton_clicked", {
      name: selectedFile?.name || "Unknown",
    });
  }


  return (
    <div className="app-shell">
      <header className="topbar">

        {version ? (
         <div style={{fontSize:"12px"}}>
           <p>API Version: {version.api}</p>
           <p>Server Version: {version.server}</p>
           <p>Nozzle Diameter: {version.nozzle_diameter}</p>
           <p>Hostname: {version.hostname}</p>
           <p>Firmware: {version.firmware}</p>
           <p>Printer: {version.printer}</p>
         </div>
        ): (
          <p style={{fontSize:"12px"}}>Waiting for version...</p>
        )}


        <div
          style={{
            flex: 1,
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          <img
            src="logo.png"
            alt="Logo"
            style={{ width: "140px", height: "140px" }}
          />

          <p
            style={{
              margin: "0 0 0 30px",
              fontSize: "1.8rem",
              color: "#f5f1f1",
            }}
          >
            3D Printer Manager
          </p>
        </div>

        <div style={{fontSize:"16px"}}>
          Status: <b>{version ? (<p>connected</p>):(<p>none</p>)  }</b>
        </div>
      </header>

      <div className="app-body">
        <aside className="sidebar">
          <button className="add-file-btn" onClick={handleAddFileClick} style={{visibility: version ? "visible" : "hidden"}}>
              Add local file to Printer
          </button>
          <h3>Printer Files</h3>
                {files
                  .filter(file => file.type === "PRINT_FILE")
                  .map(file => (                       
                      <div key={file.display_name} className="card" onClick={() => handleCardClick(file.display_name)}>
                        <img src={file.thumbnail_path} alt="Logo" style={{width: "40px", height: "40px"}} />                                                      
                        <p>{file.display_name}</p>
                        {/* <p>{`Last Modified: ${new Date(file.m_timestamp * 1000).toLocaleString()}`}</p>       */}
                      </div>                                       
                ))}
        </aside>

        <main className="app-content">
          {children}


        {showWarning && (
            <div className="warning-overlay">
                <div className="warning-dialog">
                    <h2>File In Use</h2>

                    <p>
                        This file cannot be deleted because it is currently
                        being used by another process. Check printer status and try again later.
                    </p>

                    <button onClick={() => setShowWarning(false)}>
                        OK
                    </button>
                </div>
            </div>
        )}



       {/* {status ? (
         <div>
           <p>State: {status.printer.state}</p>
           <p>Nozzle: {status.printer.temp_nozzle} °C</p>
           <p>Bed: {status.printer.bed_temp} °C</p>
           <p>Progress: {status.printer.progress}%</p>
         </div>
       ) : (
         <p>Waiting for printer...</p>
       )} */}


        {selectedFile && jobInfo === null  && (
          <div className="file-details">
            <h2>Selected File Details</h2>            
            <img src={selectedFile.thumbnail_path} alt="Thumbnail" style={{width: "260px", height: "260px"}} />
            <p>{selectedFile.display_name}</p>            
            <p>Type: {selectedFile.type}</p>
            <p>Last Modified: {new Date(selectedFile.m_timestamp * 1000).toLocaleString()}</p>
            <p>System name: {selectedFile.name}</p>


            <div className="button-row">
                <button className="delete-btn" onClick={handleDeleteButtonClick}>
                    Delete File
                </button>
                {/* <button className="pause-btn" style={{visibility: "hidden"}}>Pause Print</button>                 */}
                <button className="print-btn" onClick={handlePrintButtonClick}>
                    Print File
                </button>
            </div>


          </div>
        )}


        <div className="file-details" style={{visibility: selectedFile  ? "hidden" : "visible"}}>
            <h2>File Details</h2>
            <p>Select a file to see details</p>
        </div>



        <div className="file-details" style={{ visibility: jobInfo ? "visible" : "hidden", gridColumn: "2", gridRow: "1" }}>
            <h2>Print Job Details</h2>
            <p>{jobInfo?.file?.display_name}</p>
            <img src={selectedFile?.thumbnail_path} alt="Thumbnail" style={{width: "260px", height: "260px"}} />            
        </div>        

        </main>
      </div>
    </div>
  );
}