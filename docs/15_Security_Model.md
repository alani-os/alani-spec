# Document 15: Security Model

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-15 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Security architects |
| Depends on | Doc 4 |

## Purpose

Threat model, principals, trust boundaries, and security goals.

The security model identifies assets, actors, trust boundaries, threats, assumptions, and required security properties for the Alani OS family.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Assets: kernel code, page tables, capabilities, keys, audit logs, corpus records, model weights, prompts, outputs, persistent memory, device firmware.
- Threats: malicious userspace agent, compromised model, hostile corpus entry, malicious driver, replayed audit log, confused deputy, side-channel leakage.
- Boundaries: bootloader-kernel, kernel-runtime, runtime-agent, kernel-device, cognition-memory, audit-storage, CI-release.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Use capability-based authorization as the primary runtime model.
- Use secure boot and attestation as target features, with MVK stubs.
- Treat models, corpus records, memory vectors, audit logs, and device data as protected assets.
- Classify agent output as untrusted until policy allows action.

## Normative Requirements

- ALANI-D15-REQ-01: The model MUST define principals: kernel, runtime, agents, operators, devices, services, and corpus contributors.
- ALANI-D15-REQ-02: The model MUST enumerate trust boundaries crossed by every syscall and device call.
- ALANI-D15-REQ-03: The model MUST require least privilege for tasks and devices.
- ALANI-D15-REQ-04: The model MUST include confidentiality, integrity, availability, and accountability goals.

## Design and Implementation Guidance

- Assets: kernel code, page tables, capabilities, keys, audit logs, corpus records, model weights, prompts, outputs, persistent memory, device firmware.
- Threats: malicious userspace agent, compromised model, hostile corpus entry, malicious driver, replayed audit log, confused deputy, side-channel leakage.
- Boundaries: bootloader-kernel, kernel-runtime, runtime-agent, kernel-device, cognition-memory, audit-storage, CI-release.
- Security posture: default deny; privileged actions require explicit capability; errors must not disclose sensitive state.

### Repository Impact

alani-security

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- Threat matrix.
- Capability taxonomy.
- Security classification labels.
- Audit requirements for security decisions.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D15-AC-01: Each privileged syscall maps to a capability.
- ALANI-D15-AC-02: Threat model is reviewed before MVK release.
- ALANI-D15-AC-03: Corpus records include license and sensitivity metadata.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D15-01: Model and data supply chains are difficult to secure.
- RISK-D15-02: Bare-metal vulnerabilities can compromise all higher layers.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 4 | Must be read before implementing this document. |

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

