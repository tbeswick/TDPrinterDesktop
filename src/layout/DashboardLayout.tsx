import Topbar from "./Topbar";
import { ReactNode, useState } from "react";
import { PrinterStatus } from "../types/printerstatus";
import { VersionInfo } from "../types/versioninfo";
import { usePrinterEvents } from "../hooks/usePrinterEvents";
import { FileItem } from "../types/printerfile";
import "./layout.css";
import logo from "../assets/react.svg";


//({file.name}) - Type: {file.type} - Last Modified: {new Date(file.m_timestamp * 1000).toLocaleString()}


export default function DashboardLayout({
  children,
}: {
  children: ReactNode;
}) {

  const [status, setStatus] = useState<PrinterStatus | null>(null);
  const [files, setFiles] = useState<FileItem[]>([]); 
  const [version, setVersion] = useState<VersionInfo | null>(null);


     usePrinterEvents(
         setStatus,
         setFiles,
         setVersion  
     );


  return (
    <div className="app-shell">
      <Topbar />

      <div className="app-body">
        <aside className="sidebar">
          <h3>Printer Files</h3>
                {files
                  .filter(file => file.type === "PRINT_FILE")
                  .map(file => (                       
                      <div key={file.display_name} className="card">
                        <img src={logo} alt="Logo" style={{width: "20%", margin: "auto"}} />                          
                        <div className="container">       
                          <h4><b>{file.display_name}</b></h4>
                          <p>{`Last Modified: ${new Date(file.m_timestamp * 1000).toLocaleString()}`}</p>
                        </div>
                      </div>                                       
                ))}
        </aside>

        <main className="app-content">
          {children}

       {status ? (
         <div>
           <p>Connected: {status.printer.connected ? "Yes" : "No"}</p>
           <p>State: {status.printer.state}</p>
           <p>Nozzle: {status.printer.temp_nozzle} °C</p>
           <p>Bed: {status.printer.bed_temp} °C</p>
           <p>Progress: {status.printer.progress}%</p>
         </div>
       ) : (
         <p>Waiting for printer...</p>
       )}

       {version ? (
         <div>
           <p>API Version: {version.api}</p>
           <p>Server Version: {version.server}</p>
           <p>Nozzle Diameter: {version.nozzle_diameter}</p>
           <p>Hostname: {version.hostname}</p>
           <p>Firmware: {version.firmware}</p>
           <p>Printer: {version.printer}</p>
         </div>
       ) : (
         <p>Waiting for printer version...</p>
       )}









        </main>
      </div>
    </div>
  );
}