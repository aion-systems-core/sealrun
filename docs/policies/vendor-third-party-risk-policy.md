# Vendor and third-party risk policy

## Purpose

Manage security and operational risk introduced by external vendors and dependencies that touch SealRun workloads, **OIDC** identity providers, **SIEM** / **OTel** pipelines, cloud runtimes (for example **Modal**), model hubs (**Hugging Face**), or **Cosign** / **Sigstore** infrastructure.

## Scope

Applies to production-critical vendors, subprocessors with data processing agreements, and tooling in signing, scanning, and attestation paths.

## Policy statements

1. **Criticality-based review:** New critical vendors undergo security and resilience review before production use.
2. **Contractual controls:** Review data handling, subprocessors, breach notification, and support SLAs.
3. **Re-assessment:** High-risk vendors are re-assessed at least annually or after material incidents.
4. **Register:** Maintain a vendor register listing IdP, **SIEM**, collectors, model providers, and signing infrastructure owners.
5. **Exceptions:** Vendor risk exceptions follow `docs/policies/exceptions-policy.md` with compensating controls and expiry.

## Minimum review fields

| Field | Notes |
|-------|--------|
| Service criticality | Impact to **replay**, **evidence**, or authentication availability. |
| Data classification | Whether prompts, **capsule** payloads, or telemetry contain regulated data. |
| Authentication model | **OIDC**, API keys, mTLS; alignment with [OIDC auth](../oidc-auth.md) and [RBAC](../rbac.md). |
| Compliance posture | Certifications relevant to your industry (customer-owned assessment). |
| Failure and contingency | Fallback for **SIEM** / **OTel** outage; see exporter runbook. |

## Evidence

- Completed vendor questionnaires and DPIAs where applicable.
- DPA execution records.
- Links to **SBOM** or dependency scans for software vendors.

## Compliance references

- `docs/compliance/controls-matrix.md` (CC-11).
- `docs/compliance/iso27001-annex-a-mapping.md` (supplier relationships).
- Integration guides: `docs/integrations/`.
