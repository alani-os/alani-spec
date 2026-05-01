# Alani Specification Index

Generated: 2026-04-30T23:21:07Z  
Version: 0.2.0-draft

This bundle expands the attached Alani Executive Summary into implementation-ready specifications. Version 0.2.0-draft preserves the original 62 numbered documents and adds an expansion addendum plus per-repository specifications for all top-level repositories.

## Numbered documents

| # | Title | File |
|---:|---|---|
| 1 | Alani Overview | `docs/01_Alani_Overview.md` |
| 2 | System Model | `docs/02_System_Model.md` |
| 3 | Design Principles | `docs/03_Design_Principles.md` |
| 4 | Use Cases Requirements | `docs/04_Use_Cases_Requirements.md` |
| 5 | Architecture Overview | `docs/05_Architecture_Overview.md` |
| 6 | Kernel Architecture | `docs/06_Kernel_Architecture.md` |
| 7 | Cognitive Subsystem | `docs/07_Cognitive_Subsystem.md` |
| 8 | Syscall Interface | `docs/08_Syscall_Interface.md` |
| 9 | Syscall Reference | `docs/09_Syscall_Reference.md` |
| 10 | Kernel ABI Rust | `docs/10_Kernel_ABI_Rust.md` |
| 11 | Memory Management | `docs/11_Memory_Management.md` |
| 12 | Scheduling Multitasking | `docs/12_Scheduling_Multitasking.md` |
| 13 | Device Model | `docs/13_Device_Model.md` |
| 14 | Cognitive Devices | `docs/14_Cognitive_Devices.md` |
| 15 | Security Model | `docs/15_Security_Model.md` |
| 16 | Security Mechanisms | `docs/16_Security_Mechanisms.md` |
| 17 | Observability Tracing | `docs/17_Observability_Tracing.md` |
| 18 | Auditing Forensics | `docs/18_Auditing_Forensics.md` |
| 19 | Minimal Viable Kernel MVK | `docs/19_Minimal_Viable_Kernel_MVK.md` |
| 20 | Differentiation | `docs/20_Differentiation.md` |
| 21 | Future Work | `docs/21_Future_Work.md` |
| 22 | Glossary Acronyms | `docs/22_Glossary_Acronyms.md` |
| 23 | alani kernel README | `docs/23_alani_kernel_README.md` |
| 24 | alani kernel Design Doc | `docs/24_alani_kernel_Design_Doc.md` |
| 25 | alani runtime README | `docs/25_alani_runtime_README.md` |
| 26 | alani runtime Design | `docs/26_alani_runtime_Design.md` |
| 27 | alani lib README | `docs/27_alani_lib_README.md` |
| 28 | alani lib API | `docs/28_alani_lib_API.md` |
| 29 | alani cognition README | `docs/29_alani_cognition_README.md` |
| 30 | alani cognition Design | `docs/30_alani_cognition_Design.md` |
| 31 | alani memory README | `docs/31_alani_memory_README.md` |
| 32 | alani memory API | `docs/32_alani_memory_API.md` |
| 33 | alani devices README | `docs/33_alani_devices_README.md` |
| 34 | alani devices API | `docs/34_alani_devices_API.md` |
| 35 | alani security README | `docs/35_alani_security_README.md` |
| 36 | alani security API | `docs/36_alani_security_API.md` |
| 37 | alani audit README | `docs/37_alani_audit_README.md` |
| 38 | alani audit API | `docs/38_alani_audit_API.md` |
| 39 | alani terminal README | `docs/39_alani_terminal_README.md` |
| 40 | alani terminal API | `docs/40_alani_terminal_API.md` |
| 41 | alani userspace README | `docs/41_alani_userspace_README.md` |
| 42 | Inter Repo Interfaces | `docs/42_Inter_Repo_Interfaces.md` |
| 43 | APIs Data Formats | `docs/43_APIs_Data_Formats.md` |
| 44 | Performance Benchmarking | `docs/44_Performance_Benchmarking.md` |
| 45 | Testing Strategy | `docs/45_Testing_Strategy.md` |
| 46 | Quality Assurance | `docs/46_Quality_Assurance.md` |
| 47 | Development Workflow | `docs/47_Development_Workflow.md` |
| 48 | Branching Releases | `docs/48_Branching_Releases.md` |
| 49 | Code Review Guidelines | `docs/49_Code_Review_Guidelines.md` |
| 50 | Continuous Integration | `docs/50_Continuous_Integration.md` |
| 51 | Release Packaging | `docs/51_Release_Packaging.md` |
| 52 | Corpus Creation Plan | `docs/52_Corpus_Creation_Plan.md` |
| 53 | Data Sources | `docs/53_Data_Sources.md` |
| 54 | Data Schema Formats | `docs/54_Data_Schema_Formats.md` |
| 55 | Labeling Guidelines | `docs/55_Labeling_Guidelines.md` |
| 56 | Storage Access Control | `docs/56_Storage_Access_Control.md` |
| 57 | Sample Data | `docs/57_Sample_Data.md` |
| 58 | Data Generation Scripts | `docs/58_Data_Generation_Scripts.md` |
| 59 | Doc Templates | `docs/59_Doc_Templates.md` |
| 60 | Code Style Guide | `docs/60_Code_Style_Guide.md` |
| 61 | Contribution Guidelines | `docs/61_Contribution_Guidelines.md` |
| 62 | License Legal | `docs/62_License_Legal.md` |
| 63 | Repository Expansion Addendum | `docs/63_Repository_Expansion_Addendum.md` |

## Top-level repositories

Total top-level repositories: 32

