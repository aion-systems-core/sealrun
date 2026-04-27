# Service level agreement targets (example)

## Overview

Example **response and mitigation targets** for SealRun enterprise support programs. Contractual SLAs are customer-specific; this document is a baseline for internal operations and procurement discussions. Align targets with **incident response** policy and runbooks covering **replay**, **drift**, **evidence** integrity, **tenant isolation**, and **SIEM** / **OTel** export.

## Support response and resolution targets

| Severity | Description | Initial response target | Resolution or mitigation target |
|----------|-------------|-------------------------|----------------------------------|
| SEV1 | Critical outage or security-impacting incident affecting authentication, isolation, or evidence integrity | 30 minutes | 4 hours to mitigation |
| SEV2 | Major degradation or policy and control enforcement failure | 2 hours | 1 business day |
| SEV3 | Partial degradation with documented workaround | 1 business day | 5 business days |
| SEV4 | Minor issue, documentation defect, or non-critical bug | 2 business days | Best effort within planned release |

## Coverage notes

- Targets assume defined support windows and staffing models.
- Regulated customers may impose additional breach notification timelines independent of this table.
- Link operational evidence to `docs/support-escalation-path.md` and `docs/status-page-template.md`.

## Related documents

- `docs/policies/incident-response-policy.md`
- `docs/enterprise/runbooks/*.md`
- [Trust Center](trust-center.md)
