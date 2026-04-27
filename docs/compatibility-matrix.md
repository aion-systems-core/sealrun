# Compatibility matrix

## Purpose

Single table of **version anchors** (product, CLI, capsule schema, Why schema, policy) for integrators validating deterministic compatibility.

This matrix summarizes core version anchors for deterministic compatibility in SealRun.

## At a glance

- Compatibility is contract-governed and version-explicit.
- Capsule, Why, and policy versions are stable integration anchors.
- Use doctor and distribution identity outputs for runtime validation.

| Component | Version |
|-----------|---------|
| SealRun product | `VERSION` file |
| CLI binary | `sealrun --version` |
| AI capsule schema | `version = "1"` |
| Why schema | `why_schema_version = "2"` |
| Governance policy version | `policy_version = "1"` |
| RBAC policy file | `rbac.policy.yaml` (enterprise) |
| Tenant metadata schema | `tenant.json` (enterprise) |
| Capsule index schema | `capsules.index.json` (enterprise) |
| Evidence index schema | `evidence.index.json` (enterprise) |
| Ecosystem integration guides | `docs/integrations/*.md` |

## Upgrade guidance

- Minor/patch upgrades should preserve capsule `version = "1"` compatibility.
- If capsule version changes in future major versions, migration docs must include conversion tooling.

## CLI surface

```bash
sealrun --version
sealrun doctor
sealrun dist identity
```

## Enterprise-readiness

Compatibility is enterprise-ready when supported version combinations remain deterministic, documented, and test-covered. Enterprise reviews should additionally confirm:

- **Tenant isolation** and storage schemas (`tenant.json`, `capsules.index.json`, `evidence.index.json`) match the deployed product version.
- **RBAC** policy file (`rbac.policy.yaml`) and **OIDC** IdP settings are versioned with the release record.
- **Replay** and **drift** baselines are regenerated when capsule or policy versions advance.
- **SIEM** / **OTel** field mappings remain stable or are migrated deliberately with consumer notification.

## Integration readiness (ecosystem scaffolds)

Documented adapter guides (shared structure: overview, architecture, flows, evidence, policy, integration, compliance, next steps) are maintained for:

- [Hugging Face](integrations/huggingface-adapter-guide.md)
- [LangChain](integrations/langchain-adapter-guide.md)
- [Modal](integrations/modal-adapter-guide.md)
- [BentoML](integrations/bentoml-adapter-guide.md)

Governance baselines for **policy evaluation** are documented under `docs/enterprise/governance/bundles/` with validation guidance in [Governance compliance test suite](enterprise/governance/compliance-test-suite.md). Release integrity references [Release attestation](release-attestation.md) for **Cosign** / **Sigstore** and **SBOM** outputs.