Newly added or newly explicit repositories: `alani-filesystem`, `alani-boot`, `alani-platform`, `alani-abi`, `alani-protocol`, `alani-ipc`, `alani-storage`, `alani-observability`, `alani-init`, `alani-config`, `alani-policy`, `alani-identity`, `alani-network`, `alani-sdk`, `alani-sim`, `alani-tests`, `alani-benchmarks`, `alani-docs`, `alani-corpus`, `alani-models`, `alani-release`, `alani-package`.

| Repository | Tier | Owner | Direct architectural dependencies | Aliases |
|---|---|---|---|---|
| `alani-kernel` | MVK required | Kernel team | `alani-abi`, `alani-platform`, `alani-ipc`, `alani-storage`, `alani-filesystem`, `alani-devices`, `alani-security`, `alani-audit`, `alani-observability`, `alani-policy` | None |
| `alani-runtime` | MVK required | Runtime team | `alani-lib`, `alani-abi`, `alani-protocol`, `alani-ipc`, `alani-config`, `alani-security`, `alani-audit`, `alani-observability` | None |
| `alani-lib` | MVK required | Core library team | `alani-abi` | None |
| `alani-cognition` | MVK required | Cognition team | `alani-lib`, `alani-protocol`, `alani-memory`, `alani-models`, `alani-devices`, `alani-security`, `alani-observability` | None |
| `alani-memory` | MVK required | Memory team | `alani-lib`, `alani-abi`, `alani-storage`, `alani-observability` | None |
| `alani-devices` | MVK required | Device team | `alani-lib`, `alani-abi`, `alani-platform`, `alani-observability` | None |
| `alani-security` | MVK required | Security team | `alani-lib`, `alani-abi`, `alani-policy`, `alani-identity` | None |
| `alani-audit` | MVK required | Audit team | `alani-lib`, `alani-protocol`, `alani-observability`, `alani-storage` | None |
| `alani-terminal` | MVK required | Terminal team | `alani-runtime`, `alani-lib`, `alani-protocol`, `alani-observability` | None |
| `alani-userspace` | MVK required | Userspace team | `alani-runtime`, `alani-lib`, `alani-cognition`, `alani-protocol` | None |
| `alani-filesystem` | MVK required | Storage and filesystem team | `alani-abi`, `alani-protocol`, `alani-storage`, `alani-policy`, `alani-audit`, `alani-observability` | None |
| `alani-boot` | MVK required | Platform team | `alani-abi`, `alani-platform`, `alani-config` | None |
| `alani-platform` | MVK required | Platform team | `alani-abi` | `alani-arch` |
| `alani-abi` | MVK required | ABI owners | None | None |
| `alani-protocol` | MVK required | Interface owners | `alani-abi` | None |
| `alani-ipc` | MVK required | Kernel and runtime teams | `alani-abi`, `alani-protocol`, `alani-policy`, `alani-observability` | None |
| `alani-storage` | MVK required | Storage team | `alani-abi`, `alani-devices`, `alani-observability` | None |
| `alani-observability` | MVK required | Observability team | `alani-abi`, `alani-protocol` | None |
| `alani-init` | MVK required | Runtime team | `alani-runtime`, `alani-config`, `alani-policy`, `alani-observability`, `alani-audit` | `alani-service-manager` |
| `alani-config` | MVK required | Platform and runtime teams | `alani-protocol` | None |
| `alani-policy` | MVK required | Security and policy teams | `alani-abi`, `alani-protocol`, `alani-config` | None |
| `alani-identity` | MVK required | Security team | `alani-abi`, `alani-protocol`, `alani-security`, `alani-audit` | None |
| `alani-network` | Post-MVK boundary | Networking team | `alani-abi`, `alani-devices`, `alani-protocol`, `alani-policy`, `alani-observability` | None |
| `alani-sdk` | MVK required | Developer experience team | `alani-config`, `alani-protocol`, `alani-docs` | None |
| `alani-sim` | MVK required | Simulation team | `alani-platform`, `alani-devices`, `alani-storage`, `alani-network`, `alani-observability`, `alani-boot` | `alani-emulator` |
| `alani-tests` | MVK required | QA team | `alani-abi`, `alani-protocol`, `alani-ipc`, `alani-filesystem`, `alani-sim`, `alani-corpus`, `alani-policy` | `alani-conformance` |
| `alani-benchmarks` | MVK required | Performance team | `alani-abi`, `alani-ipc`, `alani-storage`, `alani-filesystem`, `alani-cognition`, `alani-memory`, `alani-sim` | None |
| `alani-docs` | MVK required | Documentation team | `alani-config` | `alani-spec` |
| `alani-corpus` | MVK required | Data and cognition teams | `alani-protocol`, `alani-config`, `alani-policy` | None |
| `alani-models` | Post-MVK boundary | Cognition team | `alani-protocol`, `alani-cognition`, `alani-corpus`, `alani-policy` | None |
| `alani-release` | MVK required | Release engineering team | `alani-package`, `alani-sdk`, `alani-docs`, `alani-corpus`, `alani-config` | `alani-image` |
| `alani-package` | Post-MVK boundary | Runtime and release teams | `alani-abi`, `alani-protocol`, `alani-config`, `alani-policy`, `alani-filesystem`, `alani-identity` | None |

## Bundle structure

- `docs/` contains the original 62 implementation specifications plus Doc 63.
- `docs/repositories/` contains one repository specification for every top-level repo.
- `repo-templates/individual/` contains starter Rust repositories.
- `repo-templates/alani-workspace/` contains an integration workspace template referencing all repositories.
- `corpus/` contains the synthetic corpus, schema, taxonomy, datasheet, generator, and validator.
- `tools/` contains repo initialization, bundle validation, and release packaging helpers.

## Validation

```bash
python3 tools/check_bundle.py
bash tools/init_repos.sh /tmp/alani-repos
```
