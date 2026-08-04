import { ReactNode, useState } from "react";
import { PrinterStatus } from "../types/printerstatus";
import { VersionInfo } from "../types/versioninfo";
import { usePrinterEvents } from "../hooks/usePrinterEvents";
import { FileItem } from "../types/printerfile";
import { invoke } from "@tauri-apps/api/core";
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


     usePrinterEvents(
         setStatus,
         setFiles,
         setVersion
     );


  async function handleCardClick(cardId: String) {
    const fileItem = await invoke<FileItem>("card_clicked", { cardId });
    setSelectedFile(fileItem);
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


        <div style={{ flex: 1 }}>TDPrinter Desktop</div>

        <div style={{fontSize:"16px"}}>
          Status: <b>{version ? (<p>connected</p>):(<p>none</p>)  }</b>
        </div>
      </header>

      <div className="app-body">
        <aside className="sidebar">
          <h3>Printer Files</h3>
                {files
                  .filter(file => file.type === "PRINT_FILE")
                  .map(file => (                       
                      <div key={file.display_name} className="card" onClick={() => handleCardClick(file.display_name)}>
                        <img src={file.thumbnail_path} alt="Logo" style={{width: "40px", height: "40px"}} />                                                      
                        <p>{file.display_name}</p>
                        <p>{`Last Modified: ${new Date(file.m_timestamp * 1000).toLocaleString()}`}</p>      
                      </div>                                       
                ))}
        </aside>

        <main className="app-content">
          {children}

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


        {selectedFile && (
          <div className="file-details">
            <h2>Selected File Details</h2>            
            <p>{selectedFile.display_name}</p>
            <img src={selectedFile.thumbnail_path} alt="Thumbnail" style={{width: "260px", height: "260px"}} />
            <p>Type: {selectedFile.type}</p>
            <p>Last Modified: {new Date(selectedFile.m_timestamp * 1000).toLocaleString()}</p>
            <p>{selectedFile.name}</p>


            <div className="button-row">
                <button className="delete-btn">Delete File</button>
                {/* <button className="pause-btn" style={{visibility: "hidden"}}>Pause Print</button>                 */}
                <button className="print-btn">Print File</button>
            </div>


          </div>
        )}


        <div className="file-details" style={{visibility: selectedFile ? "hidden" : "visible"}}>
            <h2>File Details</h2>
            <p>Select a file to see details</p>
        </div>









        </main>
      </div>
    </div>
  );
}