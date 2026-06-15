#!/usr/bin/env bash
# Verify that the libjpeg-turbo commit hash in Cargo.toml matches the one
# embedded in the THIRD_PARTY_LICENSES file.
#
# This guards against forgetting to update the bundled license text when
# bumping the libjpeg-turbo version.
set -euo pipefail

# Cargo.toml の [package.metadata.external-dependencies.libjpeg-turbo] セクションから
# version (commit hash) を抽出する。
# awk のセクションフラグでセクション境界 (libjpeg-turbo の開始 → 次の [...]) 内の
# version 行だけを拾い、tr で前後のダブルクォートを除去する。
# range pattern (/start/,/end/) はヘッダ行自体が end にマッチして 1 行で終わるため使えない。
cargo_hash=$(awk '
  /^\[package\.metadata\.external-dependencies\.libjpeg-turbo\]/ { in_sec=1; next }
  /^\[/ { in_sec=0 }
  in_sec && $1 == "version" { print $3 }
' Cargo.toml | tr -d '"')

# THIRD_PARTY_LICENSES 冒頭の "# libjpeg-turbo (commit <hash>)" から hash を取り出す。
license_hash=$(sed -n '1s/.*(commit \([0-9a-f]*\)).*/\1/p' THIRD_PARTY_LICENSES)

if [ -z "$cargo_hash" ]; then
  echo "ERROR: failed to extract libjpeg-turbo version from Cargo.toml"
  exit 1
fi
if [ -z "$license_hash" ]; then
  echo "ERROR: failed to extract libjpeg-turbo commit hash from THIRD_PARTY_LICENSES"
  exit 1
fi

if [ "$cargo_hash" != "$license_hash" ]; then
  echo "ERROR: libjpeg-turbo commit hash mismatch:"
  echo "  Cargo.toml:           $cargo_hash"
  echo "  THIRD_PARTY_LICENSES: $license_hash"
  exit 1
fi

echo "OK: libjpeg-turbo commit hash matches ($cargo_hash)"
