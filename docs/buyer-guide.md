# Buyer evaluation guide

## Overview

This guide helps security, platform, and procurement teams evaluate SealRun for deterministic **capsule** execution, **replay** and **drift** assurance, **evidence chain** auditability, and enterprise controls: **tenant isolation**, **RBAC**, **OIDC**, **SIEM** / **OTel**, **Cosign** / **Sigstore** **attestation**, and **SBOM** practices.

## Evaluation goals

- Verify deterministic **replay** quality on representative workloads.
- Validate **policy evaluation** and governance bundle alignment to organizational posture.
- Assess **tenant isolation** and access control models.
- Confirm operational readiness using SLAs, escalation paths, and runbooks.

## Recommended evaluation flow

1. Execute deterministic runs and **replay** on gold and edge-case **capsules**; archive outputs with `docs/templates/audit-evidence-replay-template.md`.
2. Exercise **drift** detection between approved baselines and candidate builds; document disposition with `docs/templates/audit-evidence-drift-template.md`.
3. Attempt negative tests for cross-tenant access; expect denials and clear logging suitable for **SIEM** correlation.
4. Review **OIDC** login, session status, and logout; review **RBAC** assignments and exports for least privilege.
5. Validate lifecycle controls including retention, purge, and legal hold semantics per `docs/lifecycle-controls.md`.
6. Run **SIEM** sink `send-test` and **OTel** export against non-production collectors; confirm field mapping includes `tenant_id` where required.
7. Run **release attestation** sign and verify on a sample artifact; archive **SBOM** references per `docs/release-attestation.md`.

## Decision checklist

| Criterion | Evidence to collect |
|-----------|---------------------|
| Determinism and **evidence** controls meet requirements | Replay and drift artifacts; policy evaluation JSON |
| Governance bundles match risk posture | Mapped bundle YAML, compliance test suite outputs |
| Incident and SLA model fits operations | Walkthrough of `docs/sla.md`, escalation path, runbooks |
| Procurement artifacts are complete | Trust center, whitepaper, controls matrix (design reference), ISO mapping (design reference) |

## Related documents

- [Trust Center](trust-center.md)
- [Security whitepaper](security-whitepaper.md)
- [Governance compliance test suite](governance/compliance-test-suite.md)
- Integration scaffolds: `docs/integrations/*.md` (also indexed in [Compatibility matrix](compatibility-matrix.md))
