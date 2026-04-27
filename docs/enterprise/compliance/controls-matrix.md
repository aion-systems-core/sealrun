# Controls matrix (SOC 2–style design reference, non-certified)

## Overview

This matrix is a **control design reference** for SealRun enterprise capabilities. It does **not** assert SOC 2 Type II or any formal certification. Use it to map product and documentation **evidence** to common trust-service criteria themes when building a customer-controlled control library.

## How to read the matrix

- **Implementation evidence** lists representative commands, artifacts, or documents. Your organization must define sampling frequency and owners.
- Pair each control with **replay**, **drift**, and **evidence chain** narratives where integrity or availability claims are made.
- **Tenant isolation**, **RBAC**, **OIDC**, **SIEM** / **OTel**, **attestation**, and **SBOM** are cross-cutting evidence themes referenced throughout.

| Control ID | Domain | Control objective | Implementation evidence (examples) | Suggested frequency | Typical owner |
|------------|--------|---------------------|--------------------------------------|---------------------|---------------|
| CC-01 | Access | Restrict privileged actions to authorized roles | `sealrun enterprise rbac export`; change tickets for `rbac.policy.yaml` | Continuous / quarterly review | Security |
| CC-02 | Access | Authenticate enterprise users via OIDC | `sealrun enterprise auth status`; IdP configuration records | Continuous | Security |
| CC-03 | Change management | Enforce tracked and test-validated releases | CI logs; `cargo test --workspace --all-targets`; changelog and release records | Per release | Engineering |
| CC-04 | Availability / integrity | Detect and triage replay and drift failures | Runbooks under `docs/enterprise/runbooks/`; incident tickets; replay and drift artifacts | Continuous | SRE |
| CC-05 | Integrity | Preserve deterministic replay and evidence continuity | Replay outputs; evidence index snapshots; policy evaluation JSON | Continuous | Platform |
| CC-06 | Confidentiality | Preserve tenant isolation at the storage layer | Tenant index configuration; isolation tests; tenant isolation runbook | Continuous | Platform |
| CC-07 | Security / privacy | Enforce retention, purge, and legal hold | Lifecycle CLI transcripts; hold/disable audit trail | Daily / per change | SRE |
| CC-08 | Monitoring | Export governance events to SIEM and OTel | Sink `send-test` results; OTel export logs; collector health | Continuous | SRE |
| CC-09 | Supply chain | Produce signed release attestations and SBOM | Cosign sign and verify transcripts; SBOM objects; release template | Per release | Release engineering |
| CC-10 | Governance | Validate policy constraints before acceptance | `sealrun enterprise policy-api validate` and `evaluate` outputs; governance decision records | Continuous | Compliance / platform |
| CC-11 | Vendor risk | Assess third-party dependencies and tooling | Vendor register; DPAs; `docs/policies/vendor-third-party-risk-policy.md` | Quarterly | Security |
| CC-12 | Incident response | Execute defined response processes with SLAs | Incident tickets; SLA reports; post-incident reviews | Per incident | SRE |

## Related documents

- ISO 27001 Annex A mapping (non-certified): `docs/enterprise/compliance/iso27001-annex-a-mapping.md`
- Policies: `docs/policies/`
- Trust center hub: [Trust Center](../../trust-center.md)
