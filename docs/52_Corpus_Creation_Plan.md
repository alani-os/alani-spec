# Document 52: Corpus Creation Plan

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Document ID | ALANI-SPEC-52 |
| Status | Draft implementation specification |
| Version | 0.1.0-draft |
| Generated | 2026-04-30T21:27:04Z |
| Audience | Data scientists |
| Depends on | Doc 45, Doc 50 |

## Purpose

Full lifecycle plan for building the Alani corpus.

Corpus creation plan defines lifecycle from source selection through generation, annotation, validation, splitting, and evaluation.

## Scope

This document is normative for the Alani implementation unless a later approved Architecture Decision Record updates it. It covers the behaviors, interfaces, constraints, and acceptance criteria needed by Rust developers to create and maintain the relevant repositories. It does not replace detailed code review, threat review, hardware validation, or legal review.

In scope:

- Implementation guidance is expressed as checklists, file layouts, command examples, and acceptance gates.
- Where automation exists in this bundle, the document names the script path and expected output.
- Process documents should be updated with the same review standard as code because they shape production behavior.

Out of scope:

- Production certification, safety certification, and third-party compliance attestation.
- Undocumented interface changes outside the version and review process.
- Claims of production readiness before MVK acceptance criteria pass.

## Key Decisions

- Use written policy as the source of truth, not tribal knowledge.
- Make automated checks enforce as many rules as practical.
- Keep exceptions explicit, documented, time-bound, and auditable.
- Treat documentation, corpus, and repository templates as release artifacts.

## Normative Requirements

- ALANI-D52-REQ-01: The document MUST cover: source intake, synthetic generation, schema, labels, QA.
- ALANI-D52-REQ-02: Procedures MUST be reproducible from a clean checkout or clean artifact bundle.
- ALANI-D52-REQ-03: Inputs, outputs, owners, and approval gates MUST be named.
- ALANI-D52-REQ-04: Security and audit implications MUST be considered before release.

## Design and Implementation Guidance

- Implementation guidance is expressed as checklists, file layouts, command examples, and acceptance gates.
- Where automation exists in this bundle, the document names the script path and expected output.
- Process documents should be updated with the same review standard as code because they shape production behavior.
- Every process with release or security impact should emit or preserve evidence: logs, checksums, PR links, review records, or audit events.

### Repository Impact

See Doc 42 for repository ownership.

### Developer Workflow

1. Read this document and all dependencies listed in the traceability table.
2. Create or update the relevant issue with linked requirement IDs.
3. Implement the change behind the smallest stable interface.
4. Add or update unit tests, integration tests, corpus fixtures, and documentation examples.
5. Run formatting, linting, tests, corpus validation where applicable, and dependency audit before opening a pull request.

## Interfaces and Data Contracts

- tools/init_repos.sh
- tools/build_release.sh
- tools/check_bundle.py
- corpus/scripts/*
- repo-templates/individual/*

Interface rules:

- Public interfaces are versioned and must preserve compatibility inside a major version unless explicitly marked experimental.
- Any interface that crosses a trust boundary must validate length, type, ownership, and authority before use.
- Any interface that emits operational or security-relevant behavior must propagate TraceContext when available.
- Any interface that processes sensitive content must define redaction behavior.

Corpus deliverables included in this bundle:

| Path | Purpose |
|---|---|
| corpus/data/train.jsonl | Synthetic training split |
| corpus/data/validation.jsonl | Synthetic validation split |
| corpus/data/test.jsonl | Synthetic test split |
| corpus/schema/alani_corpus.schema.json | Machine-readable record schema |
| corpus/taxonomy/labels.yaml | Label taxonomy and definitions |
| corpus/scripts/generate_synthetic_corpus.py | Deterministic generator |
| corpus/scripts/validate_corpus.py | Standard-library validator |

## Acceptance Criteria

- ALANI-D52-AC-01: A contributor can perform the procedure without private context.
- ALANI-D52-AC-02: The document names failure modes and how to recover.
- ALANI-D52-AC-03: The procedure can be mapped to CI or release evidence.

## Test and Verification Strategy

- Unit tests cover pure functions, error mapping, and edge cases.
- Integration tests cover cross-repository behavior through the public APIs described here.
- Negative tests cover authorization failure, invalid data, boundary sizes, and malformed input.
- Host-mode mocks are acceptable for MVK, but hardware-specific behavior must be marked as simulated.
- Audit or trace evidence should be captured for operations that affect security, persistence, or release artifacts.

## Risks, Constraints, and Open Questions

- RISK-D52-01: Manual process steps can diverge from automation.
- RISK-D52-02: Legal and data-source assumptions require human review before external publication.

Open questions:

- Which target hardware profile will be promoted from simulation to required MVK boot target?
- Which cryptographic provider will be selected after legal and security review?
- Which future features require formal RFCs before implementation?

## Traceability

| Dependency | Reason |
|---|---|
| Doc 45 | Must be read before implementing this document. |
| Doc 50 | Must be read before implementing this document. |

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

