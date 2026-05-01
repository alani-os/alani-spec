# Document 09: Syscall Reference

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-09 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Kernel, runtime, and userspace developers |
| Depends on | Doc 8 |

## Purpose

Detailed syscall API reference.

The syscall reference enumerates the initial syscall set required for MVK and near-term expansion. Each entry includes purpose, privilege, parameters, return value, errors, audit behavior, and test cases.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Core syscalls: sys_info, sys_yield, sys_exit, sys_time, sys_trace_context.
- Task syscalls: sys_task_spawn, sys_task_join, sys_task_cancel, sys_task_status.
- Memory syscalls: sys_mem_map, sys_mem_unmap, sys_mem_query, sys_mem_share, sys_mem_seal.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Start with a compact syscall surface and grow through RFCs.
- Prefer capability handles to raw global identifiers.
- Use request envelopes for cognitive operations rather than many narrow model syscalls.

## Normative Requirements

- ALANI-D09-REQ-01: Every syscall entry MUST include a stable numeric ID.
- ALANI-D09-REQ-02: Every entry MUST name required capabilities and expected audit events.
- ALANI-D09-REQ-03: Every entry MUST specify whether it is safe in early boot, task context, or interrupt context.
- ALANI-D09-REQ-04: Every entry MUST have at least one positive and one negative test.

## Design and Implementation Guidance

- Core syscalls: sys_info, sys_yield, sys_exit, sys_time, sys_trace_context.
- Task syscalls: sys_task_spawn, sys_task_join, sys_task_cancel, sys_task_status.
- Memory syscalls: sys_mem_map, sys_mem_unmap, sys_mem_query, sys_mem_share, sys_mem_seal.
- Device syscalls: sys_device_list, sys_device_open, sys_device_call, sys_device_close.
- Cognition syscalls: sys_model_list, sys_model_open, sys_infer, sys_memory_query, sys_memory_put.
- Security syscalls: sys_cap_derive, sys_cap_revoke, sys_attest, sys_random.
- Audit syscalls: sys_audit_append, sys_audit_query, sys_audit_verify.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- Syscall registry table in this document.
- alani-lib syscall wrappers.
- Kernel dispatcher implementation.
- Audit event map.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.

| ID | Name | Capability | Arguments | Returns | Audit Event |
|---:|---|---|---|---|---|
| 0x0000 | sys_info | none | out: UserBuffer | AlaniStatus | optional system.info |
| 0x0100 | sys_task_spawn | task.spawn | manifest_ptr, manifest_len, opts_ptr | task_handle | task.spawn |
| 0x0200 | sys_mem_map | memory.map | addr, len, flags | mapped_addr | memory.map |
| 0x0300 | sys_device_call | device.call | device_handle, opcode, in_buf, out_buf | bytes_written | device.call |
| 0x0400 | sys_infer | cognition.infer | model_handle, req_buf, out_buf, budget | result_len | cognition.infer |
| 0x0602 | sys_audit_verify | audit.verify | range_start, range_end, out_buf | proof_len | audit.verify |

## Acceptance Criteria

- ALANI-D09-AC-01: Wrapper names, syscall IDs, and dispatcher table are consistent.
- ALANI-D09-AC-02: Unauthorized calls are denied before resource access.
- ALANI-D09-AC-03: All syscalls emit documented trace data when tracing is enabled.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D09-01: Too many syscalls increase kernel attack surface.
- RISK-D09-02: Overloaded syscalls can produce ambiguous validation rules.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 8 | Must be read before implementing this document. |

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

