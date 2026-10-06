#!/usr/bin/env bash
# Verify that libyuv source files do not introduce new identifiers in the
# `jpeg_*` namespace outside the known libjpeg-API consumer files.
#
# Runs after each source-build to catch silent shadowing if libyuv's upstream
# adds a `jpeg_foo` symbol that would collide with libjpeg-turbo after symbol
# rewrite.
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <OUT_DIR>"
  exit 1
fi

OUT_DIR="$1"

src_dir="$OUT_DIR/source/libyuv/source"
if [ ! -d "$src_dir" ]; then
  echo "ERROR: libyuv source directory not found at $src_dir"
  exit 1
fi

# libjpeg API を呼んでいる既知のファイル (これらの中の jpeg_* 参照は正当)
allowed_files="convert_jpeg.cc|mjpeg_decoder.cc|mjpeg_validate.cc"

# 全ファイルから jpeg_<lower> 識別子を grep し、許可ファイル以外を抽出
matches=$(grep -rEn '\bjpeg_[a-z_]+\b' "$src_dir" || true)
if [ -z "$matches" ]; then
  echo "OK: no jpeg_ symbol references in libyuv source"
  exit 0
fi

unexpected=$(echo "$matches" | grep -vE "/(${allowed_files}):" || true)

if [ -n "$unexpected" ]; then
  echo "ERROR: unexpected jpeg_ symbol references in libyuv source:"
  echo "$unexpected"
  exit 1
fi

echo "OK: jpeg_ symbol references only found in allowed files"
