import { Activity, Bot, Cpu, HardDrive, Monitor, Plus, Server, ShieldCheck } from 'lucide-react';

const metrics = [
  { label: 'Registered VMs', value: '00', icon: Server, note: 'No workers connected' },
  { label: 'Running VMs', value: '00', icon: Monitor, note: 'Awaiting hypervisor' },
  { label: 'Active experiments', value: '00', icon: Activity, note: 'No active experiments' },
  { label: 'Worker capacity', value: '--', icon: Cpu, note: 'Telemetry unavailable' },
];

export default function App() {
  return <div className="shell">
    <aside className="sidebar">
      <div className="brand"><div className="brand-mark"><ShieldCheck size={20}/></div><div><strong>AI Multi-OS Lab</strong><small>SECURITY RESEARCH</small></div></div>
      <div className="nav-label">WORKSPACE</div>
      <a className="nav active"><Activity size={17}/> Overview</a>
      <a className="nav"><Server size={17}/> OS Instances</a>
      <a className="nav"><Monitor size={17}/> Live Desktop</a>
      <a className="nav"><Bot size={17}/> AI Agent</a>
      <a className="nav"><ShieldCheck size={17}/> Security Lab</a>
      <a className="nav"><HardDrive size={17}/> Images & Snapshots</a>
      <div className="sidebar-foot">Independent platform<br/>agent-desktop integration</div>
    </aside>
    <main className="main">
      <header><span>Workspace / Overview</span><span className="environment"><i/> CONTROL PLANE SCAFFOLD</span></header>
      <section className="heading"><div><p className="eyebrow">MULTI-OS VIRTUALIZATION</p><h1>Lab Overview</h1><p className="muted">Manage isolated operating systems and AI-assisted security experiments.</p></div><button disabled><Plus size={16}/> Create OS</button></section>
      <section className="metrics">{metrics.map(({label,value,icon:Icon,note})=><article className="metric" key={label}><div className="metric-top"><span>{label}</span><Icon size={18}/></div><strong>{value}</strong><small>{note}</small></article>)}</section>
      <section className="grid">
        <article className="panel"><div className="panel-title"><div><h2>OS Instances</h2><p>Virtual machines registered in this lab</p></div><span className="tag">0 INSTANCES</span></div>
          <div className="empty"><div className="empty-icon"><Server size={25}/></div><strong>No virtual machines yet</strong><p>Connect a Linux KVM or Windows Hyper-V worker to begin provisioning.</p></div>
        </article>
        <article className="panel"><div className="panel-title"><div><h2>Worker Nodes</h2><p>Mixed-host virtualization capacity</p></div></div>
          <div className="worker"><div className="worker-icon linux">L</div><div><strong>Linux KVM Worker</strong><small>QEMU · KVM · libvirt</small></div><span className="offline">NOT CONNECTED</span></div>
          <div className="worker"><div className="worker-icon windows">W</div><div><strong>Windows Hyper-V Worker</strong><small>Hyper-V · PowerShell</small></div><span className="offline">NOT CONNECTED</span></div>
          <div className="notice">Worker connectivity and live resource metrics are not implemented in this scaffold.</div>
        </article>
      </section>
      <section className="panel lower"><div className="panel-title"><div><h2>Security baseline</h2><p>Default-deny experiment environment</p></div><span className="tag">POLICY DESIGN</span></div><div className="baseline"><span>Guest-to-host access</span><b>Denied by design</b><span>External network egress</span><b>Disabled by default</b><span>Shared folders / clipboard</span><b>Disabled by default</b><span>Evidence integrity</span><b>SHA-256 planned</b></div></section>
      <footer>AI Multi-OS Lab · Initial development scaffold · No live VM operations available yet</footer>
    </main>
  </div>;
}