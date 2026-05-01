# Document 11: Memory Management

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-11 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Kernel developers |
| Depends on | Doc 6 |

## Purpose

Physical/virtual memory, allocators, buffers, and persistence.

Memory management covers early physical memory discovery, virtual address space layout, allocator phases, shared buffers, persistent memory abstractions, and memory safety invariants.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Address layout includes kernel higher-half mapping, direct physical map if supported, per-task user ranges, MMIO ranges, guard pages, and trampoline code.
- Allocators: early bump allocator for boot; bitmap or buddy frame allocator for physical frames; linked-list or slab heap for kernel objects.
- Sharing: sys_mem_share returns a handle, sys_mem_seal prevents further writes, sys_mem_unmap revokes task mapping.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Use phased allocator initialization: bump allocator, frame allocator, heap allocator.
- Use page-based isolation for tasks and memory objects.
- Represent shared memory through explicit handles and sealable regions.
- Treat persistent cognitive memory as a higher-level service, not raw kernel heap.

## Normative Requirements

- ALANI-D11-REQ-01: The kernel MUST parse and preserve bootloader memory map entries.
- ALANI-D11-REQ-02: The kernel MUST protect kernel text, read-only data, stacks, and device MMIO regions.
- ALANI-D11-REQ-03: Userspace buffers MUST be copied or pinned with explicit lifetime rules.
- ALANI-D11-REQ-04: Memory errors MUST not leak privileged contents.

## Design and Implementation Guidance

- Address layout includes kernel higher-half mapping, direct physical map if supported, per-task user ranges, MMIO ranges, guard pages, and trampoline code.
- Allocators: early bump allocator for boot; bitmap or buddy frame allocator for physical frames; linked-list or slab heap for kernel objects.
- Sharing: sys_mem_share returns a handle, sys_mem_seal prevents further writes, sys_mem_unmap revokes task mapping.
- Persistent memory: alani-memory owns typed records and flush semantics; kernel only mediates access and isolation.

### Repository Impact

alani-memory

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- FrameAllocator trait.
- VirtualMemoryManager.
- UserBufferValidator.
- SharedMemoryHandle.
- MemoryDevice trait from alani-memory.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D11-AC-01: Page table unit tests validate flags and range overlap.
- ALANI-D11-AC-02: Invalid user buffers never read or write kernel memory.
- ALANI-D11-AC-03: Memory statistics can be queried for diagnostics.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D11-01: Allocator bugs can corrupt kernel state.
- RISK-D11-02: DMA and shared memory can bypass isolation if not constrained.

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

