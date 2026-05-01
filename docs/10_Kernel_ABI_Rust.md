# Document 10: Kernel ABI (Rust)

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-10 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Developers, FFI authors |
| Depends on | Doc 9 |

## Purpose

Rust definitions for the kernel-user ABI.

The Rust ABI document defines shared, repr(C)-safe structures and enums used by kernel, runtime, and userspace. It is intentionally smaller than application APIs and must remain binary-stable within a major ABI version.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- The ABI crate should compile in no_std mode with optional serde support for host tooling.
- Never expose Vec, String, Box, trait objects, or Rust references across the syscall boundary.
- Structure evolution uses size fields, version fields, reserved fields, and feature negotiation.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Use #[repr(C)] for structures crossing the ABI.
- Use explicit integer widths and alignment comments.
- Keep Rust enums crossing ABI represented as numeric constants or repr(u32).
- Place ABI definitions in alani-lib and mirror them in kernel tests.

## Normative Requirements

- ALANI-D10-REQ-01: ABI structures MUST avoid Rust-specific layout dependencies unless repr(C) is used.
- ALANI-D10-REQ-02: All pointer fields MUST be represented as usize or u64 and validated by kernel.
- ALANI-D10-REQ-03: All flags MUST specify reserved bits and unknown-bit handling.
- ALANI-D10-REQ-04: ABI version MUST be discoverable at runtime.

## Design and Implementation Guidance

- The ABI crate should compile in no_std mode with optional serde support for host tooling.
- Never expose Vec, String, Box, trait objects, or Rust references across the syscall boundary.
- Structure evolution uses size fields, version fields, reserved fields, and feature negotiation.
- Endianness is native for initial x86_64 target; portable file formats use explicit little-endian encoding.

### Repository Impact

alani-kernel

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- AbiVersion.
- UserBuffer.
- AlaniStatus.
- SyscallNumber.
- CapabilityHandle.
- TraceContext.
- InferenceBudget.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AbiVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserBuffer {
    pub ptr: u64,
    pub len: u64,
    pub flags: u32,
    pub reserved: u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlaniStatus {
    Ok = 0,
    InvalidArgument = 1,
    PermissionDenied = 2,
    NotFound = 3,
    Busy = 4,
    DeadlineExceeded = 5,
    Internal = 0xffff_ffff,
}
```

## Acceptance Criteria

- ALANI-D10-AC-01: ctest or static assertions verify size and alignment.
- ALANI-D10-AC-02: Doc examples compile in alani-lib doctests where possible.
- ALANI-D10-AC-03: ABI changes require a compatibility note and migration plan.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D10-01: Accidental Rust layout assumptions can break FFI.
- RISK-D10-02: Adding fields without size/version discipline can break older runtimes.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 9 | Must be read before implementing this document. |

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

