# SealRun in 5 Minutes (Executive Summary)

## Purpose

A five-minute executive orientation covering deterministic execution, contract controls, and why SealRun is an **Execution OS** (contract layer) rather than a sandbox OS.  
For deeper detail, see [Architecture](../architecture.md).

## At a glance

- Deterministic execution: identical inputs -> verifiable outputs
- Contract-first control surface: replay, drift, evidence, governance, measurement
- Enterprise readiness via the phase 1-12 contract model

---

SealRun guarantees deterministic execution, replay symmetry, drift detection, and audit-grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
Kernel isolation modules are contract surfaces only; they define interfaces but do not impose isolation.

This is a deliberate design choice: SealRun is a deterministic execution engine, not a security sandbox OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it can be adopted in existing environments without admin rights and without operational friction.

If isolation is required (for example in regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro-VM isolation in a future "SealRun Secure Runtime" module without breaking compatibility.

---

## What is SealRun?

SealRun is a **deterministic execution layer for AI runs**: each run is captured as a **capsule** with token trace, evidence chain, explainability (Why), causal graph, and governance checks.

## Why deterministic?

Because **identical inputs** (model, prompt, seed, frozen runtime controls) produce **verifiable** outcomes.  
This enables reliable regression testing, controlled acceptance in regulated environments, and technical traceability instead of "black box + log file."

## Why auditable?

Because artifacts are exported in **structured** formats (JSON/HTML/SVG) and can be bound to **replay**, **drift**, and **policy validation** checks.  
Audits can rely on machine-verifiable contracts instead of screenshots.

## Why evidence-capable?

Because SealRun includes a **linear evidence chain** with rolling digest links.  
Integrity is cryptographically bound, not just text-log metadata.

## Why model-agnostic?

Because contracts are defined over **recorded runs** and **backends** that respect the determinism envelope, not over a single closed-source SDK.

## Why OS instead of framework?

Because SealRun is designed as an **under-application layer**: CLI, SDK, artifact pipelines, governance, and CI integration.  
It is an OS-like foundation for reliable AI execution, not only a library feature.

## CLI surface

```bash
sealrun doctor
sealrun governance status
sealrun reliability status
sealrun measure audits
```

## Enterprise readiness

SealRun is enterprise-ready when deterministic contract outputs remain stable, auditable, and drift-resistant across releases and real operational environments.

---

**Guided link:** [Guided tour](../guided_tour.md) · [Compliance One-Pager](../compliance/sealrun_compliance_onepager.md) · [Evidence model](../evidence/evidence_model.md)
