# Roadmap and Milestones (Plan §8)

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## 8.1 Short term (0-3 months)

- [ ] Open-source launch (HN, Reddit)
- [ ] Collect and prioritize community feedback
- [ ] Bug fixes and stability improvements
- [ ] First version of the enterprise website with pricing
- [ ] Launch beta program with 3-5 companies

## 8.2 Mid term (3-9 months)

- [ ] Implement **Secure Runtime** (filesystem/network isolation via Landlock/seccomp)
- [ ] Complete **Governance Pack** (compliance exports, policy editor)
- [ ] Deliver **CI/CD Pack** with GitHub Action and GitLab CI component
- [ ] Onboard first paying customers
- [ ] Secure a speaker slot at a conference (RustConf, KubeCon)

## 8.3 Long term (9-18 months)

- [ ] Build **Enterprise Dashboard** (web UI) as an optional add-on
- [ ] Integrate **SPIFFE/SPIRE** for workload identity
- [ ] Add **Micro-VM mode** (Firecracker, Cloud Hypervisor) for stronger isolation
- [ ] Achieve **SOC2 / ISO27001** company certification (for enterprise sales)
- [ ] Expand the team (1-2 engineers, 1 sales/support)
