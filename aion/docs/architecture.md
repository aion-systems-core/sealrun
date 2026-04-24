# Architecture

SealRun Execution OS architecture defines deterministic kernel-layer execution and enterprise-layer contract control.

## At a glance

- Kernel-layer model: deterministic replay, drift, evidence, policy paths
- Enterprise-layer model: governance, ops, dist, UX, tests, measurement
- Shared invariants: identity, finality, consistency, deterministic output envelopes

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and auditâ€‘grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Executionâ€‘OS, not a Securityâ€‘Sandboxâ€‘OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/microâ€‘VM isolation in a future "SealRun Secure Runtime" module â€” without breaking compatibility.

---

## Kernel-layer diagram

```text
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ State-Layer (Replay-Contract)                               â”‚
â”‚ - capsule is canonical JSON with stable profile fields       â”‚
â”‚ - replay reads capsule state without semantic drift           â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Process-Layer (Replay-Invariant)                            â”‚
â”‚ - replay checks capsule shape and canonicalization order      â”‚
â”‚ - replay symmetry uses fixed checks and tokenized errors      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Map-Layer (Drift-Contract)                                  â”‚
â”‚ - drift labels and categories are sorted and deterministic    â”‚
â”‚ - drift profile thresholds are fixed and reproducible         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Evidence-Layer (Evidence-Chain)                             â”‚
â”‚ - evidence chain stores rolling hashes and replay anchors     â”‚
â”‚ - evidence output is audit-stable across replay runs          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Policy-Layer (Policy-Engine)                                â”‚
â”‚ - policy validates capsule and profile constraints linearly   â”‚
â”‚ - policy decisions emit deterministic contract errors         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

## Execution OS guarantees

- Deterministic execution: replay and profile checks use fixed order and stable contracts.
- Reproducible states: capsule and replay artifacts are canonicalized before comparison.
- Auditable evidence: evidence chain and replay anchors provide stable proof records.
- Governance rules: policy engine enforces deterministic constraints over capsule/profile data.

## Enterprise-layer expansion

The 5-layer execution model is the kernel view. Enterprise operation expands this with contract families surfaced via `sealrun doctor` and dedicated CLI groups:

- governance and policy hardening (`sealrun policy`, `sealrun governance`)
- reliability and operations (`sealrun reliability`, `sealrun ops`)
- distribution and supportability (`sealrun dist`)
- developer and enterprise UX (`sealrun ux`)
- test strategy and compatibility (`sealrun tests`)
- measurement and audit evidence (`sealrun measure`)

All outputs are deterministic JSON envelopes and map to sections in `docs/os_contract_spec.md`.

## Contract surface

- Kernel contracts: State, Process, Map, Evidence, Policy
- Consistency contract: run/capsule/evidence/replay finality
- Enterprise contracts: phase 1-12 contract families linked in `os_contract_spec.md`

## CLI surface

```bash
sealrun doctor
sealrun governance status
sealrun reliability status
sealrun ops runbooks
sealrun dist identity
sealrun ux api
sealrun tests strategy
sealrun measure audits
```

## Global Consistency Contract

`GlobalConsistencyContract` defines machine-readable finality states for:

- `run_finality`: replay, drift, policy, and evidence statuses must be `ok`.
- `capsule_finality`: capsule must be complete and referencable by evidence/replay anchors.
- `evidence_finality`: evidence chain must verify and have no open replay anchors.
- `replay_finality`: replay invariant, symmetry, and cross-machine checks must be `ok`.

The contract is evaluated in deterministic order and emitted by `sealrun doctor` in `global_consistency`.

## Enterprise-readiness

- Kernel-layer determinism is necessary but not sufficient.
- Enterprise readiness is achieved when kernel + enterprise layers remain contract-consistent and auditable across releases.

## Terminology

- capsule
- replay
- drift
- evidence
- policy
- profile

