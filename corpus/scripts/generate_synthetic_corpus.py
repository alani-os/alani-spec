#!/usr/bin/env python3
"""Generate deterministic synthetic Alani corpus records."""
from __future__ import annotations
import argparse
import json
import random
from pathlib import Path
from datetime import datetime, timezone

EXAMPLES = [
    ("terminal_command", "terminal", "Show the last five audit denials.", {"action": "audit.query", "arguments": {"decision": "deny", "limit": 5}}, ["audit", "query"]),
    ("syscall_trace", "syscall", "agent called sys_device_call without capability", {"syscall": "sys_device_call", "status": "permission_denied"}, ["security", "deny"]),
    ("policy_case", "security", "Runtime derives child capability for memory query only.", {"decision": "allow", "capability": "memory.query"}, ["memory", "allow"]),
    ("memory_record", "memory", "Query records related to scheduler priority aging.", {"operation": "memory.query", "query": "scheduler priority aging"}, ["scheduler"]),
    ("device_event", "device", "Timer interrupt deferred to scheduler tick.", {"event": "device.interrupt", "device_class": "timer"}, ["scheduler"]),
    ("repository_task", "repository", "Add ABI UserBuffer to alani-lib and mirror tests in alani-kernel.", {"repos": ["alani-lib", "alani-kernel"], "doc": 10}, ["syscall"]),
    ("qa_case", "qa", "Corpus validator should reject missing labels.", {"check": "corpus.validate", "expected": "fail"}, ["cognition"]),
]

def make_records(count: int, seed: int):
    random.seed(seed)
    generated = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
    records = []
    for i in range(count):
        task, base_label, text, expected, extra = EXAMPLES[i % len(EXAMPLES)]
        split = 'train' if i < int(count * 0.70) else 'validation' if i < int(count * 0.85) else 'test'
        records.append({
            'record_id': f'alani-synth-{i+1:06d}',
            'schema_version': 'alani.corpus.v1',
            'split': split,
            'source': {'kind': 'synthetic', 'license': 'Alani synthetic corpus draft - review before publication', 'generator': 'generate_synthetic_corpus.py'},
            'task': task,
            'input': {'text': text, 'context': {'trace_id': f'trc-{(i % 37) + 1:04d}', 'principal': random.choice(['agent:hello', 'operator:dev', 'task:runtime-init'])}},
            'expected': expected,
            'labels': sorted(set([base_label] + extra)),
            'annotations': {'intent': task, 'safety_class': random.choice(['public', 'internal']), 'priority': random.choice(['low', 'normal', 'high'])},
            'created_at': generated,
        })
    return records

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--count', type=int, default=160)
    parser.add_argument('--seed', type=int, default=42)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    records = make_records(args.count, args.seed)
    for split in ['train', 'validation', 'test']:
        with (args.out / f'{split}.jsonl').open('w', encoding='utf-8') as f:
            for record in records:
                if record['split'] == split:
                    f.write(json.dumps(record, sort_keys=True) + '\n')

if __name__ == '__main__':
    main()
