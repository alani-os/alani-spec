# Repository Specification: alani-audit

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Repository | `alani-audit` |
| Version | 0.2.0-draft |
| Generated | 2026-04-30T23:21:07Z |
| Tier | MVK required |
| Owner | Audit team |
| Aliases | None |

## Purpose

Append-only audit event APIs, tamper-evident metadata, durable sink traits, and forensic query
surfaces.

## Boundary Definition

`alani-audit` is a top-level repository. It owns its own Cargo manifest, CI workflow, README,
license placeholder, source tree, tests, and release evidence. It MUST publish stable interfaces through
Rust crate APIs, ABI structs, protocol schemas, generated artifacts, release manifests, or documented CLI
contracts. It MUST NOT depend on another repository's private modules or undocumented file layout.

## Direct Architectural Dependencies

| Dependency | Contract |
|---|---|
| `alani-lib` | Consume stable public APIs, schemas, generated artifacts, or release evidence only. |
| `alani-protocol` | Consume stable public APIs, schemas, generated artifacts, or release evidence only. |
| `alani-observability` | Consume stable public APIs, schemas, generated artifacts, or release evidence only. |
| `alani-storage` | Consume stable public APIs, schemas, generated artifacts, or release evidence only. |

## Provided Artifacts

- A Rust 2021 crate template with CI, formatting, tests, and license placeholder.
- Public API names that can be stabilized through Doc 42 and Doc 43.
- Host-mode smoke tests before hardware-specific or boot-specific implementation.
- Security, audit, and observability notes for trust-boundary calls.


## Required Modules

- `src/event.rs` - owns the event API and implementation seam.
- `src/sink.rs` - owns the sink API and implementation seam.
- `src/hash_chain.rs` - owns the hash chain API and implementation seam.
- `src/query.rs` - owns the query API and implementation seam.


## Starter Layout

```text
alani-audit/
├── Cargo.toml
├── README.md
├── LICENSE
├── rustfmt.toml
├── src/
│   ├── lib.rs
│   ├── event.rs
│   ├── sink.rs
│   ├── hash_chain.rs
│   ├── query.rs
├── tests/
│   └── smoke.rs
└── .github/
    └── workflows/
        └── ci.yml
```

## Cargo Metadata Contract

The starter template keeps path dependencies out of `[dependencies]` so each repository can be created
independently. Architectural dependencies are recorded as metadata until the repository owners intentionally
wire real crate dependencies.

```toml
[package.metadata.alani]
tier = "MVK required"
owner = "Audit team"
architectural_dependencies = ["alani-lib", "alani-protocol", "alani-observability", "alani-storage"]
aliases = []
```

## Security, Audit, and Observability Requirements

- Security-sensitive operations MUST fail closed.
- Any operation that changes authority, identity, policy, persistent state, or release evidence MUST be auditable.
- Diagnostic data MUST classify fields as public, operational, sensitive, or secret.
- Trace context SHOULD be accepted or created for long-running and cross-repository operations.
- Unsafe Rust, external input parsing, cryptographic behavior, and persistence semantics REQUIRE explicit review.

## Implementation Milestones

1. Instantiate the template from `repo-templates/individual/alani-audit`.
2. Keep the first public API minimal and documented.
3. Add host-mode tests before hardware, boot, device, or release-specific implementation.
4. Wire real path or published dependencies only after both repositories have stable public APIs.
5. Add cross-repository coverage in `alani-tests` before claiming compatibility.

## Acceptance Criteria

- `alani-audit` builds as an independent Rust 2021 crate once a Rust toolchain is installed.
- The README names the purpose, owner, tier, aliases, and architectural dependencies.
- No module reaches through another repository private implementation path.
- At least one negative or smoke test is present before functional expansion.
- Public interfaces are documented before another repository consumes them.


## Related Specification Documents

- Doc 05 - Architecture Overview
- Doc 42 - Inter-Repo Interfaces
- Doc 43 - APIs & Data Formats
- Doc 45 - Testing Strategy
- Doc 50 - Continuous Integration
- Doc 51 - Release Packaging
- Doc 63 - Repository Expansion Addendum
