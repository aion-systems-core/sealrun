# Runbook: Incident — Replay failure

## Overview

Replay verifies behavioral symmetry for a deterministic capsule. This runbook applies when replay fails for an expected deterministic workload, threatening integrity assertions, release promotion, or audit reconstruction.

## Trigger

- Replay command exits non-success for a capsule that previously passed, or deterministic contract checks fail in CI.
- Operator reports mismatch between replay output and golden baseline without an approved drift disposition.

## Detection

- Automated replay jobs in CI/CD or scheduled SRE checks.
- Alerts tied to replay CLI exit codes or JSON envelope error fields.
- User reports during incident investigation or pre-release validation.

## Impact

- Potential blocker for deployments relying on deterministic guarantees.
- Possible evidence chain gap if replay cannot reproduce prior state for auditors.

## Mitigation

1. Capture full replay output, command line, product version (`sealrun --version`), and environment identity (`sealrun doctor` where applicable).
2. Confirm tenant context and capsule path integrity; rule out wrong-tenant invocation (see [Multi-tenancy](../../multi-tenancy.md)).
3. Run policy engine validation for the governing policy JSON to rule out configuration drift ([Policy engine](../../policy-engine.md)).
4. Compare against last known successful replay artifact; classify whether delta is benign drift or contract violation.
5. If contract breach is suspected, halt dependent rollouts and open a security review per `docs/policies/incident-response-policy.md`.

## Verification

- Replay succeeds on a control capsule in the same environment, isolating tenant-specific corruption vs platform regression.
- RBAC and configuration diffs reviewed; changes rolled back if identified as root cause.

## Escalation

- Tier 2 SRE / platform per `docs/support-escalation-path.md`.
- Tier 3 security if tampering or evidence corruption is suspected (`docs/enterprise/runbooks/incident-evidence-corruption.md`).

## Post-incident

- Root cause documented; corrective actions tracked (code fix, policy update, or approved exception).
- Attach artifacts to the incident ticket using `docs/enterprise/templates/audit-evidence-replay-template.md`.
