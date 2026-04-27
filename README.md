# SealRun

SealRun is a deterministic execution engine for AI and automation workloads.
It produces reproducible run artifacts (capsules, replay, drift, evidence, policy outputs) that are machine-checkable and audit-ready.
Use it to standardize execution validation in CI, SRE workflows, and governed release pipelines.

[![CI](https://github.com/aion-systems-core/sealrun/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/aion-systems-core/sealrun/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)

## Key Features

- Deterministic capsule generation for repeatable run records.
- Replay symmetry checks against stored capsule state.
- Drift comparison for controlled change analysis.
- Evidence chain outputs for audit and incident workflows.
- Policy validation surfaces for governance gates.
- Stable JSON output envelopes for automation and tooling.
- Rust-first implementation with workspace-level tests.

## Quickstart

```bash
git clone https://github.com/aion-systems-core/sealrun.git
cd sealrun
cargo build --release
./target/release/sealrun --help
./target/release/sealrun --id quickstart_demo execute ai --model demo --prompt "hello" --seed 42
./target/release/sealrun --id quickstart_replay execute ai-replay --capsule aion_output/ai/quickstart_demo/capsule.aionai
./target/release/sealrun --id quickstart_left observe capture -- echo alpha
./target/release/sealrun --id quickstart_right observe capture -- echo beta
./target/release/sealrun observe drift aion_output/capture/quickstart_left/result.json aion_output/capture/quickstart_right/result.json
./target/release/sealrun doctor
```

## Developer Quickstart

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo test -p aion-cli --test golden_test
bash scripts/smoke_e2e.sh
```

## SealRun-AI

`sealrun_ai/` adds deterministic AI-assisted test generation, evaluator heuristics, fixture generation, and pipeline test helpers.
Start here: [docs/ai/README.md](docs/ai/README.md).

## Enterprise

Enterprise-facing docs cover trust, operations, controls, runbooks, and pilot readiness without changing core deterministic semantics.
Start here: [docs/enterprise/README.md](docs/enterprise/README.md).

## Pricing

Commercial packaging and support details: [docs/pricing.md](docs/pricing.md).

## Contact

contact.sealrun@gmail.com

## Docs Hub

[docs/README.md](docs/README.md)
