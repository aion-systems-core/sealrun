# Audit evidence template — Release attestation

Attach this evidence to release records when Cosign / Sigstore signing and SBOM generation are in scope for supply-chain controls.

## Record metadata

| Field | Value |
|-------|--------|
| Evidence ID | |
| Release version / tag | |
| Git commit SHA | |
| Build reference | CI job URL or build ID |
| Build operator or pipeline identity | |

## Artifacts

| Field | Value |
|-------|--------|
| Primary artifact | Binary or image digest |
| Signature object reference | `sealrun.sig` storage path |
| Public key reference | `cosign.pub` or KMS key ID |
| SBOM object reference | Storage path and digest |
| SBOM format | e.g. CycloneDX, SPDX |

## Cryptographic verification

| Field | Value |
|-------|--------|
| Cosign sign command transcript | Redact secrets |
| Cosign verify command transcript | Attach stdout showing success |
| Verification environment | OS image, cosign version |

## Review and approval

| Field | Value |
|-------|--------|
| Reviewer | |
| Approval date | |
| Link to change record | |

## Cross-references

- [Release attestation](../../release-attestation.md)
- [Change management policy](../../policies/change-management-policy.md)
- [Controls matrix](../compliance/controls-matrix.md) (CC-09)
