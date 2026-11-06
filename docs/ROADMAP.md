# Roadmap and Release Gates

| Gate | Scope | Exit evidence |
|---|---|---|
| G0 | Architecture, threat model, VM blueprint, API contracts | Schema validation, contract review |
| G1 | Linux KVM/libvirt worker | Real create/start/stop/destroy lifecycle tests on dedicated host |
| G2 | Windows Hyper-V worker | Real lifecycle tests on supported Windows host |
| G3 | Unified scheduler and worker protocol | Same blueprint scheduled to compatible workers |
| G4 | Browser desktop access | Authenticated RDP/VNC session, keyboard/mouse E2E |
| G5 | agent-desktop Guest Bridge | Snapshot/action/verify round-trip in Windows and Linux guests |
| G6 | Security lab | Network isolation, controlled egress, snapshot and evidence tests |
| G7 | Release hardening | Threat-model tests, load tests, recovery tests, reproducible release bundle |

## Current status

Initial repository scaffold only. No real hypervisor lifecycle, mTLS worker transport, desktop session, AI automation, malware execution or security isolation claim is considered complete until the corresponding test runs on target hosts and logs are retained.

## QA reporting

For each requirement report: status (complete/partial/not implemented/failed), exact command and input, expected vs actual, logs, metrics, regression impact and retest result. Never infer successful VM execution from an accepted API request.
