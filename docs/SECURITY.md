# Security Design

## Default-deny baseline

- Lab networks default to no external egress.
- Host management interfaces are not reachable from guest networks.
- No shared folders, clipboard sharing, USB passthrough or host filesystem mounts by default.
- Worker operations are typed and allowlisted; arbitrary shell/PowerShell endpoints are forbidden.
- Image registration records SHA-256 and provenance.
- Evidence is stored outside disposable guest disks and integrity manifests are generated.
- Guest-originated telemetry is treated as potentially compromised.
- Destructive operations and artifact export require explicit policy checks and audit records.

## Network profiles

- AIRGAP: no guest-to-external or guest-to-management connectivity.
- ISOLATED: explicitly selected lab VMs may communicate; no external egress.
- SIMULATED: guest traffic is routed only to lab DNS/HTTP/C2 simulation services.
- CONTROLLED_EGRESS: disabled by default; explicit destination allowlist and approval required.

## Threats to address

Guest-to-host escape, hypervisor vulnerabilities, malicious disk images, management socket exposure, worker credential theft, remote desktop session hijacking, guest-to-guest lateral movement, malicious artifact export, prompt injection through guest content, and evidence tampering.

## Operational warning

A VM snapshot is not a security boundary and cannot undo network side effects or data exfiltration. Run high-risk samples only on dedicated, patched, isolated hosts with a tested containment and recovery procedure. Do not use a general-purpose workstation or production-connected host as a malware detonation environment.

## Evidence provenance

Every artifact must record experiment ID, VM/image identity, source plane (guest/host/network/GUI/AI), collection timestamp, collector version and SHA-256. Do not merge AI inference with raw observations.
