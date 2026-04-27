# Pilot feedback loop

## Purpose

Capture **structured** feedback every week so the pilot improves product, docs, and operations without relying on hallway decisions. Links to [success criteria](success-criteria.md) metrics.

## Weekly 30-minute sync template

**Attendees:** `<sponsor, platform lead, pilot engineer, optional security>`  
**Cadence:** Same weekday/time each week (UTC: `<time>`)

### Agenda (copy to calendar)

1. **Metrics (5 min)** — Review green/yellow/red from [success criteria](success-criteria.md).
2. **Incidents (5 min)** — Open/closed; any follow-ups for runbooks?
3. **Pain points (10 min)** — Top 3 from pilot engineers; assign owners.
4. **Requests (5 min)** — CLI flags, docs, policy tweaks; classify must-fix vs backlog.
5. **Next week (5 min)** — Demos, milestones, risks.

## Pain point capture (log template)

| Date | Reporter | Area | Description | Severity | Owner | Target date |
|------|----------|------|-------------|----------|-------|-------------|
| | | replay / drift / policy / auth / SIEM / other | | | | |

## CLI and API requests

| Request ID | Request | Business justification | Product decision | Link |
|------------|---------|------------------------|------------------|------|
| | | | approved / deferred / declined | ticket |

*Note: Pilot feedback informs roadmap; it does not bypass change control for Rust or CLI surfaces without a product decision.*

## Documentation gaps

| Gap | Page / command | Suggested fix | Owner |
|-----|----------------|---------------|-------|
| | | | |

## Success metric tracking

Copy the metric table from [success criteria](success-criteria.md) and update weekly:

| Metric | Week 1 | Week 2 | Week 3 | Week 4 |
|--------|--------|--------|--------|--------|
| G1 replay rate | | | | |
| G3 SIEM health | | | | |
| … | | | | |

## Related documents

- [Onboarding script](onboarding-script.md) · [Bill of materials](bill-of-materials.md) · [Public roadmap](../../roadmap-public.md)
