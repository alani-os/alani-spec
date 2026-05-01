# Document 20: Differentiation

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-20 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Stakeholders |
| Depends on | Doc 4, Doc 5 |

## Purpose

How Alani differs from traditional operating systems and AI platforms.

Alani differentiates itself by defining OS-level cognition, persistent cognitive memory, capability-mediated model access, and forensic auditability as first-class primitives in Rust.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Traditional OS distinction: Alani gives cognition an explicit system contract with budgets, audit, and capabilities.
- Agent framework distinction: Alani owns low-level resources and isolation boundaries rather than only orchestrating API calls.
- ML platform distinction: Alani integrates model execution with syscalls, memory, devices, and forensic evidence.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Position Alani as an OS research and implementation framework, not a general Linux replacement.
- Emphasize kernel/user ABI, audit, and corpus-evaluation integration.
- Avoid claims of production readiness before MVK evidence exists.

## Normative Requirements

- ALANI-D20-REQ-01: Differentiation claims MUST be grounded in implemented features or clearly marked as roadmap.
- ALANI-D20-REQ-02: Comparisons MUST distinguish kernel architecture, runtime services, AI agent frameworks, and ML platforms.
- ALANI-D20-REQ-03: Benchmarks MUST disclose environment and limitations.

## Design and Implementation Guidance

- Traditional OS distinction: Alani gives cognition an explicit system contract with budgets, audit, and capabilities.
- Agent framework distinction: Alani owns low-level resources and isolation boundaries rather than only orchestrating API calls.
- ML platform distinction: Alani integrates model execution with syscalls, memory, devices, and forensic evidence.
- Rust distinction: memory safety and explicit unsafe boundaries are part of the platform contract.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- Differentiation matrix.
- Benchmark evidence links.
- Roadmap claims register.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D20-AC-01: Public overview avoids unsupported marketing claims.
- ALANI-D20-AC-02: Differentiation statements point to docs or tests.
- ALANI-D20-AC-03: Stakeholders can see why multiple repositories are necessary.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D20-01: Terminology can imply AI consciousness or autonomous authority; specs must avoid that.
- RISK-D20-02: Competitive comparisons can become stale.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 4 | Must be read before implementing this document. |
| Doc 5 | Must be read before implementing this document. |

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

