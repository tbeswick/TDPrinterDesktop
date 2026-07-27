import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";


interface PrinterStatus  {
  printer: {
    connected?: boolean;
    state?: string;
    temperature?: number;
    temp_nozzle?: number;
    bed_temp?: number;
    progress?: number;
    time_remaining?: number;
  }
};



function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [greet2Msg, setGreet2Msg] = useState("");
  const [name, setName] = useState("");


      // await listen("printer-status", (event) => {
      //     console.log(event.payload);
      // });  



  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

    try{

      const result = await invoke("greet", { name });

      setGreetMsg(await invoke("greet", { result }));
    }catch(error){
      console.error("Error invoking greet command:", error);
      setGreetMsg(`Error: ${error}`);
    }
  }


  async function greet2() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreet2Msg(await invoke("greet2", { name }));
  }


 const [status, setStatus] = useState<PrinterStatus | null>(null);


  useEffect(() => {
    let unlisten: (() => void) | undefined;

    listen<PrinterStatus>("printer-status-update", (event) => {
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


  // async function get_api_status(ip: string, api_key: string) {
  //   try {
  //     const status = await invoke("get_printer_status", { ip, api_key });
  //     console.log(status);
  //   } catch (error) {
  //     console.error("Error fetching printer status:", error);
  //   }
  // }

  return (

    



    <main className="container">
      <h1>Welcome to Tauri + React</h1>

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


      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
          greet2();
        //  get_api_status("192.168.1.76", "kpiTr8FC6WmrsJh");
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>


        <input
          id="greet2-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet2</button>


      </form>
      <p>{greetMsg}</p>
      <p>{greet2Msg}</p>
    </main>
  );
}

export default App;
