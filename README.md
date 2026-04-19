# AION

AION — Make Execution Explainable.

Deterministic execution debugging — capture, diff, explain, replay.

AION is a deterministic execution truth layer for debugging, comparison, and reproducible automation.

It captures what actually happened during a command, compares executions deterministically, and explains why they differ.

If you have ever seen the same command succeed once and fail the next time — AION makes the difference visible.

## About

Suggested GitHub topics for this repository:

`determinism`, `reproducibility`, `devops`, `command-line`, `observability`, `ci-cd`, `debugging`, `rust`, `cli`, `diff-tool`, `testing-tools`

---

## Why AION exists

Commands do not behave consistently.

The same command can produce different results across machines, environments, or time.

Logs are incomplete.  
Debuggers do not capture environment drift.  
CI systems hide nondeterminism instead of explaining it.

Reproducibility is broken in practice.

AION exists to make execution behavior:

* visible  
* comparable  
* explainable  
* reproducible  

---

## What AION is

AION is a system for deterministic execution analysis.

It is composed of multiple surfaces:

* Repro — deterministic capture, diff, why, replay  
* Graph — execution relationships and causality (future)  
* Envelope — deterministic execution contracts (future)  
* Trace — event-based execution recording (future)  
* Inspect — execution introspection (future)  

Repro is the first available surface.

---

## 5-second proof

```bash
aion repro run -- echo hello
aion repro diff last prev
aion repro why last prev
```

AION captures executions, compares them, and explains the difference.

## What you get

Capture — see exactly what happened during a run

Compare — see what changed between runs

Explain — understand why it changed

Replay — reproduce output without re-running

Artifacts are stored locally under ./repro_runs/.

## Installation

From the repository root:

```bash
cargo build --release -p aion -p repro
export PATH="$PWD/target/release:$PATH"
```

## Quickstart

```bash
aion repro run -- echo hello
aion repro replay last
aion repro diff last prev
aion repro why last prev
```

## Examples

Runnable examples are available in:

examples/basic_run.sh

examples/diff_example.sh

examples/why_analysis.sh

## CI Integration Examples

The snippets below sketch CI, local collaboration, and benchmarking flows. They are written as integration targets; align flags and subcommands with the current `aion repro` / `repro` CLI (see Quickstart) wherever your toolchain differs.

### GitHub Actions Example

```yaml
- name: Install AION
  run: cargo install --git https://github.com/aion-systems-core/aion

- name: Record baseline (main branch)
  run: aion repro run --label main -- ./scripts/build.sh

- name: Record current (PR branch)
  run: aion repro run --label pr -- ./scripts/build.sh

- name: Compare outputs
  run: aion repro diff main pr --ci-check
```

### Local Debugging Example

```bash
aion repro run --label my-env -- python train.py
aion fetch colleague-env.json
aion repro diff my-env colleague-env --why
```

### Reproducible Benchmarking Example

```bash
for i in {1..10}; do
    aion repro run --label "bench_$i" -- ./benchmark --iterations 1000
done
aion repro diff bench_1 bench_10 --strict
```

## Release

See RELEASE.md for version information and changes.

## Contributing

See CONTRIBUTING.md.

## License

MIT
