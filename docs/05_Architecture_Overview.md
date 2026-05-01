# Document 05: Architecture Overview

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-05 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Architects, new developers |
| Depends on | Doc 2, Doc 3 |

## Purpose

Big-picture architecture, components, and flows.

The architecture is a hybrid OS architecture: a small authority-bearing kernel coordinates memory, scheduling, devices, capabilities, and audit while userspace services implement richer runtime and cognition behavior.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Control flow starts at boot, enters kernel init, creates runtime/init, then exposes terminal and userspace applications.
- Data flow for cognition moves from user request to runtime envelope to kernel authorization to cognition service and memory/device backends.
- Audit flow is append-only: operation metadata, capability identity, resource identifiers, decision, result, and hash-chain link.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Use a kernel-centered control plane with userspace cognition service for MVK.
- Keep alani-lib as the ABI and utility glue used by runtime, terminal, and userspace.
- Represent memory, devices, security, and audit as independently testable crates.
- Use a meta-workspace only for integration, while repositories can stand alone.

## Normative Requirements

- ALANI-D05-REQ-01: The architecture MUST define owner repositories for each component.
- ALANI-D05-REQ-02: All cross-domain calls MUST use syscalls, capabilities, or versioned messages.
- ALANI-D05-REQ-03: Component dependencies MUST remain acyclic except through the kernel ABI.
- ALANI-D05-REQ-04: The architecture MUST include observability paths from every subsystem.

## Design and Implementation Guidance

- Control flow starts at boot, enters kernel init, creates runtime/init, then exposes terminal and userspace applications.
- Data flow for cognition moves from user request to runtime envelope to kernel authorization to cognition service and memory/device backends.
- Audit flow is append-only: operation metadata, capability identity, resource identifiers, decision, result, and hash-chain link.
- Failure flow distinguishes recoverable userspace errors, policy denials, device faults, and fatal kernel faults.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- Architecture diagrams are provided as Mermaid files in docs/assets.
- Interface ownership is summarized in Doc 42.
- Data formats are specified in Doc 43.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D05-AC-01: A developer can follow a request from terminal command to syscall to audit event.
- ALANI-D05-AC-02: Adding a new device class has a documented path through alani-devices, alani-kernel, and alani-audit.
- ALANI-D05-AC-03: The architecture document remains consistent with repository templates.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D05-01: Hybrid placement can cause confusion about what is in-kernel versus userspace.
- RISK-D05-02: Repository independence can drift without compatibility tests.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 2 | Must be read before implementing this document. |
| Doc 3 | Must be read before implementing this document. |

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

## Expanded Repository Boundary Map
Version 0.2.0-draft expands the architecture from the original ten repositories to 32 top-level repositories. The purpose is to keep authority, compatibility, storage, filesystem, policy, identity, observability, developer tooling, corpus, model, package, and release concerns independently owned.

| Repository | Owner | Architectural responsibility |
|---|---|---|
| `alani-kernel` | Kernel team | Authority-bearing core for scheduler, memory, syscall dispatch, device mediation, capabilities, and audit handoff. |
| `alani-runtime` | Runtime team | Userspace runtime for process and agent lifecycle, manifest loading, syscall wrapping, and workload supervision. |
| `alani-lib` | Core library team | Shared safe wrappers, result types, error vocabulary, no_std utilities, and ergonomic APIs used by userspace and services. |
| `alani-cognition` | Cognition team | Cognitive execution service for model loading, inference requests, planning, memory retrieval, and accelerator integration. |
| `alani-memory` | Memory team | Knowledge, vector, cache, and persistent memory abstractions consumed by cognition and runtime services. |
| `alani-devices` | Device team | Hardware and virtual-device traits, registration types, polling abstractions, IRQ handoff, DMA descriptors, and device classes. |
| `alani-security` | Security team | Security primitives, authorization helpers, crypto provider traits, authentication plumbing, and sandbox mechanism boundaries. |
| `alani-audit` | Audit team | Append-only audit event APIs, tamper-evident metadata, durable sink traits, and forensic query surfaces. |
| `alani-terminal` | Terminal team | Interactive shell and CLI surface for inspection, service control, cognition experiments, and debugging. |
| `alani-userspace` | Userspace team | Example agents, user applications, and integration examples demonstrating runtime, syscall, cognition, and library use. |
| `alani-filesystem` | Storage and filesystem team | Virtual filesystem, path resolution, mount table, inode model, journaling hooks, and file capabilities. |
| `alani-boot` | Platform team | UEFI and emulator boot handoff logic, kernel image metadata, boot manifest parsing, and early console support. |
| `alani-platform` | Platform team | Architecture-specific HAL for x86_64, riscv64, timers, interrupt controllers, CPU features, and page-table primitives. |
| `alani-abi` | ABI owners | Canonical syscall numbers, repr(C) structs, error codes, handle types, ABI versioning, and feature flags. |
| `alani-protocol` | Interface owners | Shared wire and data schemas for IPC envelopes, audit events, device descriptors, config records, and corpus metadata. |
| `alani-ipc` | Kernel and runtime teams | Capability-aware message passing, ports, channels, event queues, shared-memory handles, and routing contracts. |
| `alani-storage` | Storage team | Block layer, partitions, volume management, buffer cache, storage-device adapters, and persistence primitives. |
| `alani-observability` | Observability team | Tracing, metrics, diagnostics, event schema, span context, redaction classification, and export adapters. |
| `alani-init` | Runtime team | First userspace process, boot profile execution, service supervision, restart policies, and lifecycle events. |
| `alani-config` | Platform and runtime teams | Typed configuration schemas for boot, devices, runtime, security, corpus, release, and environment profiles. |
| `alani-policy` | Security and policy teams | Declarative capability, access-control, sandbox, audit, and resource policies separated from enforcement mechanisms. |
| `alani-identity` | Security team | Principals, users, service identities, credentials, local identity store, session model, and attestation subjects. |
| `alani-network` | Networking team | Network stack boundary for loopback, sockets, packet buffers, protocol adapters, and future transport services. |
| `alani-sdk` | Developer experience team | Developer CLI, repository templates, sysroot management, code generation, local build helpers, and compatibility checks. |
| `alani-sim` | Simulation team | Host-side simulator, QEMU harness, fake devices, deterministic test world, and simulated boot flow. |
| `alani-tests` | QA team | Cross-repository integration, ABI conformance, syscall compatibility, security regression, and fixture tests. |
| `alani-benchmarks` | Performance team | Benchmark scenarios and reporting for scheduler, syscalls, memory, IPC, filesystem, cognition, and devices. |
| `alani-docs` | Documentation team | Source-of-truth specification docs, diagrams, ADRs, generated API references, lints, and publication pipeline. |
| `alani-corpus` | Data and cognition teams | Corpus schema, sample data, validators, generators, labeling taxonomy, split manifests, and datasheet tooling. |
| `alani-models` | Cognition team | Model manifests, adapters, compatibility tests, quantization metadata, and model-pack interface definitions. |
| `alani-release` | Release engineering team | Reproducible image builder, release manifest, SBOM, checksums, signing pipeline, artifact verification, and release evidence. |
| `alani-package` | Runtime and release teams | Package format, local registry, dependency resolver, installation transaction model, and package integrity checks. |

The dependency graph for this expanded set is maintained in `docs/assets/repo_dependency_graph.mmd` and normatively described in Doc 42 and Doc 63.
