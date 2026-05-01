# Document 35: alani-security: README

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-35 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Security developers |
| Depends on | Doc 16 |

## Purpose

Security repository overview and setup.

This readme document for alani-security provides implementation-ready guidance for Rust developers. Provide capability, policy, and crypto abstraction utilities.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Primary aspects: capabilities, policy, crypto traits, attestation stubs.
- The README path should start with purpose, status, quick start, repository layout, feature flags, test commands, and common troubleshooting.
- The design/API path should define module ownership, invariants, public types, errors, examples, and compatibility expectations.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- alani-security owns its public crate boundary and README examples.
- Host-mode tests are required even if the final runtime target is bare-metal.
- Repository templates include CI, rustfmt, clippy, and a minimal source tree.
- Cross-repository assumptions must be represented in Doc 42 rather than hidden in code.

## Normative Requirements

- ALANI-D35-REQ-01: alani-security MUST build independently where possible and MUST document sibling repository requirements.
- ALANI-D35-REQ-02: alani-security MUST expose only the APIs named in this document until an RFC expands them.
- ALANI-D35-REQ-03: Every public item MUST include rustdoc explaining safety, errors, and examples.
- ALANI-D35-REQ-04: Every repository MUST include README, Cargo.toml, src, tests, and CI workflow files.

## Design and Implementation Guidance

- Primary aspects: capabilities, policy, crypto traits, attestation stubs.
- The README path should start with purpose, status, quick start, repository layout, feature flags, test commands, and common troubleshooting.
- The design/API path should define module ownership, invariants, public types, errors, examples, and compatibility expectations.
- The template in repo-templates/individual contains starter files; the script tools/init_repos.sh can copy them into separate repositories.
- The repository should use semantic versioning, but ABI compatibility is controlled by Doc 10 and Doc 42.

### Repository Impact

alani-security

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- alani-security/Cargo.toml
- alani-security/src/lib.rs or src/main.rs
- alani-security/tests
- alani-security/.github/workflows/ci.yml

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D35-AC-01: A developer can create alani-security from the provided template and understand its first implementation tasks.
- ALANI-D35-AC-02: README commands are copy-pasteable after repositories are placed as siblings.
- ALANI-D35-AC-03: Public APIs are traceable to an architecture or data-format document.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D35-01: Repository drift can occur if shared interfaces are duplicated without tests.
- RISK-D35-02: Path dependencies in templates must be adjusted when repositories are published independently.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 16 | Must be read before implementing this document. |

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

