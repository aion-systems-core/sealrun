# Procurement mini-pack (pilot)

## Purpose

One-page **starter** for vendor security questionnaires: where to find summaries, what is certified vs design-only, and **known open items**. Not legal advice; Legal owns customer-facing terms.

For the full documentation map (including enterprise hubs outside this pilot folder), see the [Documentation index](../README.md).

## Trust Center summary

SealRun publishes a capability map covering **tenant isolation**, **RBAC**, **OIDC**, deterministic **replay** and **drift**, **evidence chain**, **SIEM** / **OTel**, and **Cosign** / **Sigstore** **attestation** with **SBOM**. Entry point: [Trust Center](../../trust-center.md).

## DPA / subprocessor status

| Topic | Status (fill for your org) | Owner |
|-------|----------------------------|--------|
| Data Processing Agreement (DPA) | `<signed / in review / N/A self-hosted>` | Legal |
| Subprocessor list | `<URL or attach>` | Legal / Vendor mgmt |
| Data residency | `<regions>` | Legal / Infra |

## Shared responsibility model

- **Customer:** host isolation, network controls, IdP configuration, SIEM retention, endpoint security, backups of SealRun data stores.
- **SealRun (software):** deterministic execution contracts, enterprise CLI semantics, policy evaluation hooks as documented.

Full narrative: [Security guide](../../security-guide.md), [Trust Center](../../trust-center.md).

## SLA summary

Example response targets (contractual SLAs are customer-specific): [SLA](../../sla.md). Attach executed MSA or support addendum if applicable.

## Security whitepaper summary

High-level threats, evidence model, identity, observability, supply chain: [Security whitepaper](../../security-whitepaper.md).

## Design-reference compliance artifacts (non-certificates)

| Artifact | Use |
|----------|-----|
| [Controls matrix](../compliance/controls-matrix.md) | SOC 2–style **design** mapping (not a SOC 2 report) |
| [ISO Annex A mapping](../compliance/iso27001-annex-a-mapping.md) | ISO 27001 **design** mapping (not a certificate) |
| [Buyer guide](../../buyer-guide.md) | Evaluation flow |

## Known open items (maintenance table)

| ID | Item | Impact | Owner | Target |
|----|------|--------|-------|--------|
| O1 | Formal SOC 2 Type II | Procurement for regulated buyers | | |
| O2 | Penetration test report | Security questionnaire | | |
| O3 | Production SIEM field mapping signed off | SOC correlation | | |
| O4 | Executed DPA | Legal gate | | |
| O5 | Managed offering / HA SLOs | Buyers needing SaaS | | |

## Pilot Readiness Pack index

| Document | Use |
|----------|-----|
| [Success criteria](success-criteria.md) | Exit metrics |
| [Scope definition](scope-definition.md) | Boundaries |
| [Break-glass and ownership](break-glass-and-ownership.md) | Privileged ops |
| [Secrets handling](secrets-handling.md) | Token hygiene |
| [Backup and restore](backup-and-restore.md) | Resilience |
| [Bill of materials](bill-of-materials.md) | Version snapshot |
| [Onboarding script](onboarding-script.md) | Day-one commands |
| [Monitoring minimum](monitoring-minimum.md) | Alerts and fields |
| [Data classification](data-classification.md) | PII rules |
| [Feedback loop](feedback-loop.md) | Weekly sync |
| [Demo golden path](demo-golden-path.md) | Executive demo |

## Related documents

- [Support escalation path](../../support-escalation-path.md) · [Status page template](../../status-page-template.md) · [Vendor risk policy](../../policies/vendor-third-party-risk-policy.md)
