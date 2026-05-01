# Document 08: Syscall Interface

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-08 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Kernel and runtime developers |
| Depends on | Doc 6, Doc 7 |

## Purpose

System call definition, conventions, data types, and error model.

The syscall interface is the stable binary contract between userspace/runtime and the kernel. It specifies numbering, calling convention, argument layout, version negotiation, errors, and audit behavior.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Groups: system 0x0000, task 0x0100, memory 0x0200, device 0x0300, cognition 0x0400, security 0x0500, audit 0x0600, debug 0x0700.
- Calling convention: rax syscall number, rdi-r9 arguments on x86_64; RISC-V mapping to a7/a0-a5 is reserved.
- Buffer convention: user pointer is never trusted; kernel copies into bounded kernel buffers or pins memory under explicit rules.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Use numeric syscall identifiers grouped by subsystem.
- Pass fixed-size values in registers and variable-size data through user buffers described by pointer/length pairs.
- Return an integer status plus optional out-buffer contents.
- Version the syscall table independently of repository crate versions.

## Normative Requirements

- ALANI-D08-REQ-01: All syscalls MUST validate pointers, lengths, alignment, and access rights before touching userspace memory.
- ALANI-D08-REQ-02: All syscalls MUST return stable error codes defined in the ABI.
- ALANI-D08-REQ-03: Privileged syscalls MUST require capabilities.
- ALANI-D08-REQ-04: Cognitive syscalls MUST accept budgets and trace context.

## Design and Implementation Guidance

- Groups: system 0x0000, task 0x0100, memory 0x0200, device 0x0300, cognition 0x0400, security 0x0500, audit 0x0600, debug 0x0700.
- Calling convention: rax syscall number, rdi-r9 arguments on x86_64; RISC-V mapping to a7/a0-a5 is reserved.
- Buffer convention: user pointer is never trusted; kernel copies into bounded kernel buffers or pins memory under explicit rules.
- Compatibility: syscall table version query returns major, minor, patch, feature bitmap, and maximum supported buffer size.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- SyscallNumber enum.
- AlaniStatus error enum.
- UserBuffer descriptor.
- CapabilityHandle.
- TraceContext.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D08-AC-01: Doc 9 can be generated from the syscall registry table.
- ALANI-D08-AC-02: alani-lib safe wrappers cover every public syscall.
- ALANI-D08-AC-03: Fuzz tests exercise invalid pointers, lengths, capabilities, and unknown numbers.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D08-01: ABI changes are expensive once userspace relies on them.
- RISK-D08-02: Pointer validation bugs are high severity.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 6 | Must be read before implementing this document. |
| Doc 7 | Must be read before implementing this document. |

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

