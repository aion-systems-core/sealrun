# Modal adapter guide

## Overview

**Modal** runs serverless or scheduled GPU/CPU work that must still produce SealRun-grade **capsule** and **evidence** artifacts. This guide standardizes how Modal jobs participate in **replay**, **drift**, and **governance decision** workflows while preserving **tenant isolation** and enterprise observability.

## Architecture

- **Deterministic job profile:** Image digest, function entrypoint, region, and pinned dependency layers feed the capsule contract.
- **Tenant propagation:** Job identity carries tenant context into registration and evidence indexes (see [Multi-tenancy](../multi-tenancy.md)).
- **Attestation linkage:** For strict programs, attach **Cosign** signatures for images and **SBOM** references where Modal build pipelines emit them (see [Release attestation](../release-attestation.md)).

## Flows

1. **Batch inference:** Submit with fixed seed and model allow-list; collect capsule bundle at job completion.
2. **Scheduled replay check:** Nightly job replays prior capsules; **drift** reports feed SOC dashboards via **SIEM** / **OTel** (see [SIEM and OTel](../siem-otel.md)).
3. **Promotion:** Candidate image digest must pass `policy-api evaluate` including `allowed_external_calls` for any egress.
4. **Failure:** Modal platform outage correlated using `docs/runbooks/incident-replay-failure.md` and exporter runbook if telemetry breaks.

## Evidence capture points

- Modal run ID, image digest, seed, and serialized inputs (subject to privacy policy).
- **Capsule** / **replay** artifacts and **policy evaluation** JSON.
- Optional `release_attestation_id` when strict governance bundle is active.

## Policy enforcement points

- `allowed_seeds` and `allowed_models` for deterministic acceptance.
- `allowed_external_calls` for any network egress from the Modal sandbox.
- `required_evidence_fields` including `tenant_id` and policy correlation fields.

## Integration points

- **RBAC** for who may trigger production Modal deploy hooks (organizational; see [RBAC](../rbac.md)).
- **OIDC** for CLI or CI tokens that register results into enterprise storage (see [OIDC auth](../oidc-auth.md)).
- **Change management:** Modal image changes follow `docs/policies/change-management-policy.md`.

## Compliance notes

- Modal is a subprocessor under your vendor risk process: `docs/policies/vendor-third-party-risk-policy.md`.
- ISO mapping for supplier relationships: `docs/compliance/iso27001-annex-a-mapping.md`.

## Next steps

- Document Modal secrets handling separately; never embed tokens in **capsule** payloads.
- Automate policy validation in CI per `docs/governance/compliance-test-suite.md`.
- Status communications: `docs/status-page-template.md` when Modal-dependent components degrade.
