# Audit evidence template — Replay

Use this template when recording replay evidence for access reviews, change approvals, or post-incident reconstruction. Store completed copies with the change or incident record. UTF-8 text only; redact secrets.

## Record metadata

| Field | Value |
|-------|--------|
| Evidence ID | |
| Date (UTC) | |
| Environment | |
| Tenant ID | |
| SealRun / CLI version | `sealrun --version` output |
| Operator subject | OIDC subject or service principal ID |
| RBAC role at time of run | From `sealrun enterprise rbac export` snapshot |

## Capsule and policy context

| Field | Value |
|-------|--------|
| Capsule path or artifact digest | |
| Policy JSON digest or bundle name | e.g. `default`, `strict`, `regulated-finance` |
| Policy evaluation reference | Path or ID to `policy-api evaluate` output |
| Release attestation ID (if required) | From strict bundle or internal CMDB |
| SBOM reference (if required) | Link to stored SBOM object |

## Replay execution

| Field | Value |
|-------|--------|
| Replay command (verbatim) | |
| Exit code | |
| Replay result summary | Pass / fail; one-paragraph narrative |
| Determinism checks | e.g. hash of outputs, diff against baseline |
| Linked drift report (if any) | Reference to drift template |

## Review and approval

| Field | Value |
|-------|--------|
| Reviewer name | |
| Reviewer role | |
| Approval status | Approved / rejected / waived under exception ID |
| Exception ID (if applicable) | Link to `docs/policies/exceptions-policy.md` record |

## Cross-references

- [Replay failure runbook](../runbooks/incident-replay-failure.md)
- [Multi-tenancy](../../multi-tenancy.md), [Policy engine](../../policy-engine.md), [Release attestation](../../release-attestation.md)
