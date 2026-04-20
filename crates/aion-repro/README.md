# AION Repro — Deterministic Run Capture & Replay

AION Repro records command execution deterministically and provides replay/diff/why workflows through the private kernel.

## Kernel boundary

Repro contains no kernel implementation.
Repro loads the private `aion-kernel` dynamically and uses only:

- `run_execute(spec)`
- `run_diff(a, b)`
- `run_store(path, artifact)`
- `run_load(path)`

If the kernel cannot be loaded, Repro exits with:

`AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.`

## Commands

```bash
aion-repro run -- echo hello
aion-repro replay repro_runs/last.json
aion-repro diff repro_runs/a.json repro_runs/b.json
aion-repro why repro_runs/a.json repro_runs/b.json
```

## Exit codes

- `0`: success
- `2`: runtime/configuration error

## Data path

Artifact persistence is delegated to kernel APIs:

- `run_store(path, artifact)`
- `run_load(path)`
