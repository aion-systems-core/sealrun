# LangChain adapter guide

## Overview

This guide covers integrating **LangChain** chains and agents with SealRun so that chain outputs become deterministic **capsule** artifacts with an auditable **evidence chain**. **Replay** and **drift** workflows treat the adapter as the contract surface: everything upstream (tools, retrievers, LLM routers) must be reflected in captured config and policy inputs.

## Architecture

- **Execution envelope:** Chain graph, tool registry snapshot, and model routing decisions are serialized for **replay** comparisons.
- **Tenant-aware registration:** Persisted indexes respect **tenant isolation** (see [Multi-tenancy](../multi-tenancy.md)).
- **Governance:** **Policy evaluation** validates models, tool endpoints, and **required_evidence_fields** before downstream orchestration proceeds (see [Policy engine](../policy-engine.md)).

## Flows

1. **Document QA chain:** Snapshot retriever config and embedding model revision; run evaluate step; store capsule + evidence.
2. **Agent with tools:** Allow-list tool HTTP hosts in `allowed_external_calls`; deny dynamic URLs not in policy.
3. **CI regression:** Golden prompt set with fixed seeds; fail pipeline on unexpected **drift**.
4. **Audit:** Export `rbac export` and policy JSON with chain snapshot for access reviews (see [RBAC](../rbac.md)).

## Evidence capture points

- Serialized chain/agent config (tools, prompts redacted per policy).
- **Capsule** and **replay** artifacts, hashes, and lineage IDs.
- **Governance decision** output including violations list.

## Policy enforcement points

- `allowed_models` for each LLM node in the graph.
- `allowed_external_calls` for tools and retrieval backends.
- `required_evidence_fields` for traceability (`trace_id`, `policy_id`, `tenant_id`, plus regulated extras as needed).

## Integration points

- **OIDC** for human operators tuning chains in production namespaces (see [OIDC auth](../oidc-auth.md)).
- **SIEM** / **OTel:** Emit tool invocation denials and policy failures (see [SIEM and OTel](../siem-otel.md)).
- **Lifecycle:** Retention for high-volume chain runs per tenant (see [Lifecycle controls](../lifecycle-controls.md)).

## Compliance notes

- Tool use introduces third-party risk; align with `docs/policies/vendor-third-party-risk-policy.md`.
- Map detective controls to CC-08 and CC-10 in `docs/enterprise/compliance/controls-matrix.md`.

## Next steps

- Add integration tests that call `policy-api validate` / `evaluate` on representative chain descriptors.
- Use `docs/enterprise/templates/audit-evidence-drift-template.md` when promoting graph changes.
- Hub: [Trust Center](../trust-center.md).
