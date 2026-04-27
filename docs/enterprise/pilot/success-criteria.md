# Pilot success criteria (4–8 weeks)

## Purpose

Define **measurable** outcomes for the first SealRun enterprise pilot so stakeholders can declare success, pause, or pivot without debate. Align criteria with [scope definition](scope-definition.md), [onboarding script](onboarding-script.md), and [bill of materials](bill-of-materials.md).

## Time horizon

| Phase | Duration | Intent |
|-------|----------|--------|
| Week 0 | Kickoff | BOM frozen, scope signed, break-glass roster live |
| Weeks 1–4 | Core pilot | Must-have criteria tracked weekly |
| Weeks 5–8 | Hardening (optional) | Should-have and stretch metrics |

## Measurable goals (4–8 weeks)

| ID | Goal | Target | Evidence |
|----|------|--------|----------|
| G1 | Reference **replay** success rate | ≥ 95% of scheduled reference replays pass on agreed golden capsules | CI or scheduler logs, [audit replay template](../templates/audit-evidence-replay-template.md) |
| G2 | **Drift** disposition SLA | 100% of pilot-flagged drift events have documented accept / reject / escalate within 5 business days | [Drift template](../templates/audit-evidence-drift-template.md), tickets |
| G3 | **SIEM** / **OTel** visibility | ≥ 99% successful `send-test` / export checks weekly; zero silent exporter gaps > 24h undetected | Monitoring dashboards, [monitoring minimum](monitoring-minimum.md) |
| G4 | **Tenant isolation** checks | Zero confirmed cross-tenant access; all suspected events triaged within SLA | [Tenant isolation runbook](../runbooks/incident-tenant-isolation-breach-attempt.md) |
| G5 | **RBAC** / **OIDC** validation | 100% of pilot operators complete device login; RBAC export reviewed once per fortnight | `enterprise auth status`, `enterprise rbac export` |
| G6 | **Policy evaluation** in CI or gate | ≥ 90% of pilot merges touching policy pass `policy-api validate` in pipeline | CI artifacts |
| G7 | **Backup / restore** | One successful table-top or live restore exercise completed | [Backup and restore](backup-and-restore.md) sign-off |
| G8 | Stakeholder feedback | ≥ 4 weekly syncs held; top 5 pain points logged with owners | [Feedback loop](feedback-loop.md) |

Adjust percentages and windows with the pilot sponsor.

## Green / yellow / red summary

| Area | Green | Yellow | Red |
|------|--------|--------|-----|
| **Replay** | Meets G1; failures isolated and documented | G1 85–94%; root cause unknown for over one week | G1 below 85% or data loss suspected |
| **Drift** | G2 met; policy alignment reviewed | Backlog of undisposed drift | Critical drift in prod without halt |
| **SIEM** / **OTel** | G3 met; alerts fire on synthetic failure | Missed exporter gap under 24h once | Silent gap over 24h or no alert path |
| **Tenant** / **RBAC** / **OIDC** | G4–G5 met | Single triaged false positive; delayed RBAC review | Confirmed isolation breach or auth bypass |
| **Policy** | G6 met | Manual override frequently needed | Policy bypass or unaudited exception |
| **Backup** | G7 complete | Exercise scheduled but not done | No restorable copy proven |
| **Engagement** | G8 met | 2–3 syncs missed with catch-up | Sponsor disengaged; no decision log |

## Related documents

- [Scope definition](scope-definition.md) · [Onboarding script](onboarding-script.md) · [Demo golden path](demo-golden-path.md)
- [Trust Center](../../trust-center.md) · [SLA](../../sla.md) (example targets)
