# Drift

**Drift** answers: â€œwhat changed between two runs?â€ For AI capsules, drift is computed over stable fields (tokens, evidence digests, embedded Why/graph projections, etc.).

## At a glance

- Drift classifies deterministic differences between runs.
- Output categories and labels are tokenized and machine-readable.
- Drift contributes to finality and release admission decisions.

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and auditâ€‘grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Executionâ€‘OS, not a Securityâ€‘Sandboxâ€‘OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/microâ€‘VM isolation in a future "SealRun Secure Runtime" module â€” without breaking compatibility.

---

## CLI: drift between two run JSON files (observe)

```bash
cargo run -p aion-cli -- observe drift left.json right.json
```

Produces drift JSON/HTML/SVG under `aion_output/drift/<timestamp>/`.

## CLI: drift between two capsules (SDK)

```bash
cargo run -p aion-cli -- sdk drift --a first.aionai --b second.aionai
```

Exit code **2** when drift is detected (useful in CI).

## Example drift JSON (shape)

```json
{
  "changed": true,
  "fields": ["tokens", "seed"],
  "details": ["â€¦"]
}
```

## Contract surface

- Map-Contract (Drift-Contract) with fixed categories and tolerance profile
- Global Consistency integration via run finality
- Measurement inputs for trend and KPI reporting

## CLI surface

```bash
sealrun observe drift left.json right.json
sealrun sdk drift --a first.aionai --b second.aionai
sealrun doctor
```

## Related

- [Replay](replay.md)
- [Governance](governance.md)

## Enterprise-readiness

Drift is enterprise-ready when labels/categories and tolerance outcomes remain deterministic across CI and production comparisons.
