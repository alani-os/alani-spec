# Document 39: alani-terminal: README

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-39 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Userspace developers |
| Depends on | Doc 17 |

## Purpose

Terminal/shell repository overview and command usage.

This readme document for alani-terminal provides implementation-ready guidance for Rust developers. Expose an interactive shell for diagnostics and demonstrations.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Primary aspects: commands, scripting, trace view, agent run.
- The README path should start with purpose, status, quick start, repository layout, feature flags, test commands, and common troubleshooting.
- The design/API path should define module ownership, invariants, public types, errors, examples, and compatibility expectations.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- alani-terminal owns its public crate boundary and README examples.
- Host-mode tests are required even if the final runtime target is bare-metal.
- Repository templates include CI, rustfmt, clippy, and a minimal source tree.
- Cross-repository assumptions must be represented in Doc 42 rather than hidden in code.

## Normative Requirements

- ALANI-D39-REQ-01: alani-terminal MUST build independently where possible and MUST document sibling repository requirements.
- ALANI-D39-REQ-02: alani-terminal MUST expose only the APIs named in this document until an RFC expands them.
- ALANI-D39-REQ-03: Every public item MUST include rustdoc explaining safety, errors, and examples.
- ALANI-D39-REQ-04: Every repository MUST include README, Cargo.toml, src, tests, and CI workflow files.

## Design and Implementation Guidance

- Primary aspects: commands, scripting, trace view, agent run.
- The README path should start with purpose, status, quick start, repository layout, feature flags, test commands, and common troubleshooting.
- The design/API path should define module ownership, invariants, public types, errors, examples, and compatibility expectations.
- The template in repo-templates/individual contains starter files; the script tools/init_repos.sh can copy them into separate repositories.
- The repository should use semantic versioning, but ABI compatibility is controlled by Doc 10 and Doc 42.

### Repository Impact

alani-terminal

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- alani-terminal/Cargo.toml
- alani-terminal/src/lib.rs or src/main.rs
- alani-terminal/tests
- alani-terminal/.github/workflows/ci.yml

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D39-AC-01: A developer can create alani-terminal from the provided template and understand its first implementation tasks.
- ALANI-D39-AC-02: README commands are copy-pasteable after repositories are placed as siblings.
- ALANI-D39-AC-03: Public APIs are traceable to an architecture or data-format document.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D39-01: Repository drift can occur if shared interfaces are duplicated without tests.
- RISK-D39-02: Path dependencies in templates must be adjusted when repositories are published independently.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
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

