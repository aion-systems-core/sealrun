# AION roadmap

AION is a deterministic execution operating system: user-facing tools sit above **COS** (the kernel), which owns audit records, evidence, and replay contracts. Tools consume kernel-shaped artifacts; they do not redefine execution truth.

This document is descriptive only. Ordering is intent, not a delivery promise.

---

## Current state

### COS v1 (kernel)

- **Purpose:** Single crate surface (`cos_core`) for deterministic audit, evidence, and replay types.
- **Scope:** Types and module boundaries; no host I/O in the kernel crate itself.
- **Status:** Stable for downstream consumption; changes require version discipline.

### Repro v1 (field study #1)

- **Purpose:** Deterministic capture, diff, causal `why`, replay, and CI ledger over local artifacts.
- **Scope:** `repro` binary and `aion repro` router path; storage under `./repro_runs/` and `./repro_ci_store/` as documented.
- **Status:** Hardened for the current contract (schema, INDEX, event streams).

---

## Phase — Tool 2: trace

- **Purpose:** Read-only inspection and visualization of execution timelines from existing artifacts (no new kernel semantics).
- **Scope:** CLI and formatting over data COS/Repro already emit; deterministic ordering and layout.
- **Non-goals:** Mutating captures, new on-disk formats, background daemons, policy engines.
- **Done means:** Spec (`TRACE_SPEC.md`) implemented; `cargo test` and repro contract tests green; no new writes to capture stores by default.

---

## Phase — Tool 3: inspect

- **Purpose:** Deep, structured introspection of a single run (identity slices, env snapshots, trace linkage) without replaying side effects.
- **Scope:** Field-level reports derived from existing JSON and event streams.
- **Non-goals:** Arbitrary code execution in the tool, network calls, heuristic “fix” suggestions.
- **Done means:** Documented CLI; output stable byte-for-byte for fixed inputs; errors are structured and non-zero on failure.

---

## Phase — Tool 4: simulate

- **Purpose:** Deterministic what-if analysis over **recorded** state transitions (counterfactuals within declared models), not live mutation of production systems.
- **Scope:** Explicit simulation inputs and outputs; same reproducibility bar as Repro.
- **Non-goals:** Real-time control planes, ML-based guessing, non-reproducible randomness.
- **Done means:** Simulation contract published; golden tests for small scenarios; documented limits of validity.

---

## Phase — Tool 5: governance

- **Purpose:** Policy and provenance layers: who may run what, under which deterministic contracts, with auditable decisions.
- **Scope:** Metadata, signing hooks, and policy evaluation **without** weakening COS invariants.
- **Non-goals:** Replacing the kernel; silent policy overrides; non-deterministic scoring.
- **Done means:** Governance actions are themselves recordable and replayable; integration tests cover allow/deny paths deterministically.
