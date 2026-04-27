# Risk management policy

## Purpose

Maintain a consistent process for identifying, rating, and treating enterprise risks related to deterministic execution, **evidence chain** integrity, **tenant isolation**, **OIDC** / **RBAC**, observability (**SIEM** / **OTel**), and supply chain (**SBOM**, **Cosign** / **Sigstore**).

## Scope

Applies to platform, security, and product leadership accountable for SealRun adoption.

## Policy statements

1. **Register:** Risks are recorded with likelihood, impact, owner, and treatment plan.
2. **Rating scale:** Low, Medium, High, Critical with documented criteria.
3. **Treatment:** High and Critical risks require treatment owners and due dates; residual acceptance is explicit and signed.
4. **Cadence:** Review the risk register quarterly and after material incidents or architecture changes.
5. **Linkage:** Risks reference controls in `docs/enterprise/compliance/controls-matrix.md` where applicable.

## Risk treatment options

| Option | When to use |
|--------|-------------|
| Mitigate | Engineering or process controls reduce likelihood or impact (preferred). |
| Transfer | Insurance or contractual indemnity where appropriate. |
| Accept | Documented for low residual risk only. |
| Avoid | Decommission feature or vendor relationship. |

## Evidence

- Risk register exports with review minutes.
- Links to incidents (`docs/policies/incident-response-policy.md`) and **exceptions** (`exceptions-policy.md`).

## Compliance references

- `docs/enterprise/compliance/iso27001-annex-a-mapping.md` (organizational controls).
- [Security whitepaper](../security-whitepaper.md) for inherent technical risk context.
