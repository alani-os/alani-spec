# Document 63: Repository Expansion Addendum

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-63 |
| Status | Draft implementation specification |
| Version | 0.2.0-draft |
| Generated | 2026-04-30T23:21:07Z |
| Audience | Architects, Rust developers, repository owners, DevOps |
| Depends on | Docs 01-05, 06-18, 42, 43, 45, 50, 51 |

## Purpose

This addendum updates the repository portion of the Alani specification. The source specification defined ten top-level repositories and a 62-document plan. This addendum preserves those documents and adds the missing top-level boundaries needed for boot, platform, ABI, protocol, IPC, storage, filesystem, observability, init, configuration, policy, identity, networking, SDK, simulation, conformance, benchmarks, documentation, corpus, models, release, and packaging.

## Normative Repository Rule

Each top-level repository MUST have a repository specification, a starter Rust repository template, CI workflow, README, Cargo manifest, license placeholder, rustfmt configuration, smoke test, owner, dependency boundary, and acceptance criteria.

## Canonical Repository Set

| # | Repository | Tier | Owner | Purpose | Direct architectural dependencies | Aliases |
|---:|---|---|---|---|---|---|
| 1 | `alani-kernel` | MVK required | Kernel team | Authority-bearing core for scheduler, memory, syscall dispatch, device mediation, capabilities, and audit handoff. | `alani-abi`, `alani-platform`, `alani-ipc`, `alani-storage`, `alani-filesystem`, `alani-devices`, `alani-security`, `alani-audit`, `alani-observability`, `alani-policy` | None |
| 2 | `alani-runtime` | MVK required | Runtime team | Userspace runtime for process and agent lifecycle, manifest loading, syscall wrapping, and workload supervision. | `alani-lib`, `alani-abi`, `alani-protocol`, `alani-ipc`, `alani-config`, `alani-security`, `alani-audit`, `alani-observability` | None |
| 3 | `alani-lib` | MVK required | Core library team | Shared safe wrappers, result types, error vocabulary, no_std utilities, and ergonomic APIs used by userspace and services. | `alani-abi` | None |
| 4 | `alani-cognition` | MVK required | Cognition team | Cognitive execution service for model loading, inference requests, planning, memory retrieval, and accelerator integration. | `alani-lib`, `alani-protocol`, `alani-memory`, `alani-models`, `alani-devices`, `alani-security`, `alani-observability` | None |
| 5 | `alani-memory` | MVK required | Memory team | Knowledge, vector, cache, and persistent memory abstractions consumed by cognition and runtime services. | `alani-lib`, `alani-abi`, `alani-storage`, `alani-observability` | None |
| 6 | `alani-devices` | MVK required | Device team | Hardware and virtual-device traits, registration types, polling abstractions, IRQ handoff, DMA descriptors, and device classes. | `alani-lib`, `alani-abi`, `alani-platform`, `alani-observability` | None |
| 7 | `alani-security` | MVK required | Security team | Security primitives, authorization helpers, crypto provider traits, authentication plumbing, and sandbox mechanism boundaries. | `alani-lib`, `alani-abi`, `alani-policy`, `alani-identity` | None |
| 8 | `alani-audit` | MVK required | Audit team | Append-only audit event APIs, tamper-evident metadata, durable sink traits, and forensic query surfaces. | `alani-lib`, `alani-protocol`, `alani-observability`, `alani-storage` | None |
| 9 | `alani-terminal` | MVK required | Terminal team | Interactive shell and CLI surface for inspection, service control, cognition experiments, and debugging. | `alani-runtime`, `alani-lib`, `alani-protocol`, `alani-observability` | None |
| 10 | `alani-userspace` | MVK required | Userspace team | Example agents, user applications, and integration examples demonstrating runtime, syscall, cognition, and library use. | `alani-runtime`, `alani-lib`, `alani-cognition`, `alani-protocol` | None |
| 11 | `alani-filesystem` | MVK required | Storage and filesystem team | Virtual filesystem, path resolution, mount table, inode model, journaling hooks, and file capabilities. | `alani-abi`, `alani-protocol`, `alani-storage`, `alani-policy`, `alani-audit`, `alani-observability` | None |
| 12 | `alani-boot` | MVK required | Platform team | UEFI and emulator boot handoff logic, kernel image metadata, boot manifest parsing, and early console support. | `alani-abi`, `alani-platform`, `alani-config` | None |
| 13 | `alani-platform` | MVK required | Platform team | Architecture-specific HAL for x86_64, riscv64, timers, interrupt controllers, CPU features, and page-table primitives. | `alani-abi` | `alani-arch` |
| 14 | `alani-abi` | MVK required | ABI owners | Canonical syscall numbers, repr(C) structs, error codes, handle types, ABI versioning, and feature flags. | None | None |
| 15 | `alani-protocol` | MVK required | Interface owners | Shared wire and data schemas for IPC envelopes, audit events, device descriptors, config records, and corpus metadata. | `alani-abi` | None |
| 16 | `alani-ipc` | MVK required | Kernel and runtime teams | Capability-aware message passing, ports, channels, event queues, shared-memory handles, and routing contracts. | `alani-abi`, `alani-protocol`, `alani-policy`, `alani-observability` | None |
| 17 | `alani-storage` | MVK required | Storage team | Block layer, partitions, volume management, buffer cache, storage-device adapters, and persistence primitives. | `alani-abi`, `alani-devices`, `alani-observability` | None |
| 18 | `alani-observability` | MVK required | Observability team | Tracing, metrics, diagnostics, event schema, span context, redaction classification, and export adapters. | `alani-abi`, `alani-protocol` | None |
| 19 | `alani-init` | MVK required | Runtime team | First userspace process, boot profile execution, service supervision, restart policies, and lifecycle events. | `alani-runtime`, `alani-config`, `alani-policy`, `alani-observability`, `alani-audit` | `alani-service-manager` |
| 20 | `alani-config` | MVK required | Platform and runtime teams | Typed configuration schemas for boot, devices, runtime, security, corpus, release, and environment profiles. | `alani-protocol` | None |
| 21 | `alani-policy` | MVK required | Security and policy teams | Declarative capability, access-control, sandbox, audit, and resource policies separated from enforcement mechanisms. | `alani-abi`, `alani-protocol`, `alani-config` | None |
| 22 | `alani-identity` | MVK required | Security team | Principals, users, service identities, credentials, local identity store, session model, and attestation subjects. | `alani-abi`, `alani-protocol`, `alani-security`, `alani-audit` | None |
| 23 | `alani-network` | Post-MVK boundary | Networking team | Network stack boundary for loopback, sockets, packet buffers, protocol adapters, and future transport services. | `alani-abi`, `alani-devices`, `alani-protocol`, `alani-policy`, `alani-observability` | None |
| 24 | `alani-sdk` | MVK required | Developer experience team | Developer CLI, repository templates, sysroot management, code generation, local build helpers, and compatibility checks. | `alani-config`, `alani-protocol`, `alani-docs` | None |
| 25 | `alani-sim` | MVK required | Simulation team | Host-side simulator, QEMU harness, fake devices, deterministic test world, and simulated boot flow. | `alani-platform`, `alani-devices`, `alani-storage`, `alani-network`, `alani-observability`, `alani-boot` | `alani-emulator` |
| 26 | `alani-tests` | MVK required | QA team | Cross-repository integration, ABI conformance, syscall compatibility, security regression, and fixture tests. | `alani-abi`, `alani-protocol`, `alani-ipc`, `alani-filesystem`, `alani-sim`, `alani-corpus`, `alani-policy` | `alani-conformance` |
| 27 | `alani-benchmarks` | MVK required | Performance team | Benchmark scenarios and reporting for scheduler, syscalls, memory, IPC, filesystem, cognition, and devices. | `alani-abi`, `alani-ipc`, `alani-storage`, `alani-filesystem`, `alani-cognition`, `alani-memory`, `alani-sim` | None |
| 28 | `alani-docs` | MVK required | Documentation team | Source-of-truth specification docs, diagrams, ADRs, generated API references, lints, and publication pipeline. | `alani-config` | `alani-spec` |
| 29 | `alani-corpus` | MVK required | Data and cognition teams | Corpus schema, sample data, validators, generators, labeling taxonomy, split manifests, and datasheet tooling. | `alani-protocol`, `alani-config`, `alani-policy` | None |
| 30 | `alani-models` | Post-MVK boundary | Cognition team | Model manifests, adapters, compatibility tests, quantization metadata, and model-pack interface definitions. | `alani-protocol`, `alani-cognition`, `alani-corpus`, `alani-policy` | None |
| 31 | `alani-release` | MVK required | Release engineering team | Reproducible image builder, release manifest, SBOM, checksums, signing pipeline, artifact verification, and release evidence. | `alani-package`, `alani-sdk`, `alani-docs`, `alani-corpus`, `alani-config` | `alani-image` |
| 32 | `alani-package` | Post-MVK boundary | Runtime and release teams | Package format, local registry, dependency resolver, installation transaction model, and package integrity checks. | `alani-abi`, `alani-protocol`, `alani-config`, `alani-policy`, `alani-filesystem`, `alani-identity` | None |

