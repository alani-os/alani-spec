# Document 12: Scheduling & Multitasking

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-12 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Kernel developers |
| Depends on | Doc 6 |

## Purpose

Scheduling policies, tasks, priorities, and cognitive workloads.

Scheduling and multitasking define task representation, scheduling classes, priority policy, preemption, deadlines, blocking, and how cognitive workloads are budgeted.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Task states: New, Ready, Running, Blocked, Sleeping, Suspended, Exiting, Zombie.
- Initial policy: weighted round-robin with priority aging and deadline checks for real-time tasks.
- Cognitive priority: model requests carry budget and urgency; they should not bypass memory or device capabilities.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Implement cooperative yield first, then preemptive timer scheduling.
- Use scheduling classes: kernel, runtime, interactive, cognitive, background, audit.
- Represent model inference budgets separately from CPU priority.
- Record scheduling decisions in trace mode for replay and performance analysis.

## Normative Requirements

- ALANI-D12-REQ-01: Scheduler MUST not starve audit and security maintenance tasks.
- ALANI-D12-REQ-02: Cognitive work MUST have explicit deadlines or budget ceilings.
- ALANI-D12-REQ-03: Task state transitions MUST be auditable when they affect privileged resources.
- ALANI-D12-REQ-04: Preemption MUST preserve syscall atomicity rules.

## Design and Implementation Guidance

- Task states: New, Ready, Running, Blocked, Sleeping, Suspended, Exiting, Zombie.
- Initial policy: weighted round-robin with priority aging and deadline checks for real-time tasks.
- Cognitive priority: model requests carry budget and urgency; they should not bypass memory or device capabilities.
- Blocking: tasks block on wait queues for timer, device, IPC, memory, and audit flush events.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- TaskControlBlock.
- SchedulerClass.
- RunQueue.
- WaitQueue.
- BudgetAccount.
- ContextSwitchEvent.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D12-AC-01: Scheduler simulation demonstrates no starvation under synthetic loads.
- ALANI-D12-AC-02: Context-switch counters and latency histograms are exported to trace.
- ALANI-D12-AC-03: Cancellation of cognitive task releases handles and emits audit.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D12-01: Deadline semantics can be overpromised before hardware timer validation.
- RISK-D12-02: Priority inversion requires explicit mitigation once locks and drivers mature.

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

