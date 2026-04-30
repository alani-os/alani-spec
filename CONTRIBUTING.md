---

## 📄 CONTRIBUTING.md

```markdown
# Contributing to Alani

Thank you for your interest in contributing to Alani — a bare-metal intelligence operating system that treats reasoning as a kernel-managed primitive.

Alani is an **open specification**. Contributions span specification design, reference implementation, tooling, documentation, and research. This guide describes how to participate effectively.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Contribution Areas](#contribution-areas)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Specification Changes](#specification-changes)
- [Implementation Guidelines](#implementation-guidelines)
- [Commit Conventions](#commit-conventions)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)
- [Review Process](#review-process)
- [Licensing](#licensing)

---

## Code of Conduct

All participants in the Alani project are expected to conduct themselves professionally and respectfully. We are building critical systems infrastructure — precision, rigor, and constructive collaboration are valued above all.

- Be respectful and inclusive in all interactions.
- Provide constructive, technically grounded feedback.
- Assume good intent. Ask clarifying questions before making assumptions.
- No harassment, discrimination, or personal attacks of any kind.

Violations may result in removal from the project at the maintainers' discretion.

---

## How to Contribute

1. **Open an Issue** — Describe what you want to work on before writing code or spec changes. This prevents duplicate effort and ensures alignment.
2. **Fork the Repository** — Create a personal fork and work in a feature branch.
3. **Submit a Pull Request** — Reference the related issue. Follow the commit and PR conventions below.
4. **Participate in Review** — Respond to feedback and iterate.

---

## Contribution Areas

### Specification

The Alani specification defines the architecture, syscall interface, device model, memory hierarchy, security model, and scheduling semantics. Specification contributions include:

- **Syscall interface design** — Proposing new syscalls, modifying signatures, defining error codes, or refining semantics.
- **Device model extensions** — New cognitive device types, capability negotiation protocols, or driver interfaces.
- **Memory architecture** — Tier definitions, permission models, persistence semantics, or garbage collection strategies.
- **Security model** — Capability refinement, isolation boundaries, audit mechanisms.
- **Scheduling** — New scheduling dimensions, policy definitions, or fairness guarantees.

Specification changes require an **Alani Specification Proposal (ASP)** — see [Specification Changes](#specification-changes).

### Reference Implementation

The Rust-based Minimal Viable Kernel (MVK) is the reference implementation. Implementation contributions include:

- **Kernel subsystems** — Scheduler, memory manager, IPC, syscall dispatch.
- **Cognition subsystem** — `intent.rs`, `context.rs`, `planner.rs`, `router.rs`, `policy.rs`.
- **Device drivers** — Model device and memory device implementations.
- **Userspace** — Agent runtime, shell, tool integrations.
- **Boot and hardware** — Boot sequence, hardware abstraction, platform support.

### Tooling and Infrastructure

- **Build system** — Cargo configuration, cross-compilation, CI/CD pipelines.
- **Testing** — Unit tests, integration tests, syscall conformance tests.
- **Benchmarks** — Latency, throughput, and resource utilization measurements.
- **Tracing and debugging** — Trace log viewers, kernel debuggers, profiling tools.

### Documentation

- **Architecture guides** — Deep-dive explanations of subsystems.
- **Syscall reference** — Comprehensive documentation of every syscall.
- **Tutorials** — Getting started guides, agent development walkthroughs.
- **Research notes** — Formal analysis, comparisons with existing systems.

### Research

- **Formal verification** — Proving properties of the cognitive syscall protocol.
- **Scheduling theory** — Multi-dimensional scheduling algorithms for cognitive workloads.
- **Security analysis** — Threat models, capability system verification.
- **Benchmarking** — Comparative analysis with existing AI orchestration systems.

---

## Getting Started

### Prerequisites

- **Rust** (nightly toolchain) — The kernel targets `no_std` bare-metal environments.
- **QEMU** — For testing the bootable kernel without physical hardware.
- **cargo-make** or **just** — Task runner for build automation.
- **Git** — Version control.

### Clone and Build

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/<your-username>/alani.git
cd alani

# Install the nightly Rust toolchain
rustup toolchain install nightly
rustup override set nightly

# Add the bare-metal target
rustup target add x86_64-unknown-none

# Build the kernel
cd kernel
cargo build --target x86_64-unknown-none

# Run in QEMU (if configured)
cargo run --target x86_64-unknown-none
```

---

## Development Setup

### Repository Structure

```
alani/
├── kernel/                  # Kernel source
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── scheduler.rs     # Scheduling
│   │   ├── memory.rs        # Memory management
│   │   ├── ipc.rs           # IPC
│   │   ├── syscall.rs       # Syscall dispatch
│   │   └── cognition/       # Cognitive subsystem
│   └── Cargo.toml
├── devices/                 # Device drivers
├── userspace/               # Userspace runtime
├── spec/                    # Specification documents
├── docs/                    # Documentation
├── tests/                   # Integration tests
└── benches/                 # Benchmarks
```

