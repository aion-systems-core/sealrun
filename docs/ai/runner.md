# Runner

`sealrun_ai.ai_runner.AITestRunner` executes generated tests through the configured SealRun binary and evaluates outcomes.

## Responsibilities

- Deterministic command mapping from high-level AI commands to concrete CLI subcommands.
- Subprocess execution with structured result payloads (`exit_code`, `stdout`, `stderr`).
- Selftest orchestration (`engine_selftest`).
- Pipeline variant execution (`pipeline_test`) with deterministic fixture usage.

## Safety checks

- Validation of command and flag envelopes before execution.
- Structured error result for missing commands or binary lookup failure.
