# AI Multi-OS Lab

Independent, security-focused multi-OS virtualization and AI desktop automation platform.

## Project boundary

- This repository is independent from `eentost/agent-desktop`.
- `agent-desktop` remains an external, unmodified GUI automation dependency.
- Linux virtualization is managed through QEMU/KVM and libvirt.
- Windows virtualization is managed through Hyper-V and a constrained PowerShell worker.
- Browser desktop access is provided through Apache Guacamole.
- Local LLM inference is an optional integration; no external paid model API is required.

## Current implementation status

This is the initial G0/G1 foundation scaffold, not a production-ready virtualization platform.

| Area | Status | Evidence |
|---|---|---|
| Architecture and security contracts | Implemented as documentation | `docs/` |
| VM Blueprint schema | Implemented | `contracts/vm-blueprint.schema.json` |
| OpenAPI contract | Initial contract | `contracts/openapi.yaml` |
| Controller API | Health/version/capability skeleton | `backend/` |
| Linux Worker | Safe `virsh` command adapter skeleton | `worker/linux/` |
| Windows Worker | Allowlisted Hyper-V PowerShell entrypoint | `worker/windows/` |
| React console | Initial UI shell | `frontend/` |
| Compose deployment | Development service topology | `deploy/compose/` |
| Real VM lifecycle / mTLS / GUI automation / telemetry | Not yet implemented or verified | Release gates G1-G6 |

No VM operation is reported as successful until a real worker confirms the resulting hypervisor state.

## Repository layout

```text
backend/                 Rust control-plane API
worker/linux/            Linux KVM/libvirt worker adapter
worker/windows/          Windows Hyper-V constrained worker
frontend/                React + TypeScript management console
contracts/               OpenAPI and JSON schemas
deploy/compose/          Controller-side Docker Compose
docs/                    Architecture, threat model, roadmap
.github/workflows/       CI quality gates
```

## Development prerequisites

- Rust 1.89+ for backend and Linux worker
- Node.js 22+ and npm for frontend
- Docker Compose v2 for controller services
- Linux Worker: Ubuntu 24.04+, QEMU/KVM, libvirt, `virsh`
- Windows Worker: supported Windows edition with Hyper-V and PowerShell 7 or Windows PowerShell 5.1 compatibility validation

## Quick start: controller services

```bash
cd deploy/compose
cp .env.example .env
# Set unique strong values for every secret before starting services.
docker compose config
docker compose up -d
```

The Compose file intentionally does not publish database, NATS, MinIO, or Guacamole management ports to the host. A production deployment must add TLS termination, authentication, network policies, secret management, health checks, resource limits, and persistent backup procedures.

## Run the controller API locally

```bash
cd backend
cargo run
```

Default listen address: `127.0.0.1:8080`.

## Run the Linux worker adapter

```bash
cd worker/linux
cargo run -- capabilities
cargo run -- list
cargo run -- start lab-ubuntu-01
```

The worker invokes `virsh` with argument arrays (never through a shell). It does not expose a network listener in this initial milestone. Destructive operations are intentionally excluded from the first adapter.

## Frontend

```bash
cd frontend
npm install
npm run dev
```

## Security baseline

- Isolated networks are the default for experiment VMs.
- No host shared folders, clipboard, USB passthrough, or direct management-network access by default.
- No arbitrary shell or PowerShell command endpoint.
- Guest telemetry is treated as potentially untrusted; host-side and network-side evidence remain separate.
- Artifact export requires a controlled quarantine workflow.
- Do not run live malware on a general-purpose workstation or an internet-connected production host.

See [Security Design](docs/SECURITY.md) and [Architecture](docs/ARCHITECTURE.md).

## License

Apache-2.0. See `LICENSE`.
