# Pilot scope definition

## Purpose

Freeze **in-scope** and **out-of-scope** boundaries for the first enterprise pilot to prevent scope creep and ambiguous success claims. Cross-reference [success criteria](success-criteria.md) and [bill of materials](bill-of-materials.md).

## In-scope workloads (fill before kickoff)

| Workload ID | Description | Owner | Environment |
|-------------|-------------|-------|-------------|
| W1 | `<e.g. batch scoring pipeline>` | `<team>` | `<dev / staging>` |
| W2 | `<e.g. nightly replay validation>` | `<team>` | `<staging>` |

Add rows as needed; keep the table short for the first pilot.

## Out-of-scope workloads (explicit)

| Exclusion | Reason | Revisit date |
|-----------|--------|--------------|
| `<e.g. real-time low-latency serving>` | Not validated for determinism in pilot | |
| `<e.g. multi-region active-active>` | Ops maturity not in pilot | |
| `<e.g. PHI / classified tier X>` | Data classification not approved | See [data classification](data-classification.md) |

## Allowed adapters

Document only adapters the pilot will use (see [integration guides](../../integrations/huggingface-adapter-guide.md)):

| Adapter | In pilot? | Notes |
|---------|-----------|--------|
| Hugging Face | `<yes / no>` | |
| LangChain | `<yes / no>` | |
| Modal | `<yes / no>` | |
| BentoML | `<yes / no>` | |
| Custom / other | `<describe>` | Requires security review |

## Allowed IdP

| Field | Value |
|-------|--------|
| IdP vendor | `<e.g. Okta / Entra ID>` |
| OIDC device flow | Enabled for pilot app registration |
| Allowed redirect / device URLs | `<document>` |
| Pilot app client ID (reference, not secret) | `<client_id>` |

Details: [OIDC auth](../../oidc-auth.md).

## Allowed environments

| Environment | Use in pilot | Notes |
|-------------|--------------|--------|
| Developer workstations | `<yes / no>` | |
| Shared dev cluster | `<yes / no>` | |
| Staging | `<yes / no>` | Recommended primary |
| Production (subset) | `<yes / no>` | Requires explicit sponsor sign-off |
| Production (full) | Default **no** for first pilot | |

## Explicit exclusions (non-goals)

1. Formal SOC 2 / ISO 27001 audit as exit criterion (design references only: [controls matrix](../compliance/controls-matrix.md)).
2. New Rust or CLI feature development **during** the pilot unless filed as a separate change program.
3. Unsupported model vendors or hosts not listed in the active [policy engine](../../policy-engine.md) JSON / governance bundle.
4. Customer PII in capsules where [data classification](data-classification.md) forbids it.

## Sign-off

| Role | Name | Date | Signature |
|------|------|------|-------------|
| Pilot sponsor | | | |
| Security | | | |
| Platform / SRE | | | |

## Related documents

- [Success criteria](success-criteria.md) · [Break-glass and ownership](break-glass-and-ownership.md) · [Secrets handling](secrets-handling.md)
