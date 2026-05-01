# Alani Integration Workspace

Generated: 2026-04-30T23:21:07Z  
Version: 0.2.0-draft

This workspace is a local integration harness. Run `bash ../../tools/init_repos.sh /tmp/alani-repos` from the bundle root, then:

```bash
cd /tmp/alani-repos/alani-workspace
cargo test --workspace --all-features
```

The workspace references all 32 top-level repositories.
