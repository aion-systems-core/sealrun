# AION — Deterministic Execution OS

AION is a deterministic Execution-OS for CI/CD, developer workflows, and enterprise automation.  
This public repository contains tooling only. The kernel is private (`aion-kernel`).

## Repository split

- Private repository: `aion-kernel`
- Public repository: `aion` (this repo)

## Public architecture

AION OS
|
|-- aion-kernel (private)
|     |-- run_execute(spec)
|     |-- run_diff(a, b)
|     |-- run_store(path, artifact)
|     `-- run_load(path)
|
|-- aion-repro (public tool)
|     |-- capture
|     |-- replay/diff/why
|     `-- C-ABI kernel client
|
|-- aion-guard (public tool)
|     |-- baseline record/check
|     |-- deterministic CI exit codes
|     `-- C-ABI kernel client
|
`-- aion-cli (public router)
      `-- routes to aion-repro / aion-guard

## FFI boundary

Public tools load the private kernel dynamically and call only:

- `run_execute(spec)`
- `run_diff(a, b)`
- `run_store(path, artifact)`
- `run_load(path)`

Kernel loading uses:

- `dlopen` / `dlsym` on Unix-like systems
- `LoadLibrary` / `GetProcAddress` on Windows

If the kernel library is unavailable, tools fail with:

`AION Kernel not found. Install aion-kernel or set AION_KERNEL_PATH.`

## Build

```bash
cargo build --workspace --release
```

## Tool READMEs

- [`crates/aion-repro/README.md`](crates/aion-repro/README.md)
- [`crates/aion-guard/README.md`](crates/aion-guard/README.md)
