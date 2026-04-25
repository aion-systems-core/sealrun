# Incident response policy

## Purpose

Provide a standardized response to security, reliability, and compliance incidents affecting SealRun deterministic execution, **evidence chain** integrity, **tenant isolation**, **RBAC**, **OIDC**, **SIEM** / **OTel** export, or **release attestation** processes.

## Scope

Applies to all teams operating SealRun enterprise deployments and shared CI/CD pipelines that produce signed artifacts.

## Severity model

Incidents use SEV1–SEV4 aligned to response targets in `docs/sla.md`. Security-impacting events that compromise **evidence** or **tenant isolation** default to SEV1 or SEV2 until disproven.

## Policy statements

1. **Ownership:** Each incident has a designated incident commander coordinating technical and communications workstreams.
2. **Acknowledgment:** Initial acknowledgment meets `docs/sla.md` targets for the declared severity.
3. **Containment and evidence:** Preserve **capsule** lineage, **replay** artifacts, and logs before destructive remediation; enable **legal hold** when data preservation is required (see [Lifecycle controls](../lifecycle-controls.md)).
4. **Review:** SEV1/SEV2 require post-incident review within five business days with action items tracked to completion.
5. **Notifications:** Follow `docs/support-escalation-path.md` and customer contractual clauses for external communication.

## Related runbooks

| Topic | Runbook |
|-------|---------|
| **Replay** failure | `docs/runbooks/incident-replay-failure.md` |
| **Drift** anomaly | `docs/runbooks/incident-drift-anomaly.md` |
| **Evidence** corruption | `docs/runbooks/incident-evidence-corruption.md` |
| **Tenant isolation** attempt | `docs/runbooks/incident-tenant-isolation-breach-attempt.md` |
| **SIEM** / **OTel** exporter | `docs/runbooks/incident-siem-otel-exporter-failure.md` |

## Compliance references

- `docs/compliance/controls-matrix.md` (CC-12).
- `docs/compliance/iso27001-annex-a-mapping.md` (incident management).
- Status communications: `docs/status-page-template.md`.
