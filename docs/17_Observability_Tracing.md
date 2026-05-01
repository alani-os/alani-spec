# Document 17: Observability & Tracing

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-17 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Developers, operators, system administrators |
| Depends on | Doc 6, Doc 7, Doc 16 |

## Purpose

Logging, telemetry, metrics, tracing formats, and monitoring interfaces.

Observability and tracing define operational visibility into kernel, runtime, devices, cognition, security, and corpus processing. The goal is diagnosability without compromising safety or privacy.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Event schema includes timestamp, monotonic counter, component, severity, trace_id, span_id, principal, resource, operation, status, and redaction metadata.
- Metrics include boot phase duration, syscall counts/errors, scheduler latency, memory pressure, device errors, inference latency, corpus validation errors.
- Trace backends: early serial logs, in-memory ring buffer, audit sink bridge, host test JSONL export.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Use structured events, not free-form logs, for machine-consumed telemetry.
- Propagate TraceContext across syscalls and service calls.
- Separate debug logs, operational metrics, audit events, and evaluation records.
- Default to redaction for sensitive payloads.

## Normative Requirements

- ALANI-D17-REQ-01: Every subsystem MUST define event types and severity levels.
- ALANI-D17-REQ-02: Trace identifiers MUST be stable across component boundaries.
- ALANI-D17-REQ-03: Metrics MUST have units and cardinality controls.
- ALANI-D17-REQ-04: Sensitive fields MUST be classified and redacted by policy before export.

## Design and Implementation Guidance

- Event schema includes timestamp, monotonic counter, component, severity, trace_id, span_id, principal, resource, operation, status, and redaction metadata.
- Metrics include boot phase duration, syscall counts/errors, scheduler latency, memory pressure, device errors, inference latency, corpus validation errors.
- Trace backends: early serial logs, in-memory ring buffer, audit sink bridge, host test JSONL export.
- Sampling: high-frequency events can be sampled, but security denials and audit-critical events are never sampled out.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- TraceContext.
- EventEnvelope.
- MetricSample.
- LogLevel.
- RedactionPolicy.

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.



## Acceptance Criteria

- ALANI-D17-AC-01: A terminal command can display recent trace events.
- ALANI-D17-AC-02: Kernel and runtime events share trace IDs for a sample request.
- ALANI-D17-AC-03: CI validates event JSON schema examples.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D17-01: Too much logging can leak secrets or affect timing.
- RISK-D17-02: High-cardinality labels can overwhelm telemetry storage.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 6 | Must be read before implementing this document. |
| Doc 7 | Must be read before implementing this document. |
| Doc 16 | Must be read before implementing this document. |

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

