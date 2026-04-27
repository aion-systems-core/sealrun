# Break-glass, ownership, and escalation

## Purpose

Assign **accountable owners** for privileged SealRun enterprise actions and define **emergency** paths without bypassing auditability. Works with [scope definition](scope-definition.md) and [support escalation path](../../support-escalation-path.md).

## Role matrix (fill placeholders)

| Function | Primary owner | Backup | Contact |
|----------|---------------|--------|---------|
| Pilot executive sponsor | `<Name>` | `<Name>` | `<email>` |
| Security lead | `<Name>` | `<Name>` | `<phone>` |
| Platform / SRE lead | `<Name>` | `<Name>` | `<phone>` |
| Compliance / legal liaison | `<Name>` | `<Name>` | `<email>` |

## Admin roles (RBAC)

| SealRun role | Allowed actions (summary) | Primary assignees | Break-glass |
|--------------|----------------------------|-------------------|-------------|
| `admin` | Tenant lifecycle, RBAC assignments, broad enterprise CLI | `<group>` | `<named individuals>` |
| `operator` | Replay, diff, limited lifecycle per local policy | `<group>` | Escalate to admin roster |
| `auditor` | Read exports, evidence queries | `<group>` | No break-glass to mutate |
| `viewer` | Read-only | `<group>` | N/A |

Reference: [RBAC](../../rbac.md). Export evidence: `sealrun enterprise rbac export`.

## Legal hold

| Action | Owner | Approval | Evidence |
|--------|-------|----------|----------|
| Enable legal hold | `<role>` | `<sponsor or legal>` | Ticket + CLI transcript |
| Disable legal hold | `<role>` | `<legal>` | Ticket + CLI transcript |

Reference: [Lifecycle controls](../../lifecycle-controls.md).

## Purge

| Action | Owner | Preconditions | Evidence |
|--------|-------|-----------------|----------|
| `lifecycle purge` | `<role>` | Retention elapsed; legal hold **off**; change window | Ticket + CLI transcript |

Mis-purge response: follow [evidence corruption runbook](../runbooks/incident-evidence-corruption.md) if indexes affected.

## Escalation path (with placeholders)

1. **L1 — Pilot desk** `<email or Slack channel>` — triage within `<e.g. 4 business hours>`.
2. **L2 — SRE / platform** `<pager or phone>` — for replay, drift, exporter, storage.
3. **L3 — Security** `<pager>` — for isolation, auth bypass, suspected tampering.
4. **L4 — Executive** `<name>` — contractual or media-impacting events.

Align severities with [SLA](../../sla.md).

## On-call rotation template

| Week starting (UTC) | Primary on-call | Secondary | Notes |
|---------------------|-----------------|-----------|--------|
| `<YYYY-MM-DD>` | `<handle>` | `<handle>` | |
| `<YYYY-MM-DD>` | `<handle>` | `<handle>` | |

Handoff checklist: auth status valid, open incidents reviewed, SIEM exporter synthetic check green (see [monitoring minimum](monitoring-minimum.md)).

## Emergency override protocol

Use only when normal change velocity would cause material harm (safety, regulatory, production outage).

1. **Trigger:** `<describe eligible triggers>`.
2. **Approver:** Minimum `<two roles>` (e.g. Security + Sponsor) verbal + written within `<hours>`.
3. **Action:** Document exact commands, time window, and rollback. No secrets in chat—use [secrets handling](secrets-handling.md).
4. **Retro:** Within `<5 business days>` per [change management policy](../../policies/change-management-policy.md); file [exceptions policy](../../policies/exceptions-policy.md) record if controls were bypassed.

## Related documents

- [Secrets handling](secrets-handling.md) · [Success criteria](success-criteria.md) · [Incident response policy](../../policies/incident-response-policy.md)
