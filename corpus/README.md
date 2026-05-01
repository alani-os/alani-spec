# Alani Synthetic Corpus

This corpus is created for the Alani specification bundle. It contains 160 deterministic synthetic JSONL records split into train, validation, and test sets.

## Files

- `data/train.jsonl` - 112 records
- `data/validation.jsonl` - 24 records
- `data/test.jsonl` - 24 records
- `schema/alani_corpus.schema.json` - JSON Schema
- `taxonomy/labels.yaml` - label definitions
- `scripts/generate_synthetic_corpus.py` - deterministic generator
- `scripts/validate_corpus.py` - standard-library validator
- `corpus_index.json` - corpus statistics

## Usage

```bash
python corpus/scripts/validate_corpus.py corpus/data/train.jsonl corpus/data/validation.jsonl corpus/data/test.jsonl
```

The included data is synthetic and intentionally avoids external copyrighted source text. Non-synthetic future data must pass the source and license checks described in Docs 53-56.
