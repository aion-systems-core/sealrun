# Marketing and Launch Plan (Plan §7)

---

SealRun guarantees deterministic execution, replay symmetry, drift detection, and audit-grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
Kernel isolation modules are contract surfaces only; they define interfaces but do not impose isolation.

This is a deliberate design choice: SealRun is an Execution OS, not a security sandbox OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it can be adopted without admin rights and with minimal operational friction.

If isolation is required (for example in regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro-VM isolation in a future "SealRun Secure Runtime" module without breaking compatibility.

---

## 7.1 Phase 0: Preparation (1-2 weeks)

**Goal:** prepare the repository, documentation, and website.

| Task | Owner | Status |
| :--- | :---: | :---: |
| Create public GitHub repo (MIT license) | You | done |
| Write `README.md` from template (Appendix 11.1) | You | done |
| Prepare `os_contract_spec.md` as a standalone document | You | done |
| Build simple landing page (`aion.sh` or `aion.dev`) with Tailwind/Carrd | You | done |
| Create Twitter/X account `@aion_os` | You | done |
| Optimize LinkedIn profile | You | done |

## 7.2 Phase 1: Hacker News launch (day 1)

**Goal:** maximize visibility in the developer community, gain first GitHub stars, and collect feedback.

| Platform | Action | Timing |
| :--- | :--- | :--- |
| **Hacker News** | "Show HN: SealRun - Deterministic Execution OS & Evidence Fabric for AI Agents" | Tue-Thu, 15:00-16:00 CET |
| **Reddit** | Post in `/r/rust` and `/r/programming` | 1-2 hours after HN |
| **Twitter/X** | Announcement with HN link | In parallel |
| **LinkedIn** | Personal post describing motivation | Evening |

**HN launch text:** see `docs/launch/hacker_news_show_hn.md` (Plan §11.3).

## 7.3 Phase 2: Content marketing (weeks 2-4)

**Goal:** build long-term visibility, SEO, and credibility.

| Content | Platform | Focus |
| :--- | :--- | :--- |
| **Blog post:** "Why we need a deterministic OS for AI" | DEV.to, Hashnode | Problem awareness |
| **Blog post:** "Inside SealRun: deterministic subprocess execution" | Owned blog | Technical depth |
| **YouTube video:** 5-minute demo of `sealrun execute` | YouTube | Visual proof |
| **Case study (fictional):** "How a bank uses SealRun for AI governance" | Website | Enterprise relevance |

## 7.4 Phase 3: Outbound (months 2-6)

**Goal:** acquire first paying customers.

| Action | Target | Approach |
| :--- | :--- | :--- |
| **LinkedIn outreach** | CTOs, CISOs at fintech companies | Personal message with docs link |
| **Conference submissions** | RustConf, KubeCon, AI events | Talk proposal on deterministic execution |
| **Guest posts** | The New Stack, InfoQ | Article on AI compliance |
| **Beta program** | 5-10 companies | Free enterprise license in exchange for feedback |

## 7.5 Metrics and success tracking

| Metric | Goal (month 6) |
| :--- | ---: |
| GitHub stars | > 500 |
| Website visitors / month | > 2,000 |
| Newsletter subscribers | > 200 |
| Enterprise inquiries | > 20 |
| Paying customers | > 3 |
