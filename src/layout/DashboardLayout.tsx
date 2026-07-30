import Sidebar from "./Sidebar";
import Topbar from "./Topbar";
import { ReactNode, useState } from "react";
import { PrinterStatus } from "../types/printerstatus";
import { VersionInfo } from "../types/versioninfo";
import { usePrinterEvents } from "../hooks/usePrinterEvents";
import { FileItem } from "../types/printerfile";
import "./layout.css";

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
        <Sidebar />

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