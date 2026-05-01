# Document 13: Device Model

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-13 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Kernel and driver developers |
| Depends on | Doc 6 |

## Purpose

Driver framework, interrupts, DMA, registration, and isolation.

The device model defines how hardware and simulated devices are discovered, registered, addressed, authorized, interrupted, and called from kernel or userspace.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Device classes: console, timer, block, entropy, network-sim, sensor-sim, accelerator, memory, audit-storage.
- Lifecycle: Discover, Probe, Register, Configure, Open, Operate, Suspend, Resume, Remove.
- Interrupt model: top-half acknowledges hardware and records event; bottom-half or task context performs heavier work.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Represent devices by class, capabilities, and operations rather than ad hoc functions.
- Keep unsafe register access inside driver modules.
- Use a common command envelope for userspace device calls.
- Support mock devices for host tests and corpus generation.

## Normative Requirements

- ALANI-D13-REQ-01: Drivers MUST declare supported operations, required capabilities, and DMA behavior.
- ALANI-D13-REQ-02: Interrupt handlers MUST perform minimal work and defer processing.
- ALANI-D13-REQ-03: Device calls MUST validate buffer sizes and direction flags.
- ALANI-D13-REQ-04: Device registry MUST prevent duplicate IDs and unauthorized opens.

## Design and Implementation Guidance

- Device classes: console, timer, block, entropy, network-sim, sensor-sim, accelerator, memory, audit-storage.
- Lifecycle: Discover, Probe, Register, Configure, Open, Operate, Suspend, Resume, Remove.
- Interrupt model: top-half acknowledges hardware and records event; bottom-half or task context performs heavier work.
- DMA policy: memory must be pinned, bounded, and mapped through IOMMU-like abstractions where available.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- Device trait.
- DeviceClass.
- DeviceId.
- DeviceOp.
- DeviceRegistry.
- InterruptBinding.
- DmaBuffer.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D13-AC-01: A mock console and mock memory device can run in host tests.
- ALANI-D13-AC-02: Device open requires a capability and emits audit.
- ALANI-D13-AC-03: Invalid opcodes return Unsupported without side effects.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D13-01: Unsafe hardware access can violate Rust safety assumptions.
- RISK-D13-02: Device emulation may diverge from real hardware timing.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
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

