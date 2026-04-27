# Change management policy

## Purpose

Ensure product and configuration changes are planned, reviewed, tested, and traceable, including changes that affect **replay**, **drift**, **policy evaluation**, **tenant isolation**, and release **attestation** artifacts.

## Scope

Covers application releases, enterprise configuration (including **RBAC** and lifecycle policy), integration adapters, and emergency changes.

## Policy statements

1. **Traceability:** Changes link to a tracked issue or change record with approver identity.
2. **Review:** Production-impacting changes require peer review per team norms.
3. **Verification:** Releases require passing CI, including `cargo test --workspace --all-targets`, and deterministic contract checks where applicable.
4. **Release artifacts:** Each release record includes changelog notes, **Cosign** / **Sigstore** **attestation** outputs where signing is in scope, **SBOM** references, and rollback guidance (see [Release attestation](../release-attestation.md)).
5. **Emergency change:** Must reference an active incident; complete retrospective within five business days per severity table in `docs/sla.md`.

## Roles and responsibilities

| Role | Responsibility |
|------|----------------|
| Engineering | Implements change, supplies test evidence, updates changelog. |
| Release engineering | Owns signing, **SBOM**, and artifact promotion mechanics. |
| SRE | Validates operational impact, runbook updates, and communication templates. |

## Evidence requirements

- CI logs and test summaries.
- Changelog entry using `docs/enterprise/templates/changelog-template.md` where applicable.
- `release-attestation` sign/verify transcripts and **SBOM** storage references (non-secret).

## Compliance references

- `docs/enterprise/compliance/controls-matrix.md` (CC-03, CC-09).
- `docs/enterprise/compliance/iso27001-annex-a-mapping.md` (system acquisition / development).
- Related: `docs/operations-guide.md`, [Trust Center](../trust-center.md).
