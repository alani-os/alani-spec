# Document 22: Glossary & Acronyms

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-22 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | All readers |
| Depends on | None |

## Purpose

Definitions of project terminology.

The glossary standardizes vocabulary across all Alani specifications and repositories.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- ABI: Application Binary Interface between userspace and kernel.
- Agent: A userspace task that requests cognitive, memory, or device services under capability control.
- Capability: An unforgeable handle authorizing operations on resources.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Use precise OS terminology and avoid anthropomorphic AI language.
- Define ambiguous terms such as agent, cognition, memory, capability, and audit.
- Update glossary when adding public APIs or corpus labels.

## Normative Requirements

- ALANI-D22-REQ-01: New specifications MUST use glossary terms consistently.
- ALANI-D22-REQ-02: Acronyms MUST be expanded on first use in each document.
- ALANI-D22-REQ-03: Terms with security impact MUST reference the authoritative security document.

## Design and Implementation Guidance

- ABI: Application Binary Interface between userspace and kernel.
- Agent: A userspace task that requests cognitive, memory, or device services under capability control.
- Capability: An unforgeable handle authorizing operations on resources.
- Cognition: Bounded model, reasoning, retrieval, or planning service, not consciousness.
- Corpus: Versioned dataset used to train, test, or evaluate Alani cognition and developer tooling.
- MVK: Minimal Viable Kernel, the first executable slice of Alani.
- TraceContext: Identifiers propagated through operations for observability.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- Glossary table.
- Acronym list.
- Cross-reference index.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D22-AC-01: Docs use terms consistently.
- ALANI-D22-AC-02: API names do not conflict with glossary definitions.
- ALANI-D22-AC-03: Corpus label taxonomy refers to glossary where applicable.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D22-01: Terms can drift as code evolves.
- RISK-D22-02: AI-adjacent terms can be overloaded in public communication.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| None | This document is foundational or standalone. |

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

