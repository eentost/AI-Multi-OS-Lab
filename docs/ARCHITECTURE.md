# Architecture

## Product boundary

AI Multi-OS Lab is an independent platform. The upstream `agent-desktop` repository remains unchanged and is consumed as an external guest-side GUI automation dependency.

## Planes

1. **Control plane** — Rust API, VM blueprint validation, worker registry, task orchestration, RBAC and audit.
2. **Virtualization plane** — Linux QEMU/KVM + libvirt workers and Windows Hyper-V workers.
3. **Guest automation plane** — Guest Bridge invokes the installed agent-desktop CLI/MCP locally.
4. **Access plane** — Apache Guacamole provides browser access over RDP/VNC/SSH.
5. **Security plane** — isolated virtual networks, controlled egress, host/network telemetry and evidence integrity.

## Mixed-host model

Workers advertise capabilities and accept typed, allowlisted operations. The controller schedules a blueprint only to a compatible worker. VM disk formats remain hypervisor-specific (QCOW2 for KVM, VHDX for Hyper-V); the blueprint is the portable intent, not a promise that disk images are portable.

## Trust boundaries

- Controller does not expose libvirt sockets or Hyper-V administration directly.
- Linux worker invokes fixed virsh/libvirt operations using argument arrays.
- Windows worker exposes a constrained operation set; arbitrary PowerShell execution is prohibited.
- Guest telemetry is untrusted evidence. Host-side and network-side observations are stored separately.
- Guest Bridge is not a privileged host agent.
- Browser desktop sessions are authenticated and isolated from management APIs.

## Initial release boundaries

The initial scaffold is not a production virtualization service. Worker network transport, mTLS, VM creation, image import, snapshots, remote desktop provisioning, Guest Bridge, telemetry and full RBAC require implementation and verification.
