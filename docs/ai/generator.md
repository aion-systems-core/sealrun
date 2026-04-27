# Generator

`sealrun_ai.ai_generator.AITestGenerator` provides deterministic test generation for engine selftests and pipeline test payloads.

## Responsibilities

- Build strict JSON-only prompts for engine test generation.
- Call Ollama endpoint with healthcheck + deterministic retry profile.
- Parse JSON safely and return list outputs only.
- Use deterministic fallback tests when LLM is unavailable.

## Determinism controls

- `temperature=0.0`
- fixed retry profile and no random backoff
- strict command/flag constraints in prompt text
- fallback templates with fixed ordering
