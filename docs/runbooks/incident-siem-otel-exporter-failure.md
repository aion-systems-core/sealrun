# Runbook: Incident — SIEM / OTel exporter failure

## Overview

Loss of telemetry from SealRun enterprise exports weakens detective controls for policy evaluation denials, authentication failures, and operational errors. This runbook restores SIEM and OpenTelemetry (OTel) export paths.

## Trigger

- `send-test` or `otel export` commands fail repeatedly.
- Monitoring shows drop in event volume from SealRun sources.
- Collector or vendor outage correlated in status dashboards.

## Detection

- Synthetic checks hitting Splunk HEC, Datadog, or Elastic endpoints.
- Collector health metrics and error logs for OTLP HTTP ingest.

## Impact

- Delayed security investigations; weaker assurance during concurrent incidents (replay, drift, evidence).

## Mitigation

1. Run sink and OTel test commands; capture stdout/stderr (redact tokens) ([SIEM and OTel](../siem-otel.md)).
2. Verify DNS, TLS, firewall egress, token expiration, and payload schema accepted by the collector.
3. Fail over to alternate region sink or backup collector if architecture supports it.
4. Buffer or backfill: export deterministic artifacts from enterprise storage where product semantics support reconstruction; document gaps candidly for auditors.
5. Open vendor or internal platform incident if external outage is confirmed.

## Verification

- `send-test` succeeds for each configured SIEM sink; OTel spans or logs appear in staging before production re-enable.
- Alert rules return to green; volume within expected band for workload.

## Escalation

- Network / cloud platform for egress or certificate issues.
- Vendor TAM for SIEM SaaS incidents per `docs/policies/vendor-third-party-risk-policy.md`.

## Post-incident

- Document delivery gap window and impact assessment.
- Create action items for token rotation automation, dual endpoints, or retry policies.
- Update `docs/status-page-template.md` entry if customer-visible observability commitments exist.
