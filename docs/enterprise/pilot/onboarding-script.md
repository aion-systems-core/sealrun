# Pilot onboarding script

## Purpose

Repeatable **day-one** sequence for operators joining the pilot. Execute in order; capture outputs for the first [bill of materials](bill-of-materials.md) entry. Assumes [scope definition](scope-definition.md) is signed and secrets exist per [secrets handling](secrets-handling.md).

## Prerequisites

- SealRun binary installed and on `PATH`.
- Network egress to IdP, SIEM, and OTel endpoints allow-listed.
- **RBAC** subject provisioned; **OIDC** app registration ready.

## Step 1 — Environment health (`doctor`)

```bash
sealrun doctor
```

**Expected:** Structured JSON with ordered checks; no undocumented free-text error blobs. Save redacted output to pilot folder in ticket system.

## Step 2 — Enterprise login (`auth`)

```bash
sealrun enterprise auth login \
  --client-id <CLIENT_ID> \
  --device-authorization-endpoint <DEVICE_URL> \
  --token-endpoint <TOKEN_URL>
sealrun enterprise auth status
```

**Expected:** `auth status` shows authenticated session (exact fields per product). Do not paste tokens.

Reference: [OIDC auth](../../oidc-auth.md).

## Step 3 — Test tenant

```bash
sealrun enterprise tenants list
sealrun enterprise tenants create <PILOT_TENANT_ID>
sealrun enterprise lifecycle retention set --tenant <PILOT_TENANT_ID> --days <N>
```

**Expected:** Tenant appears in list; retention command succeeds.

Reference: [Multi-tenancy](../../multi-tenancy.md), [Lifecycle controls](../../lifecycle-controls.md).

## Step 4 — Reference replay

Use the **golden capsule** path agreed in [scope definition](scope-definition.md):

```bash
sealrun enterprise tenants capsules replay --tenant <PILOT_TENANT_ID> --capsule <PATH_TO_GOLDEN_CAPSULE>
```

**Expected:** Success path documented in pilot kickoff; failures trigger [replay failure runbook](../runbooks/incident-replay-failure.md).

## Step 5 — SIEM `send-test`

```bash
sealrun enterprise sinks send-test --sink splunk --endpoint <URL> --token <REDACTED_USE_ENV>
# or datadog / elastic per scope
```

**Expected:** HTTP success from sink; event visible in SIEM search within agreed latency.

Reference: [SIEM and OTel](../../siem-otel.md).

## Step 6 — OTel export smoke

```bash
sealrun enterprise otel export --endpoint <OTLP_HTTP_ENDPOINT>
```

**Expected:** Collector accepts payload; verify in backend UI.

## Step 7 — Policy API validate / evaluate

Prepare `policy.json` and `input.json` per [policy engine](../../policy-engine.md) and [compliance test suite](../governance/compliance-test-suite.md):

```bash
sealrun enterprise policy-api validate --policy policy.json
sealrun enterprise policy-api evaluate --policy policy.json --input input.json
```

**Expected:** `validate` passes; `evaluate` returns structured pass/fail with violations list for negative fixtures.

## Step 8 — RBAC export snapshot

```bash
sealrun enterprise rbac export
```

**Expected:** YAML consistent with pilot roster ([break-glass and ownership](break-glass-and-ownership.md)).

## Completion checklist

- [ ] All steps above saved (redacted) under pilot BOM.
- [ ] First [feedback loop](feedback-loop.md) session scheduled.

## Related documents

- [Monitoring minimum](monitoring-minimum.md) · [Demo golden path](demo-golden-path.md) · [Product pilot tutorials](00_install.md) (hands-on capsule flow)
