import { useEffect, useState } from "react";
import { type PrinterStatus } from "../types/printerstatus";

export default function Overview() {
//   const [status, setStatus] = useState<PrinterStatus | null>(null);
//   useEffect(() => {
//     let mounted = true;
//     // schedule the fetch on the microtask queue to avoid synchronous setState in effect
//     Promise.resolve().then(async () => {
//       const data = await getPrinterStatus();
//       if (mounted) setStatus(data);
//     });
//     return () => {
//       mounted = false;
//     };
//   }, []);

  return (
    <div>
      

      <div>
        {/* <p>State: {status?.printer?.state}</p>
        <p>Bed Temp: {status?.printer?.temp_bed}</p>
        <p>Nozzle Temp: {status?.printer?.temp_nozzle}</p>
        <p>Target Nozzle Temp: {status?.printer?.target_nozzle}</p>  */}
      </div>
    </div>
  );
}