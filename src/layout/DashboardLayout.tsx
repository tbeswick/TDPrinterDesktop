import { ReactNode, useState } from "react";
import { useEffect } from "react";
import { PrinterStatus } from "../types/printerstatus";
import { VersionInfo } from "../types/versioninfo";
import { usePrinterEvents } from "../hooks/usePrinterEvents";
import { PrintJob } from "../types/printjobinfo"
import { FileItem } from "../types/printerfile";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { TemperatureReading } from "../types/temperature";
import TempertureChart from "../components/TemperatureChart"
import SettingsDialog from "../components/SettingsDialog";
import "./layout.css";
import { TauriEvent } from "@tauri-apps/api/event";



export default function DashboardLayout({
}: {
  children: ReactNode;
}) {

  const [status, setStatus] = useState<PrinterStatus | null>(null);
  const [files, setFiles] = useState<FileItem[]>([]);
  const [version, setVersion] = useState<VersionInfo | null>(null);
  const [selectedFile, setSelectedFile] = useState<FileItem | null>(null);
  const [showWarning, setShowWarning] = useState<boolean | null>(null);
  const [jobInfo, setJobInfo] = useState<PrintJob | null>(null);
  const [showAbout, setShowAbout] = useState<boolean | null>(null);
  const [temperatureReadings, setTemperatureReadings] = useState<TemperatureReading[]>([]);  
  const [showSettings, setShowSettings] = useState(false);  

  const [printStop, setPrintStop] = useState<boolean>(true);
  const [printPause, setPrintPause] = useState<boolean>(true);
  const [printResume, setPrintResume] = useState<boolean>(true);



  usePrinterEvents(
    setStatus,
    setFiles,
    setVersion,
    setShowWarning,
    setJobInfo,
    setTemperatureReadings
  );


    useEffect(() => {

      async function notifyBackendReady() {
        try {

          await invoke("ui_ready");

          console.log("Backend notified: UI is ready");

        } catch (error) {

          console.error(
            "Failed to notify backend that UI is ready:",
            error
          );

        }
      }

      notifyBackendReady();

    }, []);  




  async function handleCardClick(cardId: String) {
    const fileItem = await invoke<FileItem>("card_clicked", { cardId });
    setSelectedFile(fileItem);
  }


  async function handlePrintButtonClick() {
    await invoke<string>("printbutton_clicked", {
      name: selectedFile?.name || "Unknown",
    });
    // clear the selected file (clears display) after print request
    selectedFile && setSelectedFile(null);

    setPrintStop(true); // Show the Stop button when a print job starts
    setPrintPause(true); // Show the Pause button when a print job starts
    setPrintResume(false); // Hide the Resume button when a print job starts

  }  



  async function handleStopPrintClick() {

    await invoke<boolean>("stop_print_clicked", {
      jobId: jobInfo?.id || 0
    })
    setPrintStop(false); // Hide the Stop button after stopping the print
    setPrintPause(false); // Hide the Pause button after stopping the print
    setPrintResume(false); // Hide the Resume button after stopping the print

  }


  async function handlePausePrintClick() {

    await invoke<boolean>("pause_print_clicked", {
      jobId: jobInfo?.id || 0
    })    


    setPrintPause(false); // Hide the Pause button after pausing the print
    setPrintResume(true); // Show the Resume button after pausing the print
    setPrintStop(true); // Show the Stop button after pausing the print

  }
  
  async function handleResumePrintClick() {

    await invoke<boolean>("resume_print_clicked", {
      jobId: jobInfo?.id || 0
    });

    setPrintResume(false); // Hide the Resume button when a print job is resumed
    setPrintPause(true); // Show the Pause button when a print job is resumed
    setPrintStop(true); // Show the Stop button when a print job is resumed

  }  



  async function handleAboutClick() {
    setShowAbout(true);
  }



  async function handleAddFileClick() {

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






  return (
    <div className="app-shell">
      <header className="topbar" style={{ display: "flex" }}>

        <div style={{ flex: "1", textAlign: "left" }}>
          <img
            src="logo.png"
            alt="Logo"
            style={{ width: "140px", height: "140px" }}
          />
        </div>


        <div style={{ flex: "1" }} >
          <p
            style={{
              fontSize: "1.8rem",
              color: "#f5f1f1",
            }}
          >
            3D Printer Manager
          </p>
        </div>

      <div style={{ flex: "1", textAlign: "center" }}>
        <a
          href="#"
          onClick={(e) => {
            e.preventDefault();
            setShowSettings(true);
          }}
          style={{
            paddingRight: "30px",
            textDecoration: "none",
            fontSize: "18px",
            color: "#ccd5ee"
          }}
        >
          Settings
        </a>
        </div>  


        <div style={{ flex: "1", textAlign: "right" }}>
          <a href="#" onClick={handleAboutClick} style={{ paddingRight: "30px", textDecoration: "none", fontSize: "18px", color: "#ccd5ee" }} >printer version</a>
        </div>


      </header>

      <div className="app-body">
        <aside className="sidebar">
          <button className="add-file-btn" onClick={handleAddFileClick} style={{ visibility: status ? "visible" : "hidden" }}>
            Add local file to Printer
          </button>
          <h3>Printer Files</h3>
          {files
            .filter(file => file.type === "PRINT_FILE")
            .map(file => (
              <div key={file.display_name} className="card" onClick={() => handleCardClick(file.display_name)}>
                <img src={file.thumbnail_path} alt="Logo" style={{ width: "50px", height: "50px" }} />
                <p>{file.display_name}</p>
                {/* <p>{`Last Modified: ${new Date(file.m_timestamp * 1000).toLocaleString()}`}</p>       */}
              </div>
            ))}
        </aside>



        <div className="app-content">

          {status ?
            (<div className={"printer-status " + ((status?.printer.state === "IDLE") ? " idle-state" : " busy-state")}>
              {status.printer.state}
            </div> ) :
            (<div className={"printer-status connect-state"}>
              waiting for connection...
            </div>)
          }

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


          {
            showAbout && (
              <div className="warning-overlay">
                <div className="warning-dialog">
                  <h2>Version Information</h2>
                  {version ? (
                    <div>
                      <p>API Version: {version?.api}</p>
                      <p>Server Version: {version?.server}</p>
                      <p>Nozzle Diameter: {version?.nozzle_diameter}</p>
                      <p>Hostname: {version?.hostname}</p>
                      <p>Firmware: {version?.firmware}</p>
                      <p>Printer: {version?.printer}</p>
                    </div>
                  ) : (
                    <p>Printer not connected</p>
                  )
                  }
                  <button onClick={() => setShowAbout(false)}>
                    OK
                  </button>
                </div>
              </div>

            )
          }



          {showSettings && (
            <SettingsDialog
              onClose={() => setShowSettings(false)}
            />
          )}



          {selectedFile && (
            <div className="warning-overlay">
              <div className="warning-dialog">
                <h2>Selected File Details</h2>

                <button
                  className="close-btn"
                  onClick={() => setSelectedFile(null)}
                  aria-label="Close"
                >
                  ×
                </button>

                <img src={selectedFile.thumbnail_path} alt="Thumbnail" style={{ width: "260px", height: "260px" }} />
                <p>{selectedFile.display_name}</p>
                <p>Type: {selectedFile.type}</p>
                <p>Last Modified: {new Date(selectedFile.m_timestamp * 1000).toLocaleString()}</p>
                <p>System name: {selectedFile.name}</p>


                <div className="button-row" style={{ display: "flex", justifyContent: "space-between", marginTop: "20px" }}>
                  <button className="delete-btn" onClick={handleDeleteButtonClick}>
                    Delete File
                  </button>
                  <button className="print-btn" onClick={handlePrintButtonClick} style={{ visibility: jobInfo ? "hidden" : "visible" }}>
                    Print File
                  </button>
                </div>
              </div>
            </div>
          )}



          {jobInfo ? (
            <div style={{ display: "grid", gridTemplateColumns: "400px 400px 400px", gridGap: "16px" }} >
              <div style={{ paddingTop: "50px", textAlign: "left", paddingLeft: "16px", gridColumn: "1" }} >
                <img src={jobInfo?.file?.refs?.thumbnail} alt="Thumbnail" style={{ width: "300px", height: "300px", paddingLeft: "45px" }} />
                <p style={{ color: "black", fontSize: "16px" , paddingLeft:"45px"}}>{jobInfo?.file?.display_name}</p>
                <p>{jobInfo?.file?.m_timestamp}</p>
              </div>
              <div style={{ gridColumn: "2", paddingTop: "35px"}}>
                <div className="button-row">                  
                      <button className="stop-btn" style={{ visibility: printStop ? "visible" : "hidden" }} onClick={() => handleStopPrintClick()}>
                        Stop
                      </button>                    
                      <button className="pause-btn" style={{ visibility: printPause ? "visible" : "hidden" }} onClick={() => handlePausePrintClick()}>
                        Pause
                      </button>                  
                      <button className="resume-btn" style={{ visibility: printResume ? "visible" : "hidden" }} onClick={() => handleResumePrintClick()}>
                        Resume
                      </button>                             
                </div>
                {
                  (status && status.job) && 
                    <div className="job-progress" style={{gridRow:"2", gridColumn:"1", textAlign:"left", paddingLeft:"15px"}} >
                      <p>Print progress</p>
                      <progress value={status?.job.progress_abs} style={{height:"54px"}} /> 
                      <p>Print progress: {status?.job.progress}%</p>
                      <p>Job time elasped: {status?.job.time_print_string}</p>                                            
                      <p>Print time remaining: {status?.job.time_remaining_str}</p>
                    </div>                
                  }                
              </div>


            </div>
          ) : (<div style={{height:"380px", width:"100%"}} ></div>)
          }


          {
            status ? (<TempertureChart readings={temperatureReadings} />) : (<p></p>)
          }






        </div>


      </div>
    </div>
  );
}