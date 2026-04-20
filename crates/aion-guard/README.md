# aion-guard — Deterministic CI Drift Detection

`aion-guard` provides deterministic drift detection for CI/CD pipelines.  
It compares command execution against a recorded baseline and returns stable exit codes.

---

## Features

- Baseline recording  
- Drift detection (stdout, stderr, exit code)  
- Optional duration tolerance  
- Deterministic CI exit codes  
- Reproducible, auditable comparisons  

---

## Usage

Record a baseline:

aion guard record --cmd "echo hello"

Code

Check for drift:

aion guard check --cmd "echo hello"

Code

Exit codes:

- `0` — no drift  
- `1` — drift detected  
- `2` — baseline missing or invalid  

---

## Kernel Boundary

`aion-guard` dynamically loads the AION Execution Kernel at runtime.  
If the kernel is missing:

AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.

Code

---

## License

MIT
