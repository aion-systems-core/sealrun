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
        Self-assessment after documentation refinement (April 2026). Not a third-party audit, SOC 2 report, or ISO 27001 certificate.
      </Text>

      <Grid columns={3} gap={16}>
        <Stat value="88 to 92" label="Documentation and procurement pack maturity (internal band)" tone="success" />
        <Stat value="P0 to P2" label="Program checklist" tone="success" />
        <Stat value="Customer-owned" label="Certification and boundary testing" tone="warning" />
      </Grid>

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
              exporter failure with trigger, detection, mitigation, verification, and escalation sections.
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
