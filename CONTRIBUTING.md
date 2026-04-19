# Contributing to AION

AION is a deterministic system.  
All contributions must preserve determinism.

## Principles

- Execution must be reproducible  
- Behavior must be explainable  
- Outputs must be stable  
- Tests must be deterministic

## Commit rules

Every commit must:

- produce the same results when executed multiple times  
- avoid nondeterministic behavior  
- avoid time-based or random outputs unless explicitly controlled  
- avoid hidden state  
- avoid unnecessary abstractions  

## Testing rules

All tests must:

- pass deterministically  
- not depend on external state  
- not depend on system time  
- not depend on randomness  

Run before submitting:

```bash
cargo test -p repro
```

## Repro validation

Changes must not introduce execution drift.

Recommended validation:

```bash
aion repro run -- cargo test -p repro
aion repro run -- cargo test -p repro
aion repro diff last prev
```

## Review expectations

Reviewers will check:

- deterministic behavior  
- absence of drift  
- clarity of execution changes

## Scope

Public-facing changes should:

- use `aion repro ...` in examples  
- avoid internal terminology  
- remain consistent with README.md  

## Releases

Releases represent stable, reproducible states.  
Do not introduce nondeterminism before tagging a release.

## License

By contributing, you agree that your contributions are licensed under the MIT License.
