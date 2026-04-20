# AION Guard — Deterministic CI Drift Detection

AION Guard compares command execution against a golden baseline and returns deterministic exit codes for CI/CD pipelines.

## Kernel boundary

Guard contains no kernel implementation.
Guard loads the private `aion-kernel` dynamically and uses only:

- `run_execute(spec)`
- `run_diff(a, b)`
- `run_store(path, artifact)`
- `run_load(path)`

If the kernel cannot be loaded, Guard exits with:

`AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.`

## Commands

```bash
aion-guard record --cmd "echo hello"
aion-guard check --cmd "echo hello"
aion-guard check --cmd "echo world"
```

## Exit codes

- `0`: no drift
- `1`: drift detected
- `2`: runtime/configuration error (missing kernel, missing/corrupt baseline, I/O)

## Drift contract

Default comparison:

- `stdout`
- `stderr`
- `exit_code`

Optional comparison:

- `duration_ms` with tolerance

## Baseline format

Baseline data is owned by the kernel and accessed through:

- `run_store(path, artifact)`
- `run_load(path)`
