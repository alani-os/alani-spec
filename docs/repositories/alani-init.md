# Repository Specification: alani-init

## Document Control

| Field | Value |
|---|---|
| Project | Alani |
| Repository | `alani-init` |
| Version | 0.2.0-draft |
| Generated | 2026-04-30T23:21:07Z |
| Tier | MVK required |
| Owner | Runtime team |
| Aliases | `alani-service-manager` |

## Purpose

First userspace process, boot profile execution, service supervision, restart policies, and
lifecycle events.

## Boundary Definition

`alani-init` is a top-level repository. It owns its own Cargo manifest, CI workflow, README,
license placeholder, source tree, tests, and release evidence. It MUST publish stable interfaces through
Rust crate APIs, ABI structs, protocol schemas, generated artifacts, release manifests, or documented CLI
contracts. It MUST NOT depend on another repository's private modules or undocumented file layout.

## Direct Architectural Dependencies

| Dependency | Contract |
|---|---|
| `alani-runtime` | Consume stable public APIs, schemas, generated artifacts, or release evidence only. |
| `alani-config` | Consume stable public APIs, schemas, generated artifacts, or release evidence only. |
| `alani-policy` | Consume stable public APIs, schemas, generated artifacts, or release evidence only. |
| `alani-observability` | Consume stable public APIs, schemas, generated artifacts, or release evidence only. |
| `alani-audit` | Consume stable public APIs, schemas, generated artifacts, or release evidence only. |

## Provided Artifacts

- A Rust 2021 crate template with CI, formatting, tests, and license placeholder.
- Public API names that can be stabilized through Doc 42 and Doc 43.
- Host-mode smoke tests before hardware-specific or boot-specific implementation.
- Security, audit, and observability notes for trust-boundary calls.


## Required Modules

- `src/service.rs` - owns the service API and implementation seam.
- `src/supervisor.rs` - owns the supervisor API and implementation seam.
- `src/boot_profile.rs` - owns the boot profile API and implementation seam.
- `src/restart.rs` - owns the restart API and implementation seam.


## Starter Layout

```text
alani-init/
├── Cargo.toml
├── README.md
├── LICENSE
├── rustfmt.toml
├── src/
│   ├── lib.rs
│   ├── service.rs
│   ├── supervisor.rs
│   ├── boot_profile.rs
│   ├── restart.rs
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
owner = "Runtime team"
architectural_dependencies = ["alani-runtime", "alani-config", "alani-policy", "alani-observability", "alani-audit"]
aliases = ["alani-service-manager"]
```

## Security, Audit, and Observability Requirements

- Security-sensitive operations MUST fail closed.
- Any operation that changes authority, identity, policy, persistent state, or release evidence MUST be auditable.
- Diagnostic data MUST classify fields as public, operational, sensitive, or secret.
- Trace context SHOULD be accepted or created for long-running and cross-repository operations.
- Unsafe Rust, external input parsing, cryptographic behavior, and persistence semantics REQUIRE explicit review.

## Implementation Milestones

1. Instantiate the template from `repo-templates/individual/alani-init`.
2. Keep the first public API minimal and documented.
3. Add host-mode tests before hardware, boot, device, or release-specific implementation.
4. Wire real path or published dependencies only after both repositories have stable public APIs.
5. Add cross-repository coverage in `alani-tests` before claiming compatibility.

## Acceptance Criteria

- `alani-init` builds as an independent Rust 2021 crate once a Rust toolchain is installed.
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
