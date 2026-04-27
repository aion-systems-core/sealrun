# Release attestation

## Overview

Release **attestation** ties shipped binaries to cryptographic signatures and software transparency artifacts. SealRun integrates **Cosign** (Sigstore) for sign/verify workflows and **SBOM** generation via `cargo sbom` (`cargo-sbom`). Together they support supply-chain objectives referenced in procurement questionnaires and align with **evidence chain** expectations for governed releases.

## Architecture

- **Sign:** Produce signatures for release artifacts using **Cosign** in PATH.
- **Verify:** Validate artifact integrity and signature trust before promotion or deployment.
- **SBOM:** Emit machine-readable dependency inventory for vulnerability management and license review.
- **Governance link:** Strict governance bundles may require `release_attestation_id` in **required_evidence_fields** (see `docs/enterprise/governance/bundles/strict.yaml` and [Policy engine](policy-engine.md)).

## Flows

1. **Build:** CI produces the release binary and **SBOM**; store both as immutable objects.
2. **Sign:** Run `release-attestation sign` on the artifact; publish signature and public key material per your key management policy.
3. **Verify:** Promotion pipelines run `verify` and fail closed on mismatch.
4. **Governed workloads:** Attach attestation and **SBOM** references to **governance decision** records and audit templates (`docs/enterprise/templates/audit-evidence-release-attestation-template.md`).

## Evidence capture points

- Cosign sign/verify stdout/stderr (non-secret) and artifact digests.
- **SBOM** file checksum and storage location.
- Change ticket linking version, commit SHA, and attestation objects.

## Policy enforcement points

- **Policy engine:** Optional required field `release_attestation_id` for high-assurance bundles.
- **RBAC:** Restrict who may run signing operations on release hosts (organizational; see [RBAC](rbac.md)).
- **Change management:** `docs/policies/change-management-policy.md` ties releases to CI and attestation outputs.

## Integration points

- **SIEM** / **OTel:** Log verification failures and signing operations where tooling permits (see [SIEM and OTel](siem-otel.md)).
- **Compatibility matrix:** Version anchors for CLI and policy artifacts: [Compatibility matrix](compatibility-matrix.md).
- **Security guide:** Trust chain and distribution posture: [Security guide](security-guide.md).

## Compliance notes

- Map to CC-09 in `docs/enterprise/compliance/controls-matrix.md` and system acquisition controls in `docs/enterprise/compliance/iso27001-annex-a-mapping.md`.
- **Sigstore** / **Cosign** trust roots and key rotation are operator responsibilities; document in `docs/security-whitepaper.md`.

## Next steps

- Add attestation verification to deployment playbooks: [Operations guide](operations-guide.md).
- Archive per-release evidence using `docs/enterprise/templates/audit-evidence-release-attestation-template.md`.
- Hub: [Trust Center](trust-center.md).

## CLI reference

```bash
sealrun enterprise release-attestation sign --artifact target/release/sealrun
sealrun enterprise release-attestation verify \
  --artifact target/release/sealrun \
  --signature sealrun.sig \
  --public-key cosign.pub
sealrun enterprise release-attestation sbom
```

## Notes

- `sign` and `verify` require `cosign` on `PATH`.
- `sbom` requires `cargo-sbom` providing the `cargo sbom` command.
