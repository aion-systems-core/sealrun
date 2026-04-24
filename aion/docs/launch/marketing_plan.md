# Marketingâ€‘ und Launchâ€‘Plan (Plan Â§7)

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and auditâ€‘grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Executionâ€‘OS, not a Securityâ€‘Sandboxâ€‘OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/microâ€‘VM isolation in a future "SealRun Secure Runtime" module â€” without breaking compatibility.

---

## 7.1 Phase 0: Vorbereitung (1â€“2 Wochen)

**Ziele:** Repository vorbereiten, Dokumentation schreiben, Website erstellen.

| Aufgabe | Verantwortlich | Status |
| :--- | :---: | :---: |
| Ã–ffentliches GitHubâ€‘Repo anlegen (MITâ€‘Lizenz) | Du | â¬œ |
| `README.md` nach Vorlage (Anhang 11.1) schreiben | Du | â¬œ |
| `os_contract_spec.md` als separates Dokument aufbereiten | Du | â¬œ |
| Einfache Landingpage (`aion.sh` oder `aion.dev`) mit Tailwind/Carrd | Du | â¬œ |
| Twitter/Xâ€‘Account `@aion_os` erstellen | Du | â¬œ |
| LinkedInâ€‘Profil optimieren | Du | â¬œ |

## 7.2 Phase 1: Hacker News Launch (Tag 1)

**Ziel:** Maximale Sichtbarkeit in der Entwicklerâ€‘Community, erste GitHub Stars, Feedback.

| Plattform | Aktion | Zeitpunkt |
| :--- | :--- | :--- |
| **Hacker News** | â€žShow HN: SealRun â€“ Deterministic Execution OS & Evidence Fabric for AI Agentsâ€œ | Diâ€“Do, 15â€“16 Uhr MEZ |
| **Reddit** | Post in `/r/rust` und `/r/programming` | 1â€“2 Stunden nach HN |
| **Twitter/X** | AnkÃ¼ndigung mit Link zum HNâ€‘Post | Parallel |
| **LinkedIn** | PersÃ¶nlicher Post zur Motivation | Abends |

**HNâ€‘Postâ€‘Text:** siehe `docs/launch/hacker_news_show_hn.md` (Plan Â§11.3).

## 7.3 Phase 2: Contentâ€‘Marketing (Woche 2â€“4)

**Ziel:** Langfristige Sichtbarkeit, SEO, Vertrauensaufbau.

| Inhalt | Plattform | Fokus |
| :--- | :--- | :--- |
| **Blogpost:** â€žWarum wir ein deterministisches OS fÃ¼r KI brauchenâ€œ | DEV.to, Hashnode | Problemâ€‘Bewusstsein |
| **Blogpost:** â€žSealRun unter der Haube: Wie wir Subprozesse deterministisch machenâ€œ | Eigenes Blog | Technische Tiefe |
| **YouTubeâ€‘Video:** 5â€‘Minutenâ€‘Demo von `sealrun execute` | YouTube | Visueller Beweis |
| **Case Study (fiktiv):** â€žWie eine Bank SealRun fÃ¼r AIâ€‘Governance nutztâ€œ | Website | Enterpriseâ€‘Relevanz |

## 7.4 Phase 3: Outbound (Monat 2â€“6)

**Ziel:** Erste zahlende Kunden gewinnen.

| Aktion | Zielgruppe | Vorgehen |
| :--- | :--- | :--- |
| **LinkedIn Outreach** | CTOs, CISOs von FinTechs | PersÃ¶nliche Nachricht mit Link zur Doku |
| **Konferenzâ€‘Einreichungen** | RustConf, KubeCon, AIâ€‘Events | Talk Ã¼ber deterministische AusfÃ¼hrung |
| **Guest Posts** | The New Stack, InfoQ | Gastbeitrag zu AIâ€‘Compliance |
| **Betaâ€‘Programm** | 5â€“10 Unternehmen | Kostenlose Enterpriseâ€‘Lizenz im Austausch fÃ¼r Feedback |

## 7.5 Metriken und Erfolgskontrolle

| Metrik | Ziel (Monat 6) |
| :--- | ---: |
| GitHub Stars | > 500 |
| Websiteâ€‘Besucher / Monat | > 2.000 |
| Newsletterâ€‘Abonnenten | > 200 |
| Enterpriseâ€‘Anfragen | > 20 |
| Zahlende Kunden | > 3 |
