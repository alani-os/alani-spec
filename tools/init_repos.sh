#!/usr/bin/env bash
set -euo pipefail
DEST="${1:-alani-repos}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
mkdir -p "$DEST"
for repo in alani-kernel alani-runtime alani-lib alani-cognition alani-memory alani-devices alani-security alani-audit alani-terminal alani-userspace alani-filesystem alani-boot alani-platform alani-abi alani-protocol alani-ipc alani-storage alani-observability alani-init alani-config alani-policy alani-identity alani-network alani-sdk alani-sim alani-tests alani-benchmarks alani-docs alani-corpus alani-models alani-release alani-package; do
  rm -rf "$DEST/$repo"
  cp -R "$ROOT_DIR/repo-templates/individual/$repo" "$DEST/$repo"
  echo "created $DEST/$repo"
done
rm -rf "$DEST/alani-workspace"
cp -R "$ROOT_DIR/repo-templates/alani-workspace" "$DEST/alani-workspace"
echo "created $DEST/alani-workspace"
