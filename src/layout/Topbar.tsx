export default function Topbar() {
  return (
    <header className="topbar">
      <div style={{ flex: 1 }}>TDPrinter Desktop</div>

      <div style={{fontSize:"16px"}}>
        Status: <b>Connected</b>
      </div>
    </header>
  );
}