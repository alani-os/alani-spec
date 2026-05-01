# Document 02: System Model

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-02 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Architects, development leads |
| Depends on | Doc 1 |

## Purpose

Overall hardware, OS, execution, and primitive model.

The system model defines the hardware, execution rings, memory domains, process abstraction, interrupt model, and persistence assumptions under which all other Alani specifications are written.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Hardware baseline: 64-bit CPU, MMU, interrupt controller, serial or framebuffer console, timer, and block device or image-backed storage.
- Execution domains: kernel domain, runtime domain, agent domain, device-driver domain where applicable, and audit domain.
- Time model: monotonic kernel ticks for scheduling and trace ordering; wall-clock time is optional until secure time exists.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Model the platform as x86_64 first, with RISC-V as a portability target.
- Use a hybrid architecture: kernel-owned primitives plus userspace subsystems.
- Represent cognitive operations as kernel-mediated services, even when implementation runs in userspace during MVK.
- Distinguish hard real-time, soft real-time, and best-effort cognitive work.

## Normative Requirements

- ALANI-D02-REQ-01: The system MUST define privilege domains for kernel, runtime, userspace agents, and devices.
- ALANI-D02-REQ-02: All memory shared across domains MUST have explicit ownership and lifetime rules.
- ALANI-D02-REQ-03: Interrupt and exception handling MUST be deterministic enough to audit.
- ALANI-D02-REQ-04: The model MUST permit simulation without requiring all target hardware.

## Design and Implementation Guidance

- Hardware baseline: 64-bit CPU, MMU, interrupt controller, serial or framebuffer console, timer, and block device or image-backed storage.
- Execution domains: kernel domain, runtime domain, agent domain, device-driver domain where applicable, and audit domain.
- Time model: monotonic kernel ticks for scheduling and trace ordering; wall-clock time is optional until secure time exists.
- Failure model: panic in kernel is fatal; userspace failure is isolated and recorded as an audit event.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- Boot handoff structure.
- Trap frame representation.
- Memory map descriptor.
- Device discovery table.
- Audit event envelope.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D02-AC-01: A developer can write a host simulator that implements the same domain and event semantics.
- ALANI-D02-AC-02: Every syscall in Doc 9 can be mapped to a system-model primitive.
- ALANI-D02-AC-03: Scheduling and memory docs do not rely on undefined hardware behavior.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D02-01: Hardware-specific assumptions may leak into portable APIs.
- RISK-D02-02: Wall-clock, entropy, and secure storage are easy to over-assume during simulation.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 1 | Must be read before implementing this document. |

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

