#!/usr/bin/env bash
# Verify that the libjpeg-turbo tag in Cargo.toml matches the one embedded
# in the THIRD_PARTY_LICENSES file, and that the commit hash recorded in
# THIRD_PARTY_LICENSES is the one the tag points to on the remote.
#
# This guards against forgetting to update the bundled license text when
# bumping the libjpeg-turbo version.
set -euo pipefail

LIBJPEG_TURBO_GIT="https://github.com/libjpeg-turbo/libjpeg-turbo"

# Cargo.toml の [package.metadata.external-dependencies.libjpeg-turbo] セクションから
# version (tag) を抽出する。
cargo_tag=$(awk '
  /^\[package\.metadata\.external-dependencies\.libjpeg-turbo\]/ { in_sec=1; next }
  /^\[/ { in_sec=0 }
  in_sec && $1 == "version" { print $3 }
' Cargo.toml | tr -d '"')

# THIRD_PARTY_LICENSES 冒頭の "# libjpeg-turbo (tag <tag>, commit <hash>)" から tag/hash を取り出す。
license_tag=$(sed -n '1s/.*(tag \([^,]*\), commit .*/\1/p' THIRD_PARTY_LICENSES)
license_hash=$(sed -n '1s/.*commit \([0-9a-f]*\)).*/\1/p' THIRD_PARTY_LICENSES)

if [ -z "$cargo_tag" ]; then
  echo "ERROR: failed to extract libjpeg-turbo tag from Cargo.toml"
  exit 1
fi
if [ -z "$license_tag" ] || [ -z "$license_hash" ]; then
  echo "ERROR: failed to extract libjpeg-turbo tag/hash from THIRD_PARTY_LICENSES"
  exit 1
fi

if [ "$cargo_tag" != "$license_tag" ]; then
  echo "ERROR: libjpeg-turbo tag mismatch:"
  echo "  Cargo.toml:           $cargo_tag"
  echo "  THIRD_PARTY_LICENSES: $license_tag"
  exit 1
fi

# リモート上でタグが指すコミットハッシュを取得する。
# annotated tag の場合は "^{}" 行が返るが、lw_tag の場合はそのまま返るので、
# 末尾のピール済み行を採用する。
remote_hash=$(git ls-remote --tags "$LIBJPEG_TURBO_GIT" "refs/tags/${cargo_tag}" "refs/tags/${cargo_tag}^{}" | awk '{print $1}' | tail -n 1)

if [ -z "$remote_hash" ]; then
  echo "ERROR: failed to resolve tag ${cargo_tag} on remote"
  exit 1
fi

if [ "$license_hash" != "$remote_hash" ]; then
  echo "ERROR: libjpeg-turbo commit hash mismatch for tag ${cargo_tag}:"
  echo "  THIRD_PARTY_LICENSES: $license_hash"
  echo "  remote tag:           $remote_hash"
  exit 1
fi

echo "OK: libjpeg-turbo tag and commit hash match ($cargo_tag, $license_hash)"
