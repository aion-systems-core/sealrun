# AI CI

SealRun CI includes a Python-oriented `ai-python` job to keep SealRun-AI deterministic and testable.

## Checks

- Python syntax check (`py_compile`)
- Type check (`mypy`)
- Unit tests (`unittest`)
- SealRun CLI availability selftest
- AI dummy-mode selftest with deterministic fallback generation

## Why dummy mode

Dummy mode removes network and model availability dependency from baseline CI health checks.
