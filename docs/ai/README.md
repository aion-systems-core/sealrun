# SealRun-AI documentation hub

SealRun-AI is the deterministic helper subsystem for AI-assisted test generation and pipeline-oriented selftests around the SealRun CLI.

## Modules

- [generator.md](generator.md) — LLM integration, prompt hardening, fallback behavior.
- [runner.md](runner.md) — deterministic execution orchestration and command mapping.
- [evaluator.md](evaluator.md) — deterministic labeling heuristics for run outcomes.
- [fixtures.md](fixtures.md) — reproducible local fixtures for replay, drift, evidence, and policy flows.
- [ci.md](ci.md) — CI checks for syntax, typing, unit tests, and dummy-mode selftest.

## Scope and constraints

- Deterministic outputs only.
- JSON parsing before returning generated test sets.
- Fallback behavior when LLM endpoints are unavailable.
- Windows-safe execution support via configurable `sealrun_bin` path.
