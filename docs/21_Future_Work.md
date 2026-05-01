# Document 21: Future Work

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-21 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Planners, developers |
| Depends on | Doc 19, Doc 20 |

## Purpose

Roadmap, research directions, and post-MVK features.

Future work captures planned capabilities after MVK, including multiprocessor scheduling, networking, stronger secure boot, real model runtimes, formal verification, richer corpus sources, and deployment tooling.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Near-term: QEMU boot, serial terminal, expanded syscall tests, corpus validation in CI, audit export CLI.
- Medium-term: SMP, networking, device hotplug, secure boot measurements, policy engine, persistent encrypted memory.
- Research: model scheduler, formal capability proofs, memory-safety verification for unsafe kernel code, differential testing against simulators.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Separate roadmap into near-term, medium-term, and research tracks.
- Require an RFC before promoting future work to committed scope.
- Use corpus/evaluation data to prioritize cognitive features.

## Normative Requirements

- ALANI-D21-REQ-01: Future features MUST identify owning repositories and impacted interfaces.
- ALANI-D21-REQ-02: Research items MUST not block MVK unless accepted by governance.
- ALANI-D21-REQ-03: Roadmap changes MUST update requirements and docs.

## Design and Implementation Guidance

- Near-term: QEMU boot, serial terminal, expanded syscall tests, corpus validation in CI, audit export CLI.
- Medium-term: SMP, networking, device hotplug, secure boot measurements, policy engine, persistent encrypted memory.
- Research: model scheduler, formal capability proofs, memory-safety verification for unsafe kernel code, differential testing against simulators.
- Developer tooling: cargo xtask, repository bootstrapper, doc generator, dashboard for benchmarks and corpus quality.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- Roadmap board.
- RFC template.
- Architecture decision record (ADR).

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D21-AC-01: Each future item has dependencies and non-goals.
- ALANI-D21-AC-02: MVK release notes distinguish implemented from planned.
- ALANI-D21-AC-03: Research items have measurable exit criteria.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D21-01: Roadmap can become a wishlist without owners.
- RISK-D21-02: Hardware-dependent features may need external sponsorship.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 19 | Must be read before implementing this document. |
| Doc 20 | Must be read before implementing this document. |

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

