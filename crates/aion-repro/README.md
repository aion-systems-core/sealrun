# aion-repro - Deterministic Run Capture & Replay

`aion-repro` provides deterministic run capture, replay, diff, and why-analysis.  
It freezes command execution into reproducible artifacts and allows replay without re-running the original command.

---

## Features

- Deterministic run capture  
- Replay without executing the command again  
- Diff (stdout, stderr, exit code)  
- Why-analysis for understanding differences  
- Reproducible artifacts for debugging and audits  

---

## Usage

Capture a run:

```bash
aion repro run -- echo "hello"
```

Replay a previous run:

```bash
aion repro replay <id>
```

Diff two runs:

```bash
aion repro diff <id-a> <id-b>
```

Why-analysis:

```bash
aion repro why <id-a> <id-b>
```

---

## Kernel Boundary

`aion-repro` dynamically loads the AION Execution Kernel at runtime.  
If the kernel is missing:

```text
AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.
```

---

## License

MIT