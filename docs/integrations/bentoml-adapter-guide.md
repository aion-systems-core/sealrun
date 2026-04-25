# BentoML adapter guide

## Overview

**BentoML** services expose HTTP inference endpoints that can be instrumented to emit SealRun **capsule** and **evidence** artifacts for each deterministic **replay**-eligible invocation. This guide aligns BentoML integration with the same **policy evaluation**, **tenant isolation**, and observability patterns as other adapters.

## Architecture

- **Service boundary:** Middleware or sidecar captures request metadata, model name, revision, and response serialization rules.
- **RBAC-aware registration:** Only entitled service accounts register capsules under the correct tenant (see [RBAC](../rbac.md), [Multi-tenancy](../multi-tenancy.md)).
- **Policy gate:** Evaluate **policy engine** rules on model, seed, declared external calls, and evidence fields before marking a run accepted.

## Example flows

1. **Sync inference:** Single request/response path with stable serializer; emit capsule per request or batched window per your volume policy.
2. **Canary:** Route fraction of traffic to candidate bento; compare **drift** against baseline capsule set.
3. **Forensics:** On **evidence** anomaly, quarantine artifacts per `docs/runbooks/incident-evidence-corruption.md` and preserve legal hold if needed (see [Lifecycle controls](../lifecycle-controls.md)).

## Evidence capture points

- Bento tag, manifest digest, dependency lockfile hash.
- Request/response metadata (payload bodies per privacy policy).
- **Capsule**, **replay** outputs, **governance decision** JSON.

## Policy enforcement points

- `allowed_models` matching Bento manifest model entries.
- `allowed_external_calls` for any outbound hooks from the service.
- `required_evidence_fields` including correlation and `tenant_id`.

## Integration points

- **SIEM** / **OTel:** Ship policy denial and error paths from the service layer (see [SIEM and OTel](../siem-otel.md)).
- **Release attestation:** Sign Bento images or release bundles with **Cosign**; archive **SBOM** (see [Release attestation](../release-attestation.md)).
- **OIDC** for admin APIs that change Bento deployments (see [OIDC auth](../oidc-auth.md)).

## Compliance notes

- Endpoint exposure falls under access control and network zoning: `docs/policies/access-control-policy.md`, [Security guide](../security-guide.md).
- Map monitoring controls to CC-08 in `docs/compliance/controls-matrix.md`.

## Next steps

- Add contract tests that fail closed when policy JSON drifts from approved bundles in `docs/governance/bundles/`.
- Use `docs/templates/audit-evidence-governance-decision-template.md` for promotion reviews.
- [Compatibility matrix](../compatibility-matrix.md) lists integration guide anchors for integrators.
