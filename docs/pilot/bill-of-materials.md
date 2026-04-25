# Pilot bill of materials (BOM)

## Purpose

Single **versioned** snapshot of everything needed to reproduce or audit the pilot environment. Update on **every** material change (upgrade, IdP rotation, policy bundle switch). No secrets in this table—use references to vault entries.

## BOM table (copy per environment)

| Field | Value |
|-------|--------|
| BOM ID | `<e.g. PILOT-2026-04-001>` |
| Environment name | `<dev / staging / prod-subset>` |
| Date (UTC) | `<YYYY-MM-DD>` |
| Owner | `<name>` |

### SealRun and policy

| Component | Version / reference |
|-----------|---------------------|
| SealRun / CLI | Output of `sealrun --version` |
| Product `VERSION` file (if used) | `<from repo>` |
| Capsule schema anchor | See [compatibility matrix](../compatibility-matrix.md) |
| **Policy bundle** in use | `<default / strict / regulated-finance / regulated-healthcare / custom>` |
| Policy JSON digest or path | `<path or SHA256>` |
| Governance test evidence | [Compliance test suite](../governance/compliance-test-suite.md) run ID |

### Identity and access

| Component | Value |
|-----------|--------|
| **RBAC** policy file revision | Git SHA or config version |
| **RBAC** roles in use | List assignments summary (attach `rbac export` separately) |
| **OIDC** issuer URL | `<https://issuer/...>` |
| **OIDC** device / token endpoints | `<URLs>` |
| Pilot app client ID (non-secret) | `<client_id>` |
| Allowed audiences / scopes | `<document>` |

### Observability

| Component | Value |
|-----------|--------|
| **SIEM** vendor | `<splunk / datadog / elastic>` |
| **SIEM** intake endpoint (host only) | `<hostname>` |
| **OTel** OTLP HTTP endpoint (host only) | `<hostname>` |
| Log retention class | `<internal classification>` |

### Infrastructure metadata

| Field | Value |
|-------|--------|
| Cloud / DC region | `<region>` |
| OS image or k8s version | `<version>` |
| Disk encryption | `<yes / no>` |
| Network egress proxy | `<yes / no / N/A>` |
| Backup product | `<vendor>` |

## Attachments (store with BOM record)

- Redacted `sealrun doctor` output.
- Redacted `enterprise auth status` (no tokens).
- Link to change ticket and [scope definition](scope-definition.md) version.

## Related documents

- [Secrets handling](secrets-handling.md) · [Success criteria](success-criteria.md) · [Trust Center](../trust-center.md)
