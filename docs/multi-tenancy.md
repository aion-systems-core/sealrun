# Multi-tenancy

## Overview

SealRun enterprise storage is **tenant-aware** and **storage-isolated**. Every **capsule** registration, **evidence** index entry, and replay or drift operation runs in an explicit **tenancy** context so auditors and operators can prove boundary enforcement without relying on implicit filesystem layout alone.

## Architecture

- **Tenant binding:** Each capsule belongs to exactly one tenant for its lifetime in the enterprise store.
- **Partitioned indexes:** Capsule indexes and **evidence chain** indexes are persisted per tenant, reducing cross-tenant blast radius and simplifying legal discovery.
- **Lifecycle coupling:** **Tenant isolation** interacts with retention, purge, and legal hold (see [Lifecycle controls](lifecycle-controls.md)).
- **RBAC and OIDC:** Authorization and **OIDC authentication** gate tenant administration and sensitive operations (see [RBAC](rbac.md), [OIDC auth](oidc-auth.md)).

## Example flows

1. **Onboard a tenant:** Create tenant metadata, assign RBAC roles, configure retention defaults.
2. **Register workloads:** Run governed executions; capsules and evidence land in the tenant partition.
3. **Investigate or audit:** List tenant capsules, run **replay**, query evidence by field, export governance events to **SIEM** or **OpenTelemetry (OTel)** (see [SIEM and OTel](siem-otel.md)).
4. **Offboard:** Disable new registrations, apply retention, purge when allowed; legal hold blocks purge and tenant deletion.

## Evidence capture points

- Tenant identifier on every stored capsule and evidence record suitable for audit export.
- CLI transcripts and JSON envelopes from `enterprise tenants *` and `enterprise tenants evidence query` for change and access records.
- **Governance decision** and **policy evaluation** artifacts that reference `tenant_id` in required evidence fields (see [Policy engine](policy-engine.md), governance bundles under `docs/governance/bundles/`).

## Policy enforcement points

- Tenant-scoped CLI surfaces (list, replay, evidence query) enforce context before reads or writes.
- **RBAC** permissions such as `tenant-admin` gate destructive or cross-cutting actions.
- Legal hold and retention **policy evaluation** outcomes are observable via lifecycle commands.

## Integration points

- **OIDC**-authenticated operators invoking tenant-scoped enterprise commands.
- Observability: tenant context should appear in exported governance/capsule events where your sink mapping supports it (see [SIEM and OTel](siem-otel.md)).
- **Release attestation** and **SBOM** evidence stored alongside deployment records per your change process (see [Release attestation](release-attestation.md)).

## Compliance notes

- Map tenant isolation to access and confidentiality objectives in `docs/compliance/controls-matrix.md` and Annex A technological controls in `docs/compliance/iso27001-annex-a-mapping.md`.
- Cross-tenant access attempts are covered in `docs/runbooks/incident-tenant-isolation-breach-attempt.md`.
- This document does not replace host-level isolation; align with [Security guide](security-guide.md) shared responsibility.

## Next steps

- Configure **RBAC** assignments and export for review: [RBAC](rbac.md).
- Define retention, purge, and legal hold: [Lifecycle controls](lifecycle-controls.md).
- Wire **SIEM** / **OTel** export for tenant-scoped governance streams: [SIEM and OTel](siem-otel.md).
- Hub: [Trust Center](trust-center.md), [Operations guide](operations-guide.md).

## CLI reference

```bash
sealrun enterprise tenants list
sealrun enterprise tenants create <id>
sealrun enterprise tenants delete <id>
sealrun enterprise tenants capsules list --tenant <id>
sealrun enterprise tenants capsules replay --tenant <id> --capsule <path>
sealrun enterprise tenants evidence query --tenant <id> --field <k> --value <v>
```
