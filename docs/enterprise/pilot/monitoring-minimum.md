# Monitoring minimum (pilot)

## Purpose

Define the **smallest** set of alerts and field mappings so replay failures, telemetry loss, auth issues, and capacity problems cannot go unnoticed during the pilot. Align with [success criteria](success-criteria.md) and [SIEM and OTel](../../siem-otel.md).

## Alert catalog

| Alert ID | Condition | Severity | Runbook |
|----------|-----------|----------|---------|
| A1 | Reference **replay** job failed | High | [Replay failure](../runbooks/incident-replay-failure.md) |
| A2 | SIEM `send-test` or exporter health check failed | High | [SIEM / OTel exporter](../runbooks/incident-siem-otel-exporter-failure.md) |
| A3 | Spike in **OIDC** / enterprise auth failures | Medium | [Access control policy](../../policies/access-control-policy.md), IdP dashboards |
| A4 | Disk or quota threshold exceeded on enterprise store volume | Medium | Capacity runbook (internal) |
| A5 | Policy evaluation denial rate anomaly | Low–Medium | [Policy engine](../../policy-engine.md), drift runbook |

## Example Prometheus-style rules (pseudo-YAML)

These are **illustrative**; adapt metric names to your exporters and golden-path jobs.

```yaml
groups:
  - name: sealrun_pilot
    rules:
      - alert: SealRunReferenceReplayFailed
        expr: sealrun_reference_replay_success == 0
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Reference replay failed for pilot golden capsule"

      - alert: SealRunSiemExporterUnhealthy
        expr: sealrun_siem_send_test_success == 0
        for: 15m
        labels:
          severity: critical
        annotations:
          summary: "SIEM send-test unhealthy for 15m"

      - alert: SealRunAuthFailureSpike
        expr: rate(sealrun_enterprise_auth_failures_total[10m]) > 0.5
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Elevated enterprise auth failures"

      - alert: SealRunDiskSpaceLow
        expr: (node_filesystem_avail_bytes{mountpoint="/var/sealrun"} / node_filesystem_size_bytes) < 0.15
        for: 30m
        labels:
          severity: warning
        annotations:
          summary: "SealRun data volume below 15% free"
```

Instrument `sealrun_*` metrics via your wrapper jobs or log-derived metrics if Prometheus is not adjacent to SealRun.

## SIEM / OTel field mapping table

| Canonical field | SIEM field (example) | OTel attribute (example) | Notes |
|-----------------|----------------------|---------------------------|--------|
| `tenant_id` | `tenant_id` | `sealrun.tenant.id` | Required for isolation audits |
| `capsule_id` or path | `capsule_ref` | `sealrun.capsule.ref` | Avoid full paths if sensitive |
| `policy_id` | `policy_id` | `sealrun.policy.id` | Ties to [policy evaluation](../../policy-engine.md) |
| `trace_id` | `trace_id` | `trace.trace_id` | Correlate across systems |
| `event_type` | `event_type` | `sealrun.event.type` | e.g. replay_failure, sink_test |
| `severity` | `severity` | `sealrun.severity` | Map to SOC severity model |

Normalize enums so dashboards remain comparable week to week.

## Related documents

- [Onboarding script](onboarding-script.md) · [Demo golden path](demo-golden-path.md) · [Feedback loop](feedback-loop.md)
