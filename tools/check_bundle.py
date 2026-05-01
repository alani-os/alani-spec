#!/usr/bin/env python3
from pathlib import Path
import json
import sys

ROOT = Path(__file__).resolve().parents[1]
REPOS = [
    "alani-kernel",
    "alani-runtime",
    "alani-lib",
    "alani-cognition",
    "alani-memory",
    "alani-devices",
    "alani-security",
    "alani-audit",
    "alani-terminal",
    "alani-userspace",
    "alani-filesystem",
    "alani-boot",
    "alani-platform",
    "alani-abi",
    "alani-protocol",
    "alani-ipc",
    "alani-storage",
    "alani-observability",
    "alani-init",
    "alani-config",
    "alani-policy",
    "alani-identity",
    "alani-network",
    "alani-sdk",
    "alani-sim",
    "alani-tests",
    "alani-benchmarks",
    "alani-docs",
    "alani-corpus",
    "alani-models",
    "alani-release",
    "alani-package"
]

def fail(msg):
    print('ERROR: ' + msg, file=sys.stderr)
    raise SystemExit(1)

def main():
    docs = []
    for i in range(1, 63):
        matches = sorted((ROOT / 'docs').glob(f'{i:02d}_*.md'))
        if len(matches) != 1:
            fail(f'expected one baseline doc for {i:02d}, found {len(matches)}')
        docs.append(matches[0])
    if not (ROOT / 'docs' / '63_Repository_Expansion_Addendum.md').exists():
        fail('missing Doc 63')
    for repo in REPOS:
        spec = ROOT / 'docs' / 'repositories' / (repo + '.md')
        if not spec.exists():
            fail('missing repository spec ' + str(spec))
        base = ROOT / 'repo-templates' / 'individual' / repo
        for rel in ['Cargo.toml','README.md','LICENSE','rustfmt.toml','src/lib.rs','tests/smoke.rs','.github/workflows/ci.yml']:
            if not (base / rel).exists():
                fail('missing ' + str(base / rel))
    ws = ROOT / 'repo-templates' / 'alani-workspace' / 'Cargo.toml'
    text = ws.read_text(encoding='utf-8')
    for repo in REPOS:
        if '../' + repo not in text:
            fail('workspace missing ' + repo)
    total = 0
    for split in ['train','validation','test']:
        path = ROOT / 'corpus' / 'data' / (split + '.jsonl')
        if not path.exists():
            fail('missing corpus split ' + str(path))
        for line_no, line in enumerate(path.read_text(encoding='utf-8').splitlines(), 1):
            if line.strip():
                obj = json.loads(line)
                if obj.get('schema_version') != 'alani.corpus.v1':
                    fail(f'invalid schema_version in {path} line {line_no}')
                total += 1
    print(f'Bundle OK: {len(docs)} baseline docs, 1 addendum, {len(REPOS)} repo templates/specs, {total} corpus records')

if __name__ == '__main__':
    main()
