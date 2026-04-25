# Support escalation path

## Overview

Define how incidents and high-severity requests move from intake to specialized teams. This path complements **RBAC** (authorization to act) and **OIDC** (identity): escalation decisions should never bypass documented approval for **exceptions** or **legal hold** changes.

## Escalation tiers

1. **Tier 1 — Support intake**  
   Validate scope, gather **capsule** and **replay** artifacts, classify severity using `docs/sla.md`, open incident record.

2. **Tier 2 — SRE / platform**  
   Investigate deterministic outputs, **drift** reports, **evidence** index health, **SIEM** / **OTel** delivery, and lifecycle actions.

3. **Tier 3 — Security / compliance**  
   **Tenant isolation** attempts, **evidence** corruption, **policy evaluation** bypass concerns, **Cosign** / **Sigstore** incidents, vendor notifications.

4. **Executive escalation**  
   SEV1 events with contractual or regulatory customer communication requirements.

## Required handoff data

- Tenant ID and environment
- Command transcripts (`sealrun --version`, relevant `enterprise` commands, redacted tokens)
- Capsule paths or digests and latest successful **replay** reference
- **Governance decision** or **policy evaluation** JSON when policy-related
- Incident timeline and current mitigation status
- Links to **SBOM** and **attestation** objects when supply chain is implicated

## Related documents

- `docs/runbooks/*.md`
- [Operations guide](operations-guide.md)
- [Trust Center](trust-center.md)
