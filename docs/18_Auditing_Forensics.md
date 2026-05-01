# Document 18: Auditing & Forensics

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-18 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Security auditors |
| Depends on | Doc 16, Doc 17 |

## Purpose

Tamper-evident audit framework and forensic procedures.

Auditing and forensics define tamper-evident event capture, policy, integrity verification, export, retention, and investigation workflows.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Forensic workflow: preserve source image, export audit segment, verify hash chain, reconstruct trace graph, correlate with corpus/evaluation records, write findings.
- Policy: security denials, capability derivation, device opens, memory sharing, model inference, corpus import, release packaging, and CI waiver events are audit-critical.
- Retention: MVK keeps local segments; production design supports rotation, archival, and encrypted storage.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Use append-only log segments with hash-chain integrity.
- Separate audit events from debug logs and metrics.
- Record policy decisions and privileged operations by default.
- Support offline verification of exported audit bundles.

## Normative Requirements

- ALANI-D18-REQ-01: Audit records MUST include event ID, sequence, previous hash, timestamp/counter, principal, operation, resource, decision, status, and redaction state.
- ALANI-D18-REQ-02: Audit storage MUST reject out-of-order or malformed records unless in recovery mode.
- ALANI-D18-REQ-03: Verification MUST detect deletion, insertion, and modification within a segment.
- ALANI-D18-REQ-04: Audit queries MUST enforce access control.

## Design and Implementation Guidance

- Forensic workflow: preserve source image, export audit segment, verify hash chain, reconstruct trace graph, correlate with corpus/evaluation records, write findings.
- Policy: security denials, capability derivation, device opens, memory sharing, model inference, corpus import, release packaging, and CI waiver events are audit-critical.
- Retention: MVK keeps local segments; production design supports rotation, archival, and encrypted storage.
- Evidence: every incident package includes audit segment, manifest, verification proof, build identity, and operator notes.

### Repository Impact

alani-audit

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- AuditEvent.
- AuditSegment.
- AuditProof.
- AuditQuery.
- ForensicBundle.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D18-AC-01: Tampering with sample audit JSONL causes verifier failure.
- ALANI-D18-AC-02: Audit query denial is itself audited.
- ALANI-D18-AC-03: Forensic bundle format is documented and testable.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D18-01: Audit logs can contain sensitive data if redaction is inconsistent.
- RISK-D18-02: Hash chains prove integrity only from the trusted root or exported anchor.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 16 | Must be read before implementing this document. |
| Doc 17 | Must be read before implementing this document. |

| Related Artifact | Path |
|---|---|
| Repository templates | repo-templates/individual/ |
| Corpus samples | corpus/data/ |
| Tooling scripts | tools/ and corpus/scripts/ |
| Architecture diagrams | docs/assets/ |

## References

- Alani Executive Summary, attached source specification.
- Rust Book - packages, crates, and modules: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
- Cargo Book - workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Rust Style Guide: https://doc.rust-lang.org/style-guide/
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/about.html
- GitHub Docs - GitHub Flow: https://docs.github.com/en/get-started/using-github/github-flow
- GitHub Docs - Building and testing Rust: https://docs.github.com/en/actions/tutorials/build-and-test-code/rust
- RustSec Advisory Database: https://rustsec.org/

