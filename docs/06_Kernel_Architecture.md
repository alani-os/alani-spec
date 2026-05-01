# Document 06: Kernel Architecture

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-06 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Kernel developers, systems engineers |
| Depends on | Doc 5 |

## Purpose

Kernel subsystems: boot, scheduler, memory, IPC, devices, and security.

The kernel architecture defines the no_std core responsible for boot, CPU setup, memory management, scheduling, interrupts, device registry, syscall dispatch, security gatekeeping, and audit emission.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Boot path: loader handoff, early console, CPU feature validation, memory map parse, page table setup, interrupt descriptor setup, allocator init, device probe, runtime spawn.
- Module boundaries: boot, arch, memory, scheduler, task, ipc, syscall, devices, security, audit, panic, diagnostics.
- Locking policy: no blocking locks in interrupt context; prefer spin locks only for short critical sections; use explicit lock ordering.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Keep the kernel final binary independent of high-level AI crates.
- Expose internal modules behind minimal traits so host tests can mock hardware.
- Make syscall dispatch table-driven and versioned.
- Avoid dynamic allocation before allocator initialization is complete.

## Normative Requirements

- ALANI-D06-REQ-01: The kernel MUST initialize CPU, memory, interrupt, timer, scheduler, device, security, audit, and syscall subsystems in a deterministic order.
- ALANI-D06-REQ-02: The kernel MUST reject unknown syscall numbers with a stable error code.
- ALANI-D06-REQ-03: The kernel MUST maintain per-task capability state.
- ALANI-D06-REQ-04: Kernel panics MUST attempt to emit a final diagnostic event when safe.

## Design and Implementation Guidance

- Boot path: loader handoff, early console, CPU feature validation, memory map parse, page table setup, interrupt descriptor setup, allocator init, device probe, runtime spawn.
- Module boundaries: boot, arch, memory, scheduler, task, ipc, syscall, devices, security, audit, panic, diagnostics.
- Locking policy: no blocking locks in interrupt context; prefer spin locks only for short critical sections; use explicit lock ordering.
- Error policy: kernel functions return typed errors until fatal invariants fail, then panic with event metadata.

### Repository Impact

alani-kernel

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- KernelDevice trait for low-level device adapters.
- SyscallFrame for syscall arguments and return values.
- TaskControlBlock for scheduling state.
- AuditSink for structured audit event submission.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D06-AC-01: Kernel repository contains module stubs matching this architecture.
- ALANI-D06-AC-02: Host-mode unit tests exercise syscall dispatch and scheduler policy without booting hardware.
- ALANI-D06-AC-03: Boot logs show each initialization phase in order.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D06-01: Early boot debugging is hard without serial output.
- RISK-D06-02: Trait abstraction can hide interrupt-safety constraints unless documented.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
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

