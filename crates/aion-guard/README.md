# aion-guard - Deterministic CI Drift Detection

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

```bash
aion guard record --cmd "echo hello"
```

Check for drift:

```bash
aion guard check --cmd "echo hello"
```

Exit codes:

- `0` - no drift  
- `1` - drift detected  
- `2` - baseline missing or invalid  

---

## Kernel Boundary

`aion-guard` dynamically loads the AION Execution Kernel at runtime.  
If the kernel is missing:

```text
AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.
```

---

## License

MIT