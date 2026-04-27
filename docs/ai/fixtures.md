# Fixtures

`sealrun_ai.fixtures.ensure_fixtures()` generates minimal deterministic local JSON fixtures used by runner pipeline tests.

## Generated files

- `exec_capsule.json`
- `replay_capsule.json`
- `drift_left.json`
- `drift_right.json`
- `evidence_capsule.json`
- `policy_capsule.json`

## Purpose

These fixtures provide stable local artifacts for replay/drift/evidence/policy test paths when external systems are unavailable.
