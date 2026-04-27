# SealRun security whitepaper

## Overview

SealRun provides deterministic execution and audit-grade **evidence** for AI and automation workloads. This whitepaper summarizes how **capsules**, **replay**, **drift**, **governance decisions**, and **policy evaluation** interlock with enterprise controls: **tenant isolation**, **RBAC**, **OIDC**, **SIEM** / **OpenTelemetry (OTel)**, and supply-chain practices using **Cosign** / **Sigstore** and **SBOM** artifacts.

SealRun is an execution contract layer. Host-level isolation remains the responsibility of the deploying organization unless augmented by separate runtime modules; see [Security guide](security-guide.md).

## Deterministic execution model

- Workloads emit structured deterministic envelopes suitable for machine verification.
- **Replay** verifies behavioral symmetry against preserved **capsule** state.
- **Drift** surfaces meaningful deltas when inputs or dependencies change, supporting measured **governance decisions** rather than ad hoc judgment calls.

## Evidence chain

- **Capsule**-linked **evidence** records connect runs to **replay**, **drift**, and policy outcomes.
- **Governance decisions** should be stored as structured artifacts alongside human-readable narratives.
- Lifecycle events (retention, purge, legal hold) affect availability of **evidence** and must be governed through operations policy.

## Tenant isolation

- Enterprise storage partitions **capsule** and **evidence** indexes per tenant.
- CLI surfaces require tenant context for sensitive reads and writes.
- **Tenant isolation** incidents are handled via dedicated runbooks and escalation paths.

## RBAC model

- Roles: `admin`, `auditor`, `operator`, `viewer`.
- Permissions include **replay**, diff (drift-related operations), purge, retention configuration, legal hold, and tenant administration, as documented in [RBAC](rbac.md).
- Assignments live in reviewable YAML artifacts suitable for access reviews.

## OIDC authentication

- Device-code **OIDC** flow supports CLI-native enterprise authentication.
- Login, status, and logout are explicit operator actions with local token persistence; protect endpoints accordingly.

## Observability and detective controls

- **SIEM** integrations (Splunk HEC, Datadog Logs, Elastic ingest) and **OTel** export support centralized monitoring of governance-relevant signals.
- Map exported fields to SOC-style monitoring use cases described in `docs/enterprise/compliance/controls-matrix.md`.

## Supply-chain security

- **Release attestation** integrates **Cosign** sign and verify flows backed by **Sigstore** trust roots where configured.
- **SBOM** generation integrates `cargo sbom` for dependency transparency.
- Archive attestation and **SBOM** evidence with each release record; strict governance bundles may require `release_attestation_id` in **policy evaluation** inputs.

## Compliance alignment (non-certified)

- Design references: `docs/enterprise/compliance/controls-matrix.md`, `docs/enterprise/compliance/iso27001-annex-a-mapping.md`.
- Policies: `docs/policies/` for access, change, incident, vendor, exceptions, and risk management.
- This whitepaper does not replace a customer-controlled system description or formal certification package.

## Next steps for security reviewers

- Read [Trust Center](trust-center.md) for the consolidated capability map.
- Follow [Buyer guide](buyer-guide.md) for hands-on evaluation sequences.
- Review [Operations guide](operations-guide.md) and `docs/enterprise/runbooks/` for operational maturity evidence.
