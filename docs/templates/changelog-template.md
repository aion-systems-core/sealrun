# Changelog template

Use this structure for SealRun releases so procurement and operations teams can scan determinism, security, and enterprise operations in one pass.

## Release `<version>` — `<date>` (ISO-8601)

### Highlights

- Summarize customer-visible or operator-visible improvements in plain language.

### Determinism and governance

- Call out changes affecting capsule schema, replay semantics, drift reporting, or policy evaluation defaults.
- Reference governance bundle updates under `docs/governance/bundles/` when applicable.

### Security and compliance

- OIDC, RBAC, tenant isolation, SIEM / OTel, lifecycle controls, or policy documentation changes.
- Cosign / Sigstore attestation workflow updates; SBOM format or generation changes.

### Enterprise operations

- Runbook, SLA, escalation, status template, or trust-center updates tied to this release.

### Breaking changes

- Explicitly state `none` or enumerate migrations, CLI flag changes, and deprecated paths.

### Verification

| Check | Status / link |
|-------|----------------|
| CI status | |
| Test suite | e.g. `cargo test --workspace --all-targets` |
| Attestation status | Sign / verify outcome |
| SBOM archived | Object store link |
| Replay spot-check | Reference capsule IDs |

## Release cadence (example)

- Regular cadence: bi-weekly or monthly minor releases.
- Security or critical hotfixes: as needed, with expedited attestation evidence.

## Evidence retention

- Store this changelog section with the same retention class as release binaries.
- Link SBOM and attestation artifacts using `audit-evidence-release-attestation-template.md`.
