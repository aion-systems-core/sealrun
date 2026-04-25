# Backup and restore (pilot)

## Purpose

Ensure **capsule**, **evidence**, and **index** data for the pilot can be recovered after operator error, storage corruption, or infrastructure loss. One successful exercise is a common pilot exit criterion (see [success criteria](success-criteria.md)).

## What to back up

| Asset | Typical location (example) | Backup method | RPO target | RTO target |
|-------|----------------------------|---------------|------------|------------|
| **Capsule** store | `sealrun_enterprise` tenant directories; org-specific layout | Snapshot / object replication | `<fill>` | `<fill>` |
| **Evidence** store | Co-located or separate evidence objects | Same as capsules or replicated bucket | `<fill>` | `<fill>` |
| **Capsule index** | `capsules.index.json` (per tenant) | Versioned snapshot with store | `<fill>` | `<fill>` |
| **Evidence index** | `evidence.index.json` | Versioned snapshot with store | `<fill>` | `<fill>` |
| **Tenant metadata** | `tenant.json` | Config backup with change control | `<fill>` | `<fill>` |
| **RBAC policy** | `rbac.policy.yaml` | Git or secure config service | `<fill>` | `<fill>` |

Exact paths depend on deployment; document actual paths in [bill of materials](bill-of-materials.md).

## Backup frequency

| Tier | Frequency | Retention |
|------|-----------|-----------|
| Pilot staging | Daily minimum while pilot active | `<e.g. 14 days>` |
| Pilot production subset | Per org policy; not less than daily during active pilot | `<fill>` |

## Restore exercise checklist (table-top or live)

- [ ] **Pre:** Confirm change window and pilot sponsor notification.
- [ ] **Isolate:** Halt writes to affected tenant or environment if doing partial restore.
- [ ] **Restore:** Restore capsule objects, evidence objects, then indexes (order per platform team procedure).
- [ ] **Verify:** Run reference **replay** on known-good capsule; compare hash or diff to pre-incident baseline.
- [ ] **Policy:** Re-run `policy-api validate` on active policy JSON ([policy engine](../policy-engine.md)).
- [ ] **Auth:** Confirm `enterprise auth status` and `rbac export` match expected.
- [ ] **Telemetry:** Run `send-test` and OTel export smoke ([SIEM and OTel](../siem-otel.md)).
- [ ] **Record:** Attach transcripts to ticket; update [bill of materials](bill-of-materials.md) if versions changed.

## Verification steps

1. **Integrity:** Spot-check file counts and checksums vs backup manifest.
2. **Tenant isolation:** Run negative test—no cross-tenant reads ([multi-tenancy](../multi-tenancy.md)).
3. **Legal hold:** If hold was active during backup, confirm hold state restored per [lifecycle controls](../lifecycle-controls.md).

## Failure handling

- Suspected tampering: [evidence corruption runbook](../runbooks/incident-evidence-corruption.md).
- Partial restore: document gaps explicitly for audit.

## Related documents

- [Secrets handling](secrets-handling.md) · [Break-glass and ownership](break-glass-and-ownership.md) · [Onboarding script](onboarding-script.md)
