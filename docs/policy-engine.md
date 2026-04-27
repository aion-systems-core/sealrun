# Policy engine

## Overview

The enterprise **policy engine** validates and evaluates deterministic governance rules against run descriptors. It complements **RBAC** (who may act) by constraining **what** is allowed: models, seeds, external calls, and **evidence** field completeness. Outputs feed **governance decisions** and audit narratives and should align with **replay** and **drift** semantics in your organization.

## Architecture

- **Validation:** Schema and rule-shape checks before evaluation (`policy-api validate`).
- **Evaluation:** Deterministic pass/fail with structured violations (`policy-api evaluate`).
- **Bundles:** YAML bundles under `docs/enterprise/governance/bundles/` convert to JSON policy payloads for testing (see `docs/enterprise/governance/compliance-test-suite.md`).

## Policy schema (JSON)

```json
{
  "allowed_models": ["gpt-4o-mini"],
  "allowed_seeds": [1, 42],
  "allowed_external_calls": ["https://api.example.com"],
  "required_evidence_fields": ["trace_id", "policy_id"]
}
```

## Evaluation input (JSON)

```json
{
  "model": "gpt-4o-mini",
  "seed": 42,
  "external_calls": ["https://api.example.com"],
  "evidence_fields": {"trace_id": "t1", "policy_id": "p1"}
}
```

## Flows

1. **Author:** Edit policy JSON from an approved governance bundle baseline.
2. **Validate in CI:** Fail builds on invalid policies before deployment.
3. **Evaluate on candidate runs:** Attach evaluation JSON to change or incident tickets.
4. **Regulated fields:** Finance or healthcare bundles add fields such as `approver`, `reviewer`, or `data_classification`; ensure your run pipeline populates them before **policy evaluation**.

## Evidence capture points

- `validate` and `evaluate` command outputs (machine-readable).
- **Governance decision** records referencing policy version and bundle name.
- Templates: `docs/enterprise/templates/audit-evidence-governance-decision-template.md`.

## Policy enforcement points

- Admission at integration boundaries (adapters) before accepting capsule output; see adapter guides under `docs/integrations/`.
- CI gates and pre-promotion checks for configuration repos storing policy JSON.
- Optional linkage to **release attestation** identifiers in strict bundles.

## Integration points

- **Multi-tenancy** / **tenant isolation:** Include `tenant_id` in required evidence when auditing per-tenant posture.
- **SIEM** / **OTel:** Emit policy denial events to SOC pipelines (see [SIEM and OTel](siem-otel.md)).
- **OIDC** and **RBAC:** Operator identity for policy changes is governed separately (see [OIDC auth](oidc-auth.md), [RBAC](rbac.md)).

## Compliance notes

- Map to CC-10 in `docs/enterprise/compliance/controls-matrix.md` and technological controls in `docs/enterprise/compliance/iso27001-annex-a-mapping.md`.
- Exceptions to policy use `docs/policies/exceptions-policy.md` with compensating controls.

## Next steps

- Run the compliance test suite flow: `docs/enterprise/governance/compliance-test-suite.md`.
- Standardize adapter patterns: Hugging Face, LangChain, Modal, BentoML guides in `docs/integrations/`.
- Hub: [Trust Center](trust-center.md).

## CLI reference

```bash
sealrun enterprise policy-api validate --policy policy.json
sealrun enterprise policy-api evaluate --policy policy.json --input input.json
```
