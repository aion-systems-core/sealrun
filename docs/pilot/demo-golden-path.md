# Demo golden path (pilot)

## Purpose

A **deterministic**, repeatable story for executive or security demos: run → **replay** → **drift** → **policy evaluation** → **evidence** export → **SIEM** visibility. Same script before and after the pilot week to show stability. Follow [data classification](data-classification.md) and [secrets handling](secrets-handling.md).

## Storyboard (narrative)

1. **Execute** a bounded workload that produces a capsule (use product flow appropriate to your pilot scope; optional primer: [execute capsule](01_execute_capsule.md)).
2. **Replay** the capsule and show symmetry or controlled explanation of differences.
3. **Drift** compare candidate vs golden baseline; show disposition (accept / investigate).
4. **Policy** validate and evaluate JSON; show deny path on a crafted negative input.
5. **Evidence** attach replay and policy outputs to an audit ticket template ([replay template](../templates/audit-evidence-replay-template.md), [governance template](../templates/audit-evidence-governance-decision-template.md)).
6. **SIEM** show a screenshot or saved search proving the event arrived (anonymized).

## Commands (outline — use pilot-specific paths)

```bash
sealrun doctor
# ... produce capsule per agreed tutorial or workload ...
sealrun enterprise tenants capsules replay --tenant <TENANT> --capsule <CAPSULE_PATH>
sealrun enterprise policy-api validate --policy policy.json
sealrun enterprise policy-api evaluate --policy policy.json --input input-pass.json
sealrun enterprise policy-api evaluate --policy policy.json --input input-fail.json
sealrun enterprise sinks send-test --sink <sink> --endpoint <URL> --token <from-env-only>
```

Drift step may use your internal diff tooling or documented drift flow ([drift tutorial](03_drift_analysis.md) if applicable).

## Determinism rules for the demo

- Fixed **seed**, fixed model allow-list, fixed inputs from synthetic dataset.
- Same **policy.json** and bundle version as [bill of materials](bill-of-materials.md).
- Same **tenant** for the entire demo.

## Anonymization rules

- No real customer names, emails, or production URLs in slides unless approved.
- SIEM screenshot: crop to show only agreed fields; blur tenant names if using production-like IDs.
- Replace timestamps in slides with relative labels if needed for consistency.

## Expected outputs (checklist)

- [ ] `doctor` JSON healthy.
- [ ] Replay exit success with summary suitable for slide.
- [ ] Drift narrative one slide (expected vs unexpected).
- [ ] Policy evaluate: one pass, one fail with readable violations list.
- [ ] Evidence templates filled with redacted values.
- [ ] SIEM or collector screenshot with mapping per [monitoring minimum](monitoring-minimum.md).

## Related documents

- [Onboarding script](onboarding-script.md) · [Success criteria](success-criteria.md) · [Trust Center](../trust-center.md)
