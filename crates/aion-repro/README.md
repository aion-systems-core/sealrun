# aion-repro — Deterministic Run Capture & Replay

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

aion repro run -- echo "hello"

Code

Replay a previous run:

aion repro replay <id>

Code

Diff two runs:

aion repro diff <id-a> <id-b>

Code

Why-analysis:

aion repro why <id-a> <id-b>

Code

---

## Kernel Boundary

`aion-repro` dynamically loads the AION Execution Kernel at runtime.  
If the kernel is missing:

AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.

Code

---

## License

MIT
