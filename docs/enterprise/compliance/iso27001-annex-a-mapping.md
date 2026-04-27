# ISO 27001 Annex A mapping (design reference, non-certified)

## Overview

This document provides a **best-effort** mapping from Annex A themes to SealRun documentation and **evidence** sources. It does **not** claim ISO 27001 certification. Certification remains the responsibility of the implementing organization and its auditors.

## Mapping table

| Annex A theme (high level) | SealRun mapping | Primary artifacts and documents |
|----------------------------|-----------------|-----------------------------------|
| Organizational controls | Risk, exceptions, vendor governance | `docs/policies/risk-management-policy.md`, `docs/policies/exceptions-policy.md`, `docs/policies/vendor-third-party-risk-policy.md` |
| People controls | Access provisioning and segregation of duties | `docs/policies/access-control-policy.md`, `docs/rbac.md`, RBAC export outputs |
| Physical controls | Facility and hardware security | Out of scope for open-core software; operator responsibility per `docs/security-guide.md` shared responsibility |
| Technological controls | OIDC, RBAC, tenant isolation, deterministic evidence, SIEM / OTel, release attestation | `docs/oidc-auth.md`, `docs/rbac.md`, `docs/multi-tenancy.md`, `docs/siem-otel.md`, `docs/release-attestation.md`, `docs/policy-engine.md` |
| Operations security | Runbooks, lifecycle controls, escalation | `docs/enterprise/runbooks/*.md`, `docs/lifecycle-controls.md`, `docs/support-escalation-path.md`, `docs/operations-guide.md` |
| Communications security | External call policy and telemetry handling | `docs/policy-engine.md`, `docs/telemetry.md`, `docs/siem-otel.md` |
| System acquisition, development, and maintenance | Deterministic contracts and CI gates | `docs/os_contract_spec.md`, `.github/workflows/ci.yml` (reference), change management policy |
| Supplier relationships | Vendor review and monitoring | `docs/policies/vendor-third-party-risk-policy.md`, integration guides under `docs/integrations/` |
| Incident management | IR policy, SLAs, playbooks | `docs/policies/incident-response-policy.md`, `docs/sla.md`, `docs/enterprise/runbooks/*.md` |
| Business continuity | Status communications and replay-based validation | `docs/status-page-template.md`, `docs/operations-guide.md`, replay templates |

## Cross-cutting concepts

- **Capsule**, **replay**, **drift**, and **evidence chain** support integrity and availability narratives.
- **Governance decisions** and **policy evaluation** artifacts support documented approval workflows.
- **Cosign** / **Sigstore** and **SBOM** outputs support supply-chain themes when archived per release.

## Related documents

- SOC 2–style design matrix: `docs/enterprise/compliance/controls-matrix.md`
- [Trust Center](../../trust-center.md)
