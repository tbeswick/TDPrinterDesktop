
import "./layout.css";
import { useState } from "react";


const [showAbout, setShowAbout] = useState<boolean | null>(null);


async function handleAboutClick(){
    setShowAbout(true);
}



export default function Overview() {
<div>
            <div style={{flex:"1",textAlign:"left"}}>
                <img
                  src="logo.png"
                  alt="Logo"
                  style={{ width: "140px", height: "140px"  }}
                />
            </div>


            <div style={{flex:"1"}} >
              <p
                style={{
                  fontSize: "1.8rem",
                  color: "#f5f1f1",
                }}
              >
                3D Printer Manager
              </p>
            </div>

              <div style={{flex:"1", textAlign:"right"}}>
                <a href="#" onClick={handleAboutClick} style={{paddingRight:"30px", textDecoration:"none", fontSize:"24px", color:"#ccd5ee"}} > version info </a>
              </div>
</div>
}


