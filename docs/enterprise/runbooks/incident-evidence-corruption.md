# Runbook: Incident — Evidence corruption

## Overview

Evidence artifacts underpin the evidence chain for replay, drift, and governance decisions. This runbook applies when artifacts cannot be parsed, verified, or linked to expected capsule lineage.

## Trigger

- Parser errors, hash mismatches, or missing linkage between evidence index entries and capsules.
- Integrity checks fail after storage maintenance or lifecycle actions.

## Detection

- Automated integrity scans, failed replay prerequisites, or audit tooling failures.
- Operator errors during manual file manipulation (should be prohibited by policy).

## Impact

- Audit reconstruction may be incomplete until evidence is restored or regenerated.
- Legal or regulatory exposure if tampering cannot be ruled out.

## Mitigation

1. Quarantine affected evidence files or index entries; prevent further mutation via legal hold if required ([Lifecycle controls](../../lifecycle-controls.md)).
2. Validate hash and provenance relationships against immutable object storage versioning where available.
3. Attempt deterministic regeneration from source capsule when product semantics allow.
4. Review recent lifecycle actions (`purge`, retention changes) and RBAC audit logs ([RBAC](../../rbac.md)).
5. If tampering is suspected, escalate to security; preserve OIDC session and host forensics per `docs/policies/incident-response-policy.md`.

## Verification

- Spot-check regenerated evidence with successful replay on a sample set.
- Confirm tenant isolation was not violated during the incident window (`docs/enterprise/runbooks/incident-tenant-isolation-breach-attempt.md`).

## Escalation

- Security / compliance for potential integrity or confidentiality incidents.
- Legal for hold notifications when regulated data is involved.

## Post-incident

- Complete impact assessment; document preventive controls (access, backups, immutability).
- Update monitoring for early detection of index inconsistencies; archive findings with `docs/enterprise/templates/audit-evidence-replay-template.md` where replay was used to validate recovery.
