# OIDC authentication

## Overview

SealRun enterprise authentication supports **OIDC** using the **device authorization** (device-code) flow for CLI-first environments. **OIDC** complements **RBAC**: the IdP establishes identity; SealRun **RBAC** enforces authorization for **replay**, lifecycle, and tenant operations. Tokens are local session artifacts and must be protected like other secrets on the workstation.

## Architecture

1. CLI requests a device code from the configured **OIDC** authorization server.
2. User completes verification in a browser (user code entry or equivalent).
3. CLI polls the token endpoint and persists tokens for subsequent enterprise commands.
4. **Logout** clears local session state; **status** supports operational checks and evidence collection.

## Flows

1. **Interactive login:** Operator runs `auth login` with client and endpoint parameters from your IdP registration.
2. **Session verification:** Run `auth status` in runbooks and CI debug steps before privileged actions.
3. **Session end:** `auth logout` on shared jump hosts or after incident shifts.
4. **Break-glass:** Document non-OIDC emergency procedures in `docs/policies/access-control-policy.md` and record any **governance decision** for exceptions.

## Evidence capture points

- Redacted `auth status` output for access reviews (no raw tokens in tickets).
- IdP audit logs correlating device-code completion to enterprise actions.
- Change records for client ID, scopes, and endpoint rotations.

## Policy enforcement points

- Enterprise commands assume an authenticated session where the product requires it.
- **RBAC** applies after authentication; see [RBAC](rbac.md).

## Integration points

- **SIEM** / **OTel:** Forward IdP and CLI security events where available (see [SIEM and OTel](siem-otel.md)).
- **Multi-tenancy** and **evidence chain** queries: authenticated operator identity should bind to audit narratives in your SOC process.
- **Sigstore** / **Cosign** and **SBOM** processes are orthogonal to **OIDC** but should use the same change management discipline (see [Release attestation](release-attestation.md)).

## Compliance notes

- Map **OIDC** controls to CC-02 in `docs/enterprise/compliance/controls-matrix.md` and technological controls in `docs/enterprise/compliance/iso27001-annex-a-mapping.md`.
- Align with `docs/policies/access-control-policy.md` and `docs/security-guide.md` session guidance.

## Next steps

- Register a dedicated OAuth/OIDC client for SealRun CLI with least-privilege scopes.
- Automate periodic `auth status` checks in operator checklists: [Operations guide](operations-guide.md).
- Hub: [Trust Center](trust-center.md).

## CLI reference

```bash
sealrun enterprise auth login \
  --client-id <client> \
  --device-authorization-endpoint <url> \
  --token-endpoint <url>
sealrun enterprise auth status
sealrun enterprise auth logout
```
