# Audit evidence template — Governance decision

Capture machine-readable and human-readable governance decision records after policy evaluation, including violations and any approved exceptions.

## Record metadata

| Field | Value |
|-------|--------|
| Evidence ID | |
| Date (UTC) | |
| Tenant ID | |
| Capsule or workload reference | |
| OIDC subject or automation principal | |

## Policy context

| Field | Value |
|-------|--------|
| Policy bundle name | e.g. `regulated-healthcare` |
| Policy JSON version or digest | |
| Policy API output reference | Path to `validate` / `evaluate` JSON |
| Required evidence fields asserted | List from policy JSON |

## Decision outcome

| Field | Value |
|-------|--------|
| Decision outcome | Pass / fail / conditional pass |
| Violations (if any) | Structured list or attachment |
| Compensating monitoring (if conditional) | |

## Exceptions and approvals

| Field | Value |
|-------|--------|
| Exception link (if applicable) | Exception ID per `docs/policies/exceptions-policy.md` |
| Approver (if bundle requires) | Matches `approver` or delegated control owner |
| Reviewer (if bundle requires) | Matches `reviewer` field for healthcare-style bundles |
| Timestamp fields | e.g. `decision_timestamp` for finance-style bundles |

## Cross-references

- [Policy engine](../policy-engine.md)
- [RBAC](../rbac.md), [Release attestation](../release-attestation.md) when strict bundle ties to attestation ID
