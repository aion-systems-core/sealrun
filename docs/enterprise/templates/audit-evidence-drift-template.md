# Audit evidence template — Drift

Document drift disposition whenever capsule outputs differ from an approved baseline, including promotion decisions and governance outcomes.

## Record metadata

| Field | Value |
|-------|--------|
| Evidence ID | |
| Date (UTC) | |
| Tenant ID | |
| Operator or automation ID | |
| Related incident or change ID | |

## Comparison inputs

| Field | Value |
|-------|--------|
| Baseline reference | Capsule path, digest, or build ID |
| Compared run references | Candidate capsule(s) or job IDs |
| Comparison method | Tooling version, diff command, or report path |
| Policy bundle in effect | |

## Drift analysis

| Field | Value |
|-------|--------|
| Drift categories observed | e.g. tokenization, dependency version, data feed |
| Severity | Low / medium / high / critical |
| Tolerance decisions | Thresholds and business rationale |
| SIEM / OTel signal IDs | If exported for monitoring correlation |

## Final disposition

| Field | Value |
|-------|--------|
| Final disposition | Accept / reject / escalate |
| Governance decision reference | Link to governance decision template or ticket |
| Reviewer | |
| Follow-up actions | Policy update, exception, engineering fix |

## Cross-references

- [Drift anomaly runbook](../runbooks/incident-drift-anomaly.md)
- [Policy engine](../../policy-engine.md), [SIEM and OTel](../../siem-otel.md)