## MVK Required Repositories

- `alani-kernel`
- `alani-runtime`
- `alani-lib`
- `alani-cognition`
- `alani-memory`
- `alani-devices`
- `alani-security`
- `alani-audit`
- `alani-terminal`
- `alani-userspace`
- `alani-filesystem`
- `alani-boot`
- `alani-platform`
- `alani-abi`
- `alani-protocol`
- `alani-ipc`
- `alani-storage`
- `alani-observability`
- `alani-init`
- `alani-config`
- `alani-policy`
- `alani-identity`
- `alani-sdk`
- `alani-sim`
- `alani-tests`
- `alani-benchmarks`
- `alani-docs`
- `alani-corpus`
- `alani-release`


## Post-MVK or Scope-Dependent Boundaries

- `alani-network`
- `alani-models`
- `alani-package`


## Boundary Decisions

- `alani-abi` owns binary compatibility; `alani-lib` owns ergonomic safe wrappers.
- `alani-protocol` owns wire and data schemas; feature crates consume schemas rather than re-declaring them.
- `alani-storage` owns block and volume abstractions; `alani-filesystem` owns VFS, paths, inodes, mounts, and journals.
- `alani-policy` owns declarative rules; `alani-security` owns enforcement-facing primitives and cryptographic provider traits.
- `alani-identity` owns principals, credentials, and sessions.
- `alani-observability` owns traces, metrics, and diagnostics; `alani-audit` owns durable security evidence.
- `alani-release` owns release evidence; `alani-package` owns installable package artifacts.
- `alani-tests` and `alani-benchmarks` are top-level artifacts because cross-repo correctness and performance baselines are product deliverables.

## Implementation Sequence

1. Implement `alani-abi`, `alani-protocol`, `alani-config`, `alani-observability`, and `alani-policy` first.
2. Implement `alani-platform`, `alani-boot`, `alani-ipc`, `alani-storage`, and `alani-filesystem` next.
3. Integrate `alani-runtime`, `alani-init`, `alani-security`, `alani-identity`, and `alani-audit`.
4. Layer cognition, corpus, models, terminal, userspace, simulation, conformance, benchmarks, SDK, package, and release automation.
5. Promote Post-MVK repositories from stubs only after MVK interfaces have conformance coverage.

## Acceptance Criteria

- ALANI-D63-AC-01: `tools/check_bundle.py` reports 62 baseline docs, this addendum, and all 32 repository templates/specs.
- ALANI-D63-AC-02: The integration workspace lists every top-level repository.
- ALANI-D63-AC-03: Every new repository has a crate boundary and repository-specific specification.
- ALANI-D63-AC-04: The corpus remains present and valid after repository expansion.
- ALANI-D63-AC-05: Release packaging includes docs, corpus, templates, tooling, and repository expansion metadata.
