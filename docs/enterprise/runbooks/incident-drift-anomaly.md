# Runbook: Incident — Drift anomaly

## Overview

Drift compares runs to expose meaningful deltas when replay succeeds but outputs differ from baselines or expectations. This runbook covers unexpected or high-severity drift during production monitoring or pre-release validation.

## Trigger

- Drift report exceeds tolerance thresholds or flags unknown categories.
- Governance workflow receives unclassified drift during promotion.

## Detection

- Scheduled diff jobs between golden and candidate capsules.
- Policy or quality gates surfacing drift metrics to SIEM / OTel dashboards.

## Impact

- May block release until governance decision is recorded.
- May indicate dependency version skew, data feed changes, or partial non-determinism.

## Mitigation

1. Snapshot baseline and compared capsule artifacts plus the drift report.
2. Classify drift as expected (documented change), unknown (investigate), or critical (halt rollout).
3. Review active governance bundle constraints (`docs/enterprise/governance/bundles/`) and policy evaluation history.
4. If critical on governed surfaces, pause rollout and notify stakeholders per `docs/status-page-template.md` if customer-visible.

## Verification

- Repeat comparison after pinning identified variables (dependency version, seed, model ID).
- Confirm SIEM / OTel pipelines show resumed healthy export if diagnostics required sink tests ([SIEM and OTel](../../siem-otel.md)).

## Escalation

- Product / platform owner for classification when cause spans application logic.
- Compliance for regulated bundles requiring `approver` or `reviewer` fields.

## Post-incident

- Document final disposition (accept, reject, escalate) in `docs/enterprise/templates/audit-evidence-drift-template.md`.
- Record governance decision using `docs/enterprise/templates/audit-evidence-governance-decision-template.md` when policy posture changes.