### Running Tests

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Syscall conformance tests
cargo test --test syscall_conformance
```

---

## Specification Changes

Changes to the Alani specification follow the **Alani Specification Proposal (ASP)** process.

### ASP Process

1. **Draft** — Open an issue with the `asp` label. Include:
   - **Title** — A concise description of the change.
   - **Motivation** — Why this change is needed.
   - **Design** — Proposed specification text, syscall signatures, or architectural changes.
   - **Impact** — What existing components are affected.
   - **Alternatives** — Other approaches considered and why they were rejected.

2. **Discussion** — The community and maintainers review the proposal. Expect questions about backwards compatibility, security implications, and implementation feasibility.

3. **Revision** — Iterate on the proposal based on feedback.

4. **Acceptance** — A maintainer approves the ASP. The proposer (or an assignee) submits a PR with the specification changes.

5. **Merge** — The specification is updated. The ASP issue is closed.

### ASP Requirements

- Syscall changes must include complete signatures, error codes, and behavioral semantics.
- Device model changes must describe the capability negotiation protocol.
- Security changes must include a threat model analysis.
- All changes must be backwards-compatible unless a major version increment is justified.

---

## Implementation Guidelines

### Rust Conventions

- **`no_std`** — The kernel does not use the Rust standard library. All code must be `no_std` compatible.
- **`unsafe` minimization** — Use `unsafe` only when interfacing with hardware or performing operations that cannot be expressed safely. Every `unsafe` block must include a `// SAFETY:` comment explaining the invariant.
- **Error handling** — Use `Result<T, E>` throughout. No panics in kernel code.
- **Naming** — Follow Rust API naming conventions (RFC 430).
- **Documentation** — All public items must have `///` doc comments.

### Kernel Code Standards

- **No heap allocation in critical paths** — The scheduler and syscall dispatch must operate without dynamic allocation.
- **Deterministic behavior** — Kernel operations must be deterministic. Probabilistic behavior is confined to userspace model execution.
- **Traceability** — Every syscall must emit a trace event. New syscalls must include trace instrumentation.

### Testing Requirements

- **Unit tests** — Every module must have unit tests covering core logic.
- **Syscall tests** — Every new or modified syscall must have a conformance test.
- **Safety tests** — Memory isolation and permission enforcement must have dedicated test cases.

---

## Commit Conventions

Alani uses [Conventional Commits](https://www.conventionalcommits.org/) for all commit messages.

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature or capability |
| `fix` | Bug fix |
| `spec` | Specification change |
| `docs` | Documentation only |
| `refactor` | Code restructuring without behavior change |
| `test` | Adding or modifying tests |
| `perf` | Performance improvement |
| `ci` | CI/CD configuration |
| `chore` | Maintenance tasks |

### Scopes

| Scope | Description |
|-------|-------------|
| `kernel` | Core kernel code |
| `cognition` | Cognitive subsystem |
| `syscall` | Syscall interface |
| `scheduler` | Scheduler |
| `memory` | Memory management |
| `device` | Device drivers |
| `userspace` | Userspace runtime |
| `spec` | Specification documents |

### Examples

```
feat(syscall): add sys_model_capabilities for device introspection

spec(memory): define garbage collection semantics for trace memory

fix(scheduler): correct priority inversion in cognitive task queue

docs(syscall): document error codes for sys_intent_create
```

---

## Pull Request Process

1. **One concern per PR** — Each PR should address a single issue or feature.
2. **Reference the issue** — Include `Closes #<issue-number>` or `Relates to #<issue-number>` in the PR description.
3. **Describe the change** — Explain what changed, why, and how it was tested.
4. **Pass CI** — All tests must pass before review.
5. **Review** — At least one maintainer must approve the PR.
6. **Squash and merge** — PRs are squash-merged with a conventional commit message.

### PR Template

```markdown
## Summary

Brief description of the change.

## Motivation

Why is this change needed?

## Changes

- List of specific changes made.

## Testing

How was this tested?

## Related Issues

Closes #___
```

---

## Issue Guidelines

### Bug Reports

Include:
- **Description** — What happened vs. what was expected.
- **Reproduction steps** — Minimal steps to reproduce.
- **Environment** — Rust version, target platform, QEMU version.
- **Logs/traces** — Relevant trace output or error messages.

### Feature Requests

Include:
- **Description** — What capability is needed.
- **Motivation** — Why it matters for the Alani architecture.
- **Proposed approach** — How it could be implemented (optional).

### Specification Proposals

Use the ASP process described above.

---

## Review Process

- **Specification changes** are reviewed by at least two maintainers with a focus on architectural consistency, backwards compatibility, and security.
- **Kernel code** is reviewed for correctness, safety (`unsafe` usage), determinism, and performance.
- **Userspace code** is reviewed for API conformance, error handling, and documentation.
- **Documentation** is reviewed for accuracy and clarity.

Reviewers will use GitHub's review system. Address all comments before requesting re-review.

---

## Licensing

By contributing to Alani, you agree that your contributions will be licensed under the same terms as the project. See [LICENSE](./LICENSE) for details.

All specification text contributed through the ASP process is released as part of the open specification.

---

## Questions?

Open an issue with the `question` label, or start a discussion in the Discussions tab.

Thank you for helping build the future of intelligence-native operating systems.

---

<p align="center"><em>Intelligence is no longer an application feature — it is an operating system primitive.</em></p>
```