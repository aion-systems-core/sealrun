# Secrets handling (pilot)

## Purpose

Standardize how **OIDC**, **SIEM**, signing, and integration secrets are stored, rotated, and **never** leaked into tickets or chat during the pilot. Complements [break-glass and ownership](break-glass-and-ownership.md).

## Secret inventory (fill and store outside this repo)

| Secret type | Example | Primary store | Owner |
|-------------|---------|---------------|--------|
| OIDC client secret | OAuth confidential client | `<Vault / Entra / Okta>` | `<team>` |
| OIDC refresh artifacts | Device flow tokens on disk | OS user profile; disk encryption required | User + MDM |
| SIEM HEC / API tokens | Splunk, Datadog, Elastic | `<Vault>` | `<team>` |
| OTel exporter headers | OTLP auth | `<Vault>` | `<team>` |
| Cosign / signing keys | Private key for release signing | `<KMS / HSM>` | Release engineering |
| Infrastructure SSH / kubeconfig | If used for hosts | `<Vault>` | Platform |

## OIDC client

- Register a **dedicated** client for the pilot; do not reuse personal OAuth clients.
- Rotate client secret on compromise or offboarding; document rotation in change record.
- CLI reference: [OIDC auth](../oidc-auth.md).

## SIEM and OTel tokens

- Prefer short-lived tokens where the vendor supports it.
- Scope tokens to minimum index / intake permissions.
- Run `send-test` only from approved hosts; redact tokens from all logs attached to tickets.
- Reference: [SIEM and OTel](../siem-otel.md).

## Signing keys (Cosign / Sigstore)

- Private keys never in git; use KMS or hardware-backed storage where required.
- Public keys and provenance policies published through your PKI process.
- Reference: [Release attestation](../release-attestation.md).

## Rotation policy (suggested minimum)

| Secret | Rotation cadence | Triggered rotation |
|--------|------------------|-------------------|
| OIDC client secret | `<e.g. 90 days>` | Offboarding, suspected leak |
| SIEM tokens | `<e.g. 90 days>` | Vendor recommendation, employee exit |
| Signing keys | Per crypto policy | Compromise, algorithm deprecation |

## Rules: never in Slack / tickets / email

1. **Do not** paste raw tokens, private keys, or `Authorization` headers into Slack, Jira, email, or wikis.
2. **Do** attach vault-generated **links** or ticket fields that reference secret **names**, not values.
3. **Do** use scrubbed CLI output for evidence (replace token substrings with `REDACTED`).
4. **Do** report suspected exposure to Security immediately; rotate after assessment.

## Secure storage options (examples)

- Cloud: AWS Secrets Manager, GCP Secret Manager, Azure Key Vault.
- Enterprise: HashiCorp Vault, CyberArk, Delinea.
- Team minimum: encrypted password manager for human-readable backup codes only—not for machine tokens at scale.

## Related documents

- [Bill of materials](bill-of-materials.md) (non-secret references only) · [Monitoring minimum](monitoring-minimum.md)
- [Vendor risk policy](../policies/vendor-third-party-risk-policy.md)
