# Evaluator

`sealrun_ai.ai_evaluator.AITestEvaluator` labels run results deterministically.

## Label set

- `ok`
- `drift`
- `error`
- `policy_violation`
- `unknown`

## Heuristic inputs

- process `exit_code`
- `stderr` error/policy/drift/evidence markers
- optional `drift_score`
- optional `policy_violations`
- optional `evidence_count`

Rules are deterministic and side-effect free so results are reproducible in CI.
