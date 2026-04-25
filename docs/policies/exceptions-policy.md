# Exceptions policy

## Purpose

Define how policy and control **exceptions** are requested, approved, tracked, and expired so that **governance decisions** remain auditable when strict **policy evaluation**, **RBAC**, or operational controls cannot be met temporarily.

## Scope

Covers security, compliance, and operational **exceptions** to documented controls across SealRun enterprise programs.

## Policy statements

1. **Documentation:** Each **exception** includes business justification, scope, compensating controls, owner, and explicit end date.
2. **Approval:** Security and the owning engineering manager approve material **exceptions**; regulatory programs may require additional approvers aligned to bundle fields such as `approver` / `reviewer` (`docs/governance/bundles/`).
3. **Time bounding:** **Exceptions** expire automatically; renewal requires fresh risk acceptance.
4. **Non-compliance:** Expired **exceptions** without renewal are treated as non-compliant configurations and must be remediated or emergency-approved under incident policy.
5. **Evidence:** **Exception** records link to tickets, **governance decision** artifacts, and monitoring proving compensating controls operated.

## Exception record template

| Field | Description |
|-------|-------------|
| Exception ID | Unique identifier. |
| Control / policy reference | e.g., CC-10, access-control-policy.md section. |
| Risk statement | Inherent and residual risk summary. |
| Compensating controls | Monitoring, manual review, narrowed blast radius. |
| Owner | Named accountable engineer. |
| Approval date | ISO-8601 date. |
| Expiry date | Must be before next access review where applicable. |

## Compliance references

- `docs/compliance/controls-matrix.md`.
- Audit template: `docs/templates/audit-evidence-governance-decision-template.md`.
