
interface TopBarProps {
  setShowSettings: React.Dispatch<React.SetStateAction<boolean>>;
}

export default function Topbar({ setShowSettings }: TopBarProps) {

  return (
    <header className="topbar">
      <div style={{ flex: 1 }}>TDPrinter Desktop</div>

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


      <div style={{fontSize:"16px"}}>
        Status: <b>Connected</b>
      </div>
    </header>
  );
}