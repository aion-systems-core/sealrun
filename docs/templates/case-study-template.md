# Case study template

Customer-facing case studies should reinforce deterministic execution, evidence chain quality, and enterprise controls (tenant isolation, RBAC, OIDC, SIEM / OTel, attestation, SBOM).

## Customer profile

| Field | Value |
|-------|--------|
| Industry | |
| Region | |
| Team size | |
| Compliance regimes | e.g. SOC 2 readiness, ISO 27001 mapping (customer-owned certification) |

## Challenge

- Baseline pain points: reliability, audit burden, non-deterministic AI outputs.
- Risk and compliance constraints: data residency, segregation of duties, vendor risk.

## SealRun deployment scope

- Workloads covered: batch, online inference, orchestration adapters (Hugging Face, LangChain, Modal, BentoML, and so on).
- Tenant and governance model: bundles used (`default`, `strict`, regulated variants).
- Integration landscape: IdP, SIEM, OTel collectors, artifact storage.

## Technical outcomes

- Replay and drift: measurable reduction in unexplained variance or time to diagnose divergence.
- Evidence chain: how audits or internal reviews consume capsule-linked artifacts.
- Policy evaluation: examples of denied runs caught before production impact.

## Operational outcomes

- MTTR or incident volume changes tied to runbooks and telemetry.
- Release confidence from Cosign verify and SBOM practices.
- Access review efficiency using RBAC exports and OIDC integration.

## Trust signals cited

- References to `docs/trust-center.md`, `docs/security-whitepaper.md`, and relevant policies (no unsubstantiated certification claims).

## Lessons learned

- Technical: what had to be pinned for determinism.
- Process: how change management and exceptions were governed.

## Publication approvals

- Named spokesperson approval: yes / no.
- Logo use approval: yes / no.
