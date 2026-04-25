# RBAC

## Overview

SealRun enterprise **RBAC** (role-based access control) uses a YAML policy file and a deterministic permission evaluator. **RBAC** sits alongside **OIDC authentication** and **tenant isolation**: identity proves who the subject is; roles define which **replay**, lifecycle, and administrative actions they may perform.

## Architecture

- **Policy artifact:** Assignments are stored at `sealrun_enterprise/rbac.policy.yaml` for review in version control or secure configuration stores.
- **Deterministic evaluation:** Permission checks are explicit (`sealrun enterprise rbac check`) for automation and audits.
- **Separation of duties:** `auditor` and `operator` roles are distinct from `admin` to support least privilege.

## Roles

| Role | Typical use |
|------|-------------|
| `admin` | Full enterprise administration including tenant and RBAC management. |
| `auditor` | Read-heavy access to evidence, exports, and validation without mutating production state. |
| `operator` | Day-to-day replay, diff, and controlled lifecycle actions per local policy. |
| `viewer` | Read-only visibility into permitted surfaces. |

## Permissions

| Permission | Meaning (conceptual) |
|------------|----------------------|
| `replay` | Execute tenant-scoped **replay** of capsules. |
| `diff` | Compare runs for **drift** analysis. |
| `purge` | Execute purge of expired registrations when allowed. |
| `retention-set` | Change retention policy for a tenant. |
| `legal-hold` | Enable or disable legal hold. |
| `tenant-admin` | Create, configure, or delete tenant metadata within product semantics. |

Exact CLI command gating is defined by the product; treat this table as the enterprise permission model for documentation and **policy evaluation** alignment.

## Example flows

1. **Bootstrap:** Seed `rbac.policy.yaml` with break-glass `admin`, then assign scoped roles.
2. **Audit export:** Run `rbac export` and attach to periodic access reviews.
3. **Pipeline gate:** CI or release automation calls `rbac check` before running privileged enterprise commands.
4. **Incident:** Restrict `purge` and `legal-hold` to a narrow on-call group; record **governance decision** if temporary elevation is needed (see `docs/policies/exceptions-policy.md`).

## Evidence capture points

- `sealrun enterprise rbac export` output (point-in-time role matrix).
- Assignment and check command transcripts with timestamps and subjects.
- Change records referencing RBAC file updates (see `docs/policies/access-control-policy.md`).

## Policy enforcement points

- Enterprise CLI commands consult **RBAC** before sensitive operations.
- Complements **policy engine** JSON rules: **RBAC** is who may act; the **policy engine** constrains what workloads may do (models, seeds, external calls, evidence fields).

## Integration points

- **OIDC:** Map IdP groups to SealRun role assignments in your operational procedures (see [OIDC auth](oidc-auth.md)).
- **SIEM** / **OTel:** Forward authentication and authorization failure patterns per your SOC playbooks (see [SIEM and OTel](siem-otel.md)).
- **Multi-tenancy:** Tenant-scoped commands still require appropriate permissions (see [Multi-tenancy](multi-tenancy.md)).

## Compliance notes

- Map to CC-01 in `docs/compliance/controls-matrix.md` and people/access themes in `docs/compliance/iso27001-annex-a-mapping.md`.
- **Access control policy:** `docs/policies/access-control-policy.md`.

## Next steps

- Run `rbac export` after any change; store with release **attestation** evidence when RBAC changes ship with a version (see [Release attestation](release-attestation.md)).
- Review [Trust Center](trust-center.md) capability map.

## Policy file example

Stored at `sealrun_enterprise/rbac.policy.yaml`:

```yaml
assignments:
  alice: admin
  bob: viewer
```

## CLI reference

```bash
sealrun enterprise rbac assign --subject alice --role admin
sealrun enterprise rbac check --subject alice --permission tenant-admin
sealrun enterprise rbac export
```
