# Trust Center

## Overview

This page is the **baseline trust narrative** for SealRun enterprise programs. It links deterministic execution (**capsule**, **replay**, **drift**), the **evidence chain**, **governance decisions**, **policy evaluation**, **tenant isolation**, **RBAC**, **OIDC**, observability (**SIEM** / **OTel**), and supply-chain integrity (**Cosign** / **Sigstore**, **SBOM**). It does not claim third-party certifications; customers map these artifacts to their own control frameworks.

Full navigation for all Markdown hubs: [Documentation index](README.md).

## Architecture (control framing)

| Control family | SealRun alignment |
|----------------|-------------------|
| Access | **OIDC** authentication, **RBAC** authorization, least privilege |
| Confidentiality | **Tenant isolation** in enterprise storage indexes |
| Integrity | Deterministic **replay**, structured **drift**, signed releases |
| Auditability | **Evidence chain** records, policy outputs, CLI JSON envelopes |
| Monitoring | **SIEM** and **OTel** export for governance and failure signals |
| Supply chain | **Attestation** and **SBOM** workflows per [Release attestation](release-attestation.md) |

## Evidence sources (CLI and documents)

Representative commands and outputs for audits and operational reviews:

- `sealrun enterprise auth status` — **OIDC** session posture
- `sealrun enterprise audit-events` — governance-oriented event stream (where enabled)
- `sealrun enterprise trust-center` — packaged trust summary (where available in product)
- `sealrun enterprise release-attestation sign|verify|sbom` — **Cosign** / **Sigstore** and **SBOM** evidence
- `sealrun enterprise tenants list` — **tenancy** inventory
- `sealrun enterprise lifecycle retention get --tenant <id>` — retention posture
- `sealrun enterprise rbac export` — **RBAC** matrix
- `sealrun enterprise policy-api validate --policy <path>` — **policy evaluation** readiness
- `sealrun governance status` — governance contract snapshot (open-core surface)
- `sealrun doctor` — environment readiness

## Enterprise capability map

- **Multi-tenancy:** `enterprise tenants *` with tenant-scoped **capsule** and **evidence** indexes ([Multi-tenancy](multi-tenancy.md))
- **Lifecycle controls:** Retention, purge, legal hold ([Lifecycle controls](lifecycle-controls.md))
- **RBAC:** YAML-backed assignments and checks ([RBAC](rbac.md))
- **OIDC:** Device-code login, status, logout ([OIDC auth](oidc-auth.md))
- **SIEM** and **OTel:** Sink tests and OTLP HTTP export ([SIEM and OTel](siem-otel.md))
- **Release attestations:** **Cosign** / **Sigstore** plus **SBOM** ([Release attestation](release-attestation.md))
- **Policy engine:** Deterministic validation and evaluation ([Policy engine](policy-engine.md))

## Policies, runbooks, and operations

- Policies: `docs/policies/`
- Runbooks: `docs/enterprise/runbooks/` (replay, drift, evidence, tenant isolation, SIEM / OTel)
- SLA and escalation: [SLA](sla.md), [Support escalation path](support-escalation-path.md)
- Status communications: [Status page template](status-page-template.md)
- Day-two operations: [Operations guide](operations-guide.md)

## Compliance and procurement references

- Controls matrix (design reference): `docs/enterprise/compliance/controls-matrix.md`
- ISO 27001 Annex A mapping (design reference): `docs/enterprise/compliance/iso27001-annex-a-mapping.md`
- Security narrative: [Security whitepaper](security-whitepaper.md)
- Buyer evaluation: [Buyer guide](buyer-guide.md)
- Public roadmap: [Roadmap](roadmap-public.md)

## Ecosystem integration scaffolds

Adapter guides share a common evaluation pattern (architecture, flows, evidence, policy, integration, compliance):

- [Hugging Face](integrations/huggingface-adapter-guide.md)
- [LangChain](integrations/langchain-adapter-guide.md)
- [Modal](integrations/modal-adapter-guide.md)
- [BentoML](integrations/bentoml-adapter-guide.md)

Version anchors: [Compatibility matrix](compatibility-matrix.md).

## Shared responsibility

SealRun defines deterministic contracts and enterprise storage semantics; it is not a host security boundary by itself. Filesystem, network, and runtime isolation are enforced by the customer platform or additional modules. See [Security guide](security-guide.md).

## Next steps

- Run the [Governance compliance test suite](enterprise/governance/compliance-test-suite.md) against your chosen bundle.
- Archive evidence using templates in `docs/enterprise/templates/`.
- Schedule access reviews using **RBAC** exports and **OIDC** IdP logs jointly.
