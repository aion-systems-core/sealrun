# Runbook: Incident — Tenant isolation breach attempt

## Overview

Tenant isolation ensures capsule and evidence partitions do not leak across tenants. This runbook covers suspected or confirmed cross-tenant access attempts, mis-bound CLI context, or authorization bugs.

## Trigger

- Alerts on impossible cross-tenant references in logs.
- Failed authorization checks with suspicious resource identifiers.
- Responsible disclosure or penetration test finding.

## Detection

- Application and storage access logs with tenant identifiers.
- RBAC denials correlated with unusual resource patterns.
- SIEM correlation rules for tenant mismatch events ([SIEM and OTel](../siem-otel.md)).

## Impact

- Potential confidentiality breach; may trigger regulatory notification obligations (customer-owned analysis).

## Mitigation

1. Capture request context: actor identity (OIDC subject where available), session, attempted resource IDs, timestamps.
2. Confirm RBAC and tenant binding evaluations; freeze high-risk accounts pending investigation ([RBAC](../rbac.md), [OIDC auth](../oidc-auth.md)).
3. Block subject or session; preserve logs and evidence with legal hold if needed ([Lifecycle controls](../lifecycle-controls.md)).
4. Verify whether unauthorized data access occurred; sample evidence-query audit logs per tenant.
5. Notify security and compliance stakeholders per `docs/support-escalation-path.md`.

## Verification

- Re-run isolation tests; confirm indexes and CLI paths cannot reproduce the issue on patched builds.
- Review policy evaluation paths that might have accepted malformed tenant fields ([Policy engine](../policy-engine.md)).

## Escalation

- Executive notification for SEV1 confirmed breach scope per customer contracts.
- Vendor engagement if root cause is upstream storage or SDK.

## Post-incident

- Publish internal postmortem; track corrective actions (code, configuration, monitoring).
- Update drift and replay regression tests to include tenant-negative cases where feasible.
