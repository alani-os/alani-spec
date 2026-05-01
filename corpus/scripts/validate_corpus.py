#!/usr/bin/env python3
"""Validate Alani corpus JSONL files using standard-library checks."""
from __future__ import annotations
import json
import sys
from pathlib import Path

REQUIRED = ['record_id', 'schema_version', 'split', 'source', 'task', 'input', 'expected', 'labels', 'annotations']
VALID_SPLITS = {'train', 'validation', 'test'}

def validate_record(path: Path, line_no: int, record: dict) -> list[str]:
    errors = []
    for key in REQUIRED:
        if key not in record:
            errors.append(f'{path}:{line_no}: missing {key}')
    if record.get('schema_version') != 'alani.corpus.v1':
        errors.append(f'{path}:{line_no}: invalid schema_version')
    if record.get('split') not in VALID_SPLITS:
        errors.append(f'{path}:{line_no}: invalid split')
    if not isinstance(record.get('labels'), list) or not record.get('labels'):
        errors.append(f'{path}:{line_no}: labels must be non-empty list')
    source = record.get('source', {})
    for key in ['kind', 'license', 'generator']:
        if key not in source:
            errors.append(f'{path}:{line_no}: source missing {key}')
    annotations = record.get('annotations', {})
    if 'intent' not in annotations:
        errors.append(f'{path}:{line_no}: annotations missing intent')
    return errors

def main(argv: list[str]) -> int:
    all_errors = []
    total = 0
    for arg in argv[1:]:
        path = Path(arg)
        with path.open(encoding='utf-8') as f:
            for line_no, line in enumerate(f, 1):
                if not line.strip():
                    continue
                total += 1
                try:
                    record = json.loads(line)
                except json.JSONDecodeError as exc:
                    all_errors.append(f'{path}:{line_no}: invalid json: {exc}')
                    continue
                all_errors.extend(validate_record(path, line_no, record))
    if all_errors:
        print('\n'.join(all_errors), file=sys.stderr)
        return 1
    print(f'validated {total} corpus records')
    return 0

if __name__ == '__main__':
    raise SystemExit(main(sys.argv))
