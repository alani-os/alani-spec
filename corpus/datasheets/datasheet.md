# Datasheet for Alani Synthetic Corpus

## Motivation

The corpus supports early tests for terminal commands, syscall traces, policy decisions, memory operations, device events, repository tasks, and QA cases.

## Composition

The corpus contains deterministic synthetic JSONL records. It does not include personal data or externally sourced text.

## Collection Process

Records were generated from templates derived from the Alani specification. Future collection must preserve provenance, license, and sensitivity metadata.

## Recommended Uses

- CI validation fixtures.
- Evaluation of command parsing and policy-routing examples.
- Developer examples for memory, audit, and cognition APIs.

## Out-of-Scope Uses

- Training production models without expansion and review.
- Legal, safety, or security certification.
- Benchmarking real-world model quality.
