import {
  Card,
  CardBody,
  CardHeader,
  Divider,
  Grid,
  H1,
  H2,
  Stack,
  Stat,
  Table,
  Text,
} from "cursor/canvas";

export default function SealrunEnterpriseReadinessBenchmark() {
  return (
    <Stack gap={24}>
      <H1>SealRun enterprise readiness benchmark</H1>
      <Text tone="secondary">
        Self-assessment (April 2026). Not a third-party audit, SOC 2 report, or ISO 27001 certificate. After a repository-wide
        documentation audit, canonical content lives under docs/ with a single documentation index.
      </Text>

      <Grid columns={4} gap={16}>
        <Stat value="92 to 96" label="Documentation maturity (post-audit band)" tone="success" />
        <Stat value="90 to 95" label="Pilot readiness maturity" tone="success" />
        <Stat value="P0 to P2" label="Program checklist" tone="success" />
        <Stat value="Customer-owned" label="Certification and boundary testing" tone="warning" />
      </Grid>

      <Text tone="tertiary">
        Repository-wide documentation audit: complete — deduplicated aion/docs mirrors into canonical pointers; added docs/README.md
        index; normalized Flows section naming across enterprise and adapter guides; fixed enterprise changelog relative links.
      </Text>

      <Text tone="tertiary">
        Pilot Readiness Pack: complete under docs/pilot/ (see procurement-mini-pack.md for the index).
      </Text>

      <Divider />

      <H2>Program checklist (P0 through P2)</H2>
      <Table
        headers={["Priority", "Theme", "Status"]}
        rows={[
          ["P0", "Tenant isolation, RBAC, OIDC, evidence chain, replay and drift semantics documented", "Complete"],
          ["P1", "SIEM and OTel export, release attestation, SBOM, Cosign and Sigstore runbooks", "Complete"],
          ["P2", "Policies, compliance matrices, templates, buyer and trust-center packs, adapter guides", "Complete"],
        ]}
      />

      <H2>Audit summary — removed or merged content</H2>
      <Table
        headers={["Area", "Action"]}
        rows={[
          ["aion/docs mirrors", "Merged into canonical pointers to root docs/ for nine topics plus enterprise sales and compliance one-pager"],
          ["Duplicate prose", "Eliminated by single-source policy; aion tree retains unique developer and launch docs only"],
          ["Navigation", "Added docs/README.md as the documentation index linked from README and Trust Center"],
        ]}
      />

      <H2>Consistency improvements</H2>
      <Text tone="secondary">
        Shared Flows section label across enterprise core pages and all adapter guides; SIEM and OpenTelemetry (OTel) wording aligned;
        Trust Center links to the documentation index; enterprise HTML changelog uses repo-relative markdown links.
      </Text>

      <H2>Pilot success metrics (track weekly)</H2>
      <Table
        headers={["Metric", "Source"]}
        rows={[
          ["Reference replay pass rate (target per pilot charter)", "docs/pilot/success-criteria.md"],
          ["SIEM and OTel exporter health and alert coverage", "docs/pilot/monitoring-minimum.md"],
          ["Tenant, RBAC, and OIDC validation checkpoints", "docs/pilot/success-criteria.md, onboarding-script.md"],
          ["Backup or restore exercise completed", "docs/pilot/backup-and-restore.md"],
          ["Weekly feedback and pain-point backlog", "docs/pilot/feedback-loop.md"],
        ]}
      />

      <H2>Remaining items (organization-specific)</H2>
      <Text tone="secondary">
        Formal SOC 2, ISO 27001, or industry attestations; production boundary pen tests; customer-specific IdP and SIEM field mappings;
        data residency and subprocessors review; contractual SLA negotiation.
      </Text>

      <Divider />

      <Grid columns={2} gap={16}>
        <Card>
          <CardHeader title="Procurement readiness" />
          <CardBody>
            <Text tone="secondary">
              Trust center, buyer guide, SLA example, escalation path, status template, security whitepaper, and design-reference
              controls matrices provide a coherent Q and A baseline for security reviews.
            </Text>
          </CardBody>
        </Card>
        <Card>
          <CardHeader title="Compliance artifacts" />
          <CardBody>
            <Text tone="secondary">
              Policies, audit templates, governance bundles, and compliance test suite align terminology across capsule, evidence,
              replay, drift, governance decision, and policy evaluation flows.
            </Text>
          </CardBody>
        </Card>
      </Grid>

      <Grid columns={2} gap={16}>
        <Card>
          <CardHeader title="Operational maturity" />
          <CardBody>
            <Text tone="secondary">
              SRE-style runbooks cover replay failure, drift anomaly, evidence corruption, tenant isolation attempts, and SIEM or OTel
              exporter failure with trigger, detection, mitigation, verification, escalation, and post-incident sections.
            </Text>
          </CardBody>
        </Card>
        <Card>
          <CardHeader title="Ecosystem integration" />
          <CardBody>
            <Text tone="secondary">
              Hugging Face, LangChain, Modal, and BentoML guides share structure and map to policy enforcement, evidence capture, and
              observability exports.
            </Text>
          </CardBody>
        </Card>
      </Grid>

      <H2>Trust signals summary</H2>
      <Text tone="secondary">
        Deterministic replay and drift, tenant-scoped evidence, RBAC exports, OIDC device flow, signed releases with SBOM references,
        and centralized telemetry hooks form a consistent narrative for security and platform stakeholders.
      </Text>

      <Divider />

      <H2>Next steps for pilot execution</H2>
      <Table
        headers={["Step", "Action"]}
        rows={[
          ["1", "Sign scope-definition.md and break-glass-and-ownership.md; freeze bill-of-materials.md"],
          ["2", "Run onboarding-script.md in staging; attach redacted outputs to kickoff ticket"],
          ["3", "Enable monitoring-minimum.md alerts and SIEM field mapping table"],
          ["4", "Schedule weekly feedback-loop.md sessions and demo-golden-path.md dry run"],
          ["5", "Complete backup-and-restore.md exercise before production subset (if in scope)"],
        ]}
      />

      <H2>Long-term documentation governance</H2>
      <Table
        headers={["Practice", "Detail"]}
        rows={[
          ["Single source", "Edit root docs/ only; keep aion/docs pointers for deprecated mirror paths"],
          ["Index first", "Add new hubs to docs/README.md and link from Trust Center when enterprise-relevant"],
          ["Link checks", "Run cargo test -p aion-cli test_docs_links before merge"],
          ["Terminology", "Use the glossary block in docs/README.md for capsule, evidence, replay, drift, OTel, attestation, SBOM"],
        ]}
      />

      <H2>Next 90 days (suggested)</H2>
      <Table
        headers={["Window", "Action"]}
        rows={[
          ["Days 0 to 30", "Pilot SIEM and OTel field mapping on non-production tenants; archive send-test evidence"],
          ["Days 30 to 60", "Automate governance bundle checks in CI using policy-api validate and evaluate fixtures"],
          ["Days 60 to 90", "Run table-top exercises for replay, drift, and tenant isolation runbooks with real RBAC roles"],
        ]}
      />
    </Stack>
  );
}
