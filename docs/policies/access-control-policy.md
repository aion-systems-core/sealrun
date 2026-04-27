# Access control policy

## Purpose

Define **authentication**, **authorization**, and **tenant** boundary controls for SealRun enterprise environments so that **capsule**, **replay**, **drift**, and **evidence chain** operations meet least-privilege expectations.

## Scope

Applies to all personnel and automation that administer SealRun enterprise features, including **OIDC**-authenticated operators, CI service accounts, and integration runtime principals.

## Policy statements

1. **Authentication:** Privileged enterprise actions require a strong, attributable identity. **OIDC** is the default enterprise authentication mechanism for interactive CLI use (see [OIDC auth](../oidc-auth.md)).
2. **Authorization:** Access is **RBAC**-governed using roles `admin`, `auditor`, `operator`, and `viewer` with explicit permissions (see [RBAC](../rbac.md)).
3. **Least privilege:** Grant the minimum role required; use break-glass `admin` only when documented.
4. **Tenant boundaries:** **Tenant isolation** must be enforced for storage, **replay** lookup, and **evidence** queries (see [Multi-tenancy](../multi-tenancy.md)).
5. **Logging and evidence:** Access grants and material changes produce auditable artifacts (exports, change tickets, **SIEM** / **OTel** events where configured).

## Roles and responsibilities

| Role | Responsibility |
|------|----------------|
| Security | Owns policy, periodic access reviews, **OIDC** integration posture. |
| Platform engineering | Implements **RBAC** file changes through approved change paths. |
| SRE / operations | Executes tenant lifecycle and replay diagnostics within assigned **RBAC** permissions. |

## Operational procedures

- Run `sealrun enterprise auth status` during shift handoffs.
- Run `sealrun enterprise rbac export` after assignment changes; archive with access review records.
- On suspected **tenant isolation** failure, execute `docs/enterprise/runbooks/incident-tenant-isolation-breach-attempt.md`.

## Evidence and monitoring

- **OIDC** IdP audit logs correlated to enterprise CLI usage.
- `rbac export` snapshots; version history for `rbac.policy.yaml`.
- **Governance decision** records when temporary elevation or **exceptions** are used (`exceptions-policy.md`).

## Compliance references

- `docs/enterprise/compliance/controls-matrix.md` (CC-01, CC-02, CC-06).
- `docs/enterprise/compliance/iso27001-annex-a-mapping.md` (people and technological controls).
- Hub: [Trust Center](../trust-center.md), [Security guide](../security-guide.md).
