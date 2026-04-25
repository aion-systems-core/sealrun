# Lifecycle controls

## Overview

**Tenant** lifecycle controls define **retention**, **purge**, and **legal hold** behavior for enterprise storage. They govern how long **capsule** registrations and related **evidence** remain addressable, and when removal is permitted. Legal hold is the primary brake on destructive actions during investigations or regulatory holds.

## Architecture

- **Retention policy:** Configured per tenant (`days`). Expired registrations become eligible for purge according to product semantics.
- **Purge:** Removes expired capsule registrations for a tenant when not blocked by legal hold.
- **Legal hold:** Blocks purge and tenant deletion while enabled; supports defensible preservation of **evidence chain** continuity.
- **Interactions:** Complements **tenant isolation** (see [Multi-tenancy](multi-tenancy.md)) and **RBAC** permissions such as `purge`, `retention-set`, and `legal-hold` (see [RBAC](rbac.md)).

## Example flows

1. **Set retention:** Operator sets `--days` per tenant policy; document approval if required by your change record.
2. **Legal hold for investigation:** Enable legal hold before forensic replay or evidence export; run **replay** / **drift** checks without losing index integrity.
3. **Scheduled purge:** After retention elapses and hold is clear, run purge; archive attestations and **SBOM** / **Cosign** verification artifacts for the build in use (see [Release attestation](release-attestation.md)).
4. **Blocked purge:** If purge or tenant delete fails, verify legal hold state and **RBAC** permissions, then record a **governance decision** if an exception is required (see `docs/policies/exceptions-policy.md`).

## Evidence capture points

- Command outputs from `lifecycle retention get|set`, `legal-hold enable|disable`, and `purge`.
- Change tickets linking retention changes to approvers (finance / healthcare bundles may require `approver` or `reviewer` fields in policy evidence; see `docs/governance/bundles/`).
- Correlation IDs and `tenant_id` in **policy evaluation** inputs where configured (see [Policy engine](policy-engine.md)).

## Policy enforcement points

- **RBAC:** `retention-set`, `purge`, and `legal-hold` permissions restrict who may change lifecycle state.
- **Policy engine:** Optional `required_evidence_fields` can mandate lifecycle or approval metadata on governed runs.
- **Authentication:** **OIDC**-authenticated sessions for operators changing lifecycle in regulated environments (see [OIDC auth](oidc-auth.md)).

## Integration points

- **SIEM** / **OTel:** Export lifecycle-changing governance events to your SOC pipeline (see [SIEM and OTel](siem-otel.md)).
- **Runbooks:** Evidence corruption or replay failures may interact with retention; see `docs/runbooks/`.
- **Trust Center** evidence sources for auditors: [Trust Center](trust-center.md).

## Compliance notes

- Align retention and legal hold with records management and `docs/policies/change-management-policy.md`.
- Map to controls CC-07 and related rows in `docs/compliance/controls-matrix.md`.
- ISO 27001 mapping: operations security and incident preservation in `docs/compliance/iso27001-annex-a-mapping.md`.

## Next steps

- Document tenant-specific retention in your CMDB and link to **replay** baselines.
- Train operators on legal hold before purge: [Operations guide](operations-guide.md).
- Add audit templates: `docs/templates/audit-evidence-replay-template.md`, `docs/templates/audit-evidence-drift-template.md`.

## CLI reference

```bash
sealrun enterprise lifecycle retention get --tenant <id>
sealrun enterprise lifecycle retention set --tenant <id> --days 30
sealrun enterprise lifecycle purge --tenant <id>
sealrun enterprise lifecycle legal-hold enable --tenant <id>
sealrun enterprise lifecycle legal-hold disable --tenant <id>
```
