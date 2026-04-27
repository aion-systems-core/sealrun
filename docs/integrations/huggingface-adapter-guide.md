# Hugging Face adapter guide

## Overview

This guide describes how to wrap **Hugging Face** model inference behind SealRun’s deterministic execution and **evidence** model. The adapter boundary is where **capsule** artifacts are produced, **replay** symmetry is preserved, and **drift** can be measured against baselines. **Tenant isolation** and **RBAC** apply when results are registered in enterprise storage.

## Architecture

- **Deterministic envelope:** Model invocation parameters (weights revision, tokenizer revision, seed, dtype policy) are captured so **replay** can reproduce or explain divergence.
- **Tenant context:** Post-run registration associates **capsule** and **evidence chain** entries with a tenant partition (see [Multi-tenancy](../multi-tenancy.md)).
- **Governance layer:** **Policy evaluation** gates acceptance using allowed models, seeds, external hosts, and **required_evidence_fields** (see [Policy engine](../policy-engine.md)).

## Flows

1. **Offline batch:** Fixed dataset shard, pinned model revision, recorded seed; emit capsule + sidecar evidence.
2. **Online service:** Request-scoped run with allow-listed external artifact fetch; deny if host not in **policy engine** `allowed_external_calls`.
3. **Promotion:** Compare candidate capsule to golden baseline; classify **drift**; record **governance decision** before routing traffic.
4. **Incident:** On replay failure, attach adapter config snapshot and policy JSON to the ticket (see `docs/enterprise/runbooks/incident-replay-failure.md`).

## Evidence capture points

- Model card revision, tokenizer hash, framework versions.
- Input normalization metadata (truncation, padding strategy).
- **Capsule** path, **replay** outputs, **policy evaluation** JSON, optional **release_attestation_id** / **SBOM** pointers for strict bundles.

## Policy enforcement points

- `allowed_models` aligned to approved Hugging Face model IDs or internal mirrors.
- `allowed_seeds` for environments where seed control is mandated.
- `allowed_external_calls` for Hub download endpoints or mirrors only.
- `required_evidence_fields` minimally `trace_id`, `policy_id`, `tenant_id` per default bundle.

## Integration points

- **OIDC** for operators or service principals invoking registration CLI (see [OIDC auth](../oidc-auth.md)).
- **SIEM** / **OTel:** Export governance and denial events (see [SIEM and OTel](../siem-otel.md)).
- **Cosign** / **Sigstore:** Sign container images that embed the adapter runtime; link to [Release attestation](../release-attestation.md).

## Compliance notes

- Treat model weights and prompts per your data classification policy; map fields like `data_classification` for healthcare-style bundles (`docs/enterprise/governance/bundles/regulated-healthcare.yaml`).
- Controls mapping: `docs/enterprise/compliance/controls-matrix.md`, Annex A: `docs/enterprise/compliance/iso27001-annex-a-mapping.md`.

## Next steps

- Prototype evaluate/validate in CI using `docs/enterprise/governance/compliance-test-suite.md`.
- Add audit rows using `docs/enterprise/templates/audit-evidence-replay-template.md` and `docs/enterprise/templates/audit-evidence-governance-decision-template.md`.
- Read [Security guide](../security-guide.md) for Execution OS vs host isolation boundaries.
