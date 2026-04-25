# SIEM and OpenTelemetry (OTel)

## Overview

SealRun enterprise can export governance and capsule-related events to enterprise security analytics pipelines: **SIEM** (Splunk HEC, Datadog Logs, Elastic ingest) and **OpenTelemetry** via OTLP HTTP. These exports support detective controls, **drift** and **replay** failure correlation, and **tenant isolation** monitoring without replacing your centralized observability stack.

## Architecture

- **SIEM sinks:** Structured log delivery to vendor-specific HTTP endpoints with authentication tokens managed by operators.
- **OTel export:** OTLP HTTP export path for traces/metrics/logs depending on collector configuration (see your collector docs).
- **Event model:** Governance, **policy evaluation**, and capsule lifecycle signals should carry stable identifiers (`tenant_id`, capsule references, **governance decision** IDs where applicable) for correlation with **evidence chain** artifacts.

## Example flows

1. **Onboarding:** Configure sink URLs and secrets in your secret manager; run `send-test` for each **SIEM** vendor in use.
2. **OTel validation:** Point `otel export` at a staging collector; verify attributes and resource identity before production.
3. **Incident:** On exporter failure, follow `docs/runbooks/incident-siem-otel-exporter-failure.md`; correlate with **replay**/**drift** runbooks when symptoms overlap.
4. **Audit:** Archive exporter configuration versions alongside **release attestation** and **SBOM** for the build in production (see [Release attestation](release-attestation.md)).

## Evidence capture points

- Successful `send-test` and `otel export` transcripts with timestamps (redact secrets).
- Collector-side delivery acknowledgements and SIEM search queries used in investigations.
- Mapping tables from SealRun event types to your SOC use cases.

## Policy enforcement points

- Network egress to **SIEM** / collectors is an organizational control; align with `docs/policies/vendor-third-party-risk-policy.md`.
- **RBAC** governs who may trigger test exports in production-like environments (see [RBAC](rbac.md)).
- **Policy engine** may require telemetry-related fields in **evidence** (e.g., trace IDs); see [Policy engine](policy-engine.md).

## Integration points

- **OIDC**-authenticated operators running export tests (see [OIDC auth](oidc-auth.md)).
- **Multi-tenancy:** Ensure sink field mapping preserves tenant boundaries in downstream indexes (see [Multi-tenancy](multi-tenancy.md)).
- **Trust Center** lists export as a primary evidence source: [Trust Center](trust-center.md).

## Compliance notes

- Map to CC-08 in `docs/compliance/controls-matrix.md` and communications security themes in `docs/compliance/iso27001-annex-a-mapping.md` (with [Policy engine](policy-engine.md) and [Telemetry](telemetry.md) for content classification).
- Retention of exported logs follows your platform retention policy, not SealRun alone.

## Next steps

- Define alert rules for export failure, volume drops, and authentication errors.
- Document vendor DPA and subprocessors under vendor risk policy.
- Status template component row for SIEM/OTel: `docs/status-page-template.md`.

## CLI reference

```bash
sealrun enterprise sinks send-test --sink splunk --endpoint <url> --token <token>
sealrun enterprise sinks send-test --sink datadog --endpoint <url> --token <token>
sealrun enterprise sinks send-test --sink elastic --endpoint <url> --token <token>
sealrun enterprise otel export --endpoint <otlp-http-endpoint>
```
