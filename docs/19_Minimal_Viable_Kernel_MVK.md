# Document 19: Minimal Viable Kernel (MVK)

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-19 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Development leads |
| Depends on | Doc 4, Doc 6 |

## Purpose

MVK scope, priorities, and proof-of-concept deliverables.

The Minimal Viable Kernel (MVK) constrains first delivery to a bootable, testable, and auditable slice of Alani. It is a learning kernel with cognitive hooks, not a full production OS.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Milestone 0: repository creation, CI, formatting, docs.
- Milestone 1: ABI and alani-lib wrappers.
- Milestone 2: kernel boot skeleton and syscall dispatcher.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- MVK includes boot, memory init, scheduler skeleton, syscall dispatcher, terminal path, mock cognition, mock memory, and audit logging.
- MVK excludes full networking, GPU acceleration, multiprocessor scheduling, production secure boot, and persistent encrypted storage.
- MVK uses mocks for devices and model execution where hardware is unavailable.

## Normative Requirements

- ALANI-D19-REQ-01: MVK MUST build from repository templates and run host-mode tests.
- ALANI-D19-REQ-02: MVK SHOULD boot in QEMU once hardware boot path is implemented.
- ALANI-D19-REQ-03: MVK MUST run a userspace demonstration that exercises sys_info, sys_yield, sys_infer mock, and audit verify.
- ALANI-D19-REQ-04: MVK MUST package docs, corpus samples, and source metadata.

## Design and Implementation Guidance

- Milestone 0: repository creation, CI, formatting, docs.
- Milestone 1: ABI and alani-lib wrappers.
- Milestone 2: kernel boot skeleton and syscall dispatcher.
- Milestone 3: runtime and terminal demo.
- Milestone 4: mock cognition and memory path.
- Milestone 5: audit hash chain and packaging.

### Repository Impact

alani-kernel

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- MVK issue labels.
- Milestone checklist.
- Definition of Done.
- Release package layout.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D19-AC-01: All MVK requirements have tests or documented manual verification.
- ALANI-D19-AC-02: Release ZIP contains docs, corpus, repo templates, and scripts.
- ALANI-D19-AC-03: Known limitations are clear and not hidden.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D19-01: QEMU boot path may consume more time than library and host-mode work.
- RISK-D19-02: Model features can distract from OS fundamentals.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 4 | Must be read before implementing this document. |
| Doc 6 | Must be read before implementing this document. |

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

