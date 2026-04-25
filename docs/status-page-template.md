# Status page template

## Overview

Use this template for customer-facing or internal status communication when deterministic execution, **replay**, **drift**, governance, **tenant isolation**, or observability exports are degraded.

## Current status

- Overall status: **Operational** / **Degraded** / **Major incident**
- Last updated: `<UTC ISO-8601 timestamp>`
- Incident commander: `<name or rotation ID>`

## Components

| Component | Status | Customer impact | Notes |
|-----------|--------|-----------------|-------|
| Deterministic execution | Operational / degraded / down | | Capsule capture and envelopes |
| Replay and drift | Operational / degraded / down | | Contract validation and promotion gates |
| Governance policy engine | Operational / degraded / down | | Policy evaluation availability |
| Tenant storage | Operational / degraded / down | | Tenant isolation and indexes |
| SIEM / OTel export | Operational / degraded / down | | Detective controls and dashboards |
| Release attestation services | Operational / degraded / down | | Cosign tooling availability, SBOM pipelines |
| OIDC authentication | Operational / degraded / down | | IdP-dependent enterprise login |

## Ongoing incidents

- `<incident ID>`: `<short summary>`, started `<time>`, latest update `<action>`, next update `<time>`

## Planned maintenance

- `<window UTC>`: `<scope>`, expected customer impact `<none / read-only / brief outage>`

## Evidence and follow-up

- Link to internal ticket with **replay** / **drift** artifacts and **governance decision** references.
- Attach post-incident review date per `docs/policies/incident-response-policy.md`.
