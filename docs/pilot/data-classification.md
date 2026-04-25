# Data classification (pilot)

## Purpose

Define what **may** and **may not** enter **capsules**, **evidence**, and operator logs during the pilot to avoid accidental PII exposure or regulatory breach. Aligns with [scope definition](scope-definition.md) and [secrets handling](secrets-handling.md).

## Classification tiers (example — customize)

| Tier | Description | Pilot default |
|------|-------------|---------------|
| Public | Non-sensitive marketing or synthetic data | Allowed in fixtures |
| Internal | Non-production business data | Allowed with sponsor approval |
| Confidential | Customer business data | Allowed only under written scope |
| Restricted | PHI, payment card, government classified | **Out of pilot** unless explicit program |

## What may enter capsules

- Synthetic or anonymized inputs approved for the golden path.
- Non-secret configuration: model **names**, seed integers, allow-listed URLs matching [policy engine](../policy-engine.md).
- Opaque internal identifiers that are not personally identifying (e.g. internal workload IDs agreed with Legal).

## What must NOT enter capsules

- Raw **PII** (government IDs, full DOB, full address, biometric data) unless tier and DPA explicitly allow.
- **Secrets**: API keys, passwords, private keys, session tokens, SIEM tokens (see [secrets handling](secrets-handling.md)).
- Full production payloads from unrestricted production systems unless classified as in-scope.

## PII rules (pilot minimum)

1. **Default deny** for PII in pilot artifacts unless Legal documents an exception.
2. If minimal PII is required (e.g. internal user email for audit), use **pseudonymous IDs** in capsules; map in a separate access-controlled store outside default evidence export.
3. **Redact** before attaching CLI output to tickets: replace emails with `user@redacted`, tokens with `REDACTED`.

## Customer-ID handling

- Prefer **internal pseudonymous** customer keys in capsules (`cust_internal_123`) over customer legal names.
- If real customer identifiers are required, tag BOM and retention class; restrict **RBAC** `auditor` access to subsets per [RBAC](../rbac.md).

## Redaction guidelines

| Data type | Action |
|-----------|--------|
| OAuth tokens / cookies | Never capture; redact if leaked into logs |
| IP addresses | Redact last octet for sharing unless security incident |
| Hostnames | Often acceptable; confirm with sponsor |
| Free-text prompts | Treat as confidential; trim in demos |

## Related documents

- [Demo golden path](demo-golden-path.md) · [Procurement mini-pack](procurement-mini-pack.md) · [Security guide](../security-guide.md)
