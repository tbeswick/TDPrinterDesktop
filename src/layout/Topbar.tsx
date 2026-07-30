export default function Topbar() {
  return (
    <header className="topbar">
      <div style={{ flex: 1 }}>🖨 Prusa Printer</div>

      <div>
        Status: <b>Connected</b>
      </div>
    </header>
  );
}