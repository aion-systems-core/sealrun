# Governance compliance test suite

## Overview

Define **repeatable** checks for governance bundle validity and **policy evaluation** behavior. This suite complements **RBAC** and **OIDC** controls: it validates the JSON policy consumed by `sealrun enterprise policy-api`, typically derived from the YAML bundles in `docs/enterprise/governance/bundles/`.

## Bundle inventory (reference)

| Bundle file | Intent | Notable `required_evidence_fields` |
|-------------|--------|-------------------------------------|
| `bundles/default.yaml` | Baseline development and broad testing | `trace_id`, `policy_id`, `tenant_id` |
| `bundles/strict.yaml` | Reduced model and seed surface; ties releases to attestation | adds `release_attestation_id` |
| `bundles/regulated-finance.yaml` | Finance-style approvals | adds `decision_timestamp`, `approver` |
| `bundles/regulated-healthcare.yaml` | Healthcare-style classification and review | adds `data_classification`, `reviewer` |

Bundles are **documentation baselines**. Convert selected bundle YAML to the JSON schema documented in [Policy engine](../../policy-engine.md) before calling the CLI.

## Test categories

1. **Schema validity:** Required control keys exist (`allowed_models`, `allowed_seeds`, `allowed_external_calls`, `required_evidence_fields`).
2. **Rule validity:** Lists parse as non-empty where your program requires constraints; URLs are well-formed for external call rules.
3. **Positive evaluation:** Compliant `input.json` returns pass with no violations.
4. **Negative evaluation:** Disallowed model, seed, or external call returns structured violations.
5. **Evidence enforcement:** Missing **required_evidence_fields** returns violations (supports **evidence chain** completeness).

## Suggested command flow

1. Translate bundle YAML to `policy.json` aligned with [Policy engine](../../policy-engine.md).
2. Run `sealrun enterprise policy-api validate --policy policy.json`.
3. Run `sealrun enterprise policy-api evaluate --policy policy.json --input input.json` for positive and negative fixtures.
4. Archive stdout and policy artifacts using `docs/enterprise/templates/audit-evidence-governance-decision-template.md`.

## CI integration notes

- Store golden `input.json` fixtures next to infrastructure-as-code for adapters (Hugging Face, LangChain, Modal, BentoML).
- Fail pipelines on validation errors before deployment to governed environments.

## Related documents

- [Policy engine](../../policy-engine.md)
- [Buyer guide](../../buyer-guide.md) evaluation steps
- [Trust Center](../../trust-center.md)
