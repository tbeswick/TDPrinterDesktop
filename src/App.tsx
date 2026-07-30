import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import { FileItem } from "./types/printerfile";
import { PrinterStatus } from "./types/printerstatus";
import { VersionInfo } from "./types/versioninfo";
import { usePrinterEvents } from "./hooks/usePrinterEvents";


import DashboardLayout from "./layout/DashboardLayout";
import Overview from "./pages/Overview";

export default function App() {
  return (
    <DashboardLayout>
      <Overview />
    </DashboardLayout>
  );
}




// function App() {


//  const [status, setStatus] = useState<PrinterStatus | null>(null);
//  const [files, setFiles] = useState<FileItem[]>([]); 
//  const [version, setVersion] = useState<VersionInfo | null>(null);


//     usePrinterEvents(
//         setStatus,
//         setFiles,
//         setVersion  
//     );


//   return (

//     <main className="container">
//       <h1>Welcome to Tauri + React</h1>

//       {status ? (
//         <div>
//           <p>Connected: {status.printer.connected ? "Yes" : "No"}</p>
//           <p>State: {status.printer.state}</p>
//           <p>Nozzle: {status.printer.temp_nozzle} °C</p>
//           <p>Bed: {status.printer.bed_temp} °C</p>
//           <p>Progress: {status.printer.progress}%</p>
//         </div>
//       ) : (
//         <p>Waiting for printer...</p>
//       )}

//       {version ? (
//         <div>
//           <p>API Version: {version.api}</p>
//           <p>Server Version: {version.server}</p>
//           <p>Nozzle Diameter: {version.nozzle_diameter}</p>
//           <p>Hostname: {version.hostname}</p>
//           <p>Firmware: {version.firmware}</p>
//           <p>Printer: {version.printer}</p>
//         </div>
//       ) : (
//         <p>Waiting for printer version...</p>
//       )}

//         <div>
//             {files.map(file => (
//                 <div key={file.display_name}>
//                     {file.display_name}
//                 </div>
//             ))}
//         </div>



//       <div className="row">
//         <a href="https://vite.dev" target="_blank">
//           <img src="/vite.svg" className="logo vite" alt="Vite logo" />
//         </a>
//         <a href="https://tauri.app" target="_blank">
//           <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
//         </a>
//         <a href="https://react.dev" target="_blank">
//           <img src={reactLogo} className="logo react" alt="React logo" />
//         </a>
//       </div>
//       <p>Click on the Tauri, Vite, and React logos to learn more.</p>


//     </main>
//   );
// }

// export default App;
