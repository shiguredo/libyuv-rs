#!/usr/bin/env bash
# Verify that the symbol rewrite step produced clean static archives.
#
# Assertions:
#   1. libshiguredo_yuv.a has no unresolved jpeg_*/jsimd_* references
#   2. libshiguredo_jpeg.a has no unresolved jpeg_*/jsimd_* references
#   3. libshiguredo_yuv.a defines the shiguredo_yuv_MJPG* symbols
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <OUT_DIR>"
  exit 1
fi

OUT_DIR="$1"

if [ ! -d "$OUT_DIR" ]; then
  echo "ERROR: OUT_DIR does not exist: $OUT_DIR"
  exit 1
fi

# rustc の sysroot から llvm-nm のパスを解決する。
# rustc -vV の host 行は Windows で改行コードに CR が混ざるため tr で除去する。
sysroot=$(rustc --print sysroot)
host=$(rustc -vV | sed -n 's|host: ||p' | tr -d '\r')
llvm_dir="$sysroot/lib/rustlib/$host/bin"
LLVM_NM="$llvm_dir/llvm-nm"

# Windows では .exe 拡張子が必要
if [ ! -x "$LLVM_NM" ] && [ -x "$LLVM_NM.exe" ]; then
  LLVM_NM="$LLVM_NM.exe"
fi

if [ ! -x "$LLVM_NM" ]; then
  echo "ERROR: llvm-nm not found at $LLVM_NM"
  echo "       (run: rustup component add llvm-tools)"
  exit 1
fi

# プラットフォームごとの静的ライブラリパスを解決する。
if [ -f "$OUT_DIR/lib/libshiguredo_yuv.a" ]; then
  yuv_lib="$OUT_DIR/lib/libshiguredo_yuv.a"
  jpeg_lib="$OUT_DIR/lib/libshiguredo_jpeg.a"
elif [ -f "$OUT_DIR/lib/shiguredo_yuv.lib" ]; then
  yuv_lib="$OUT_DIR/lib/shiguredo_yuv.lib"
  jpeg_lib="$OUT_DIR/lib/shiguredo_jpeg.lib"
else
  echo "ERROR: shiguredo_yuv / shiguredo_jpeg static library not found in $OUT_DIR/lib/"
  exit 1
fi

if [ ! -f "$jpeg_lib" ]; then
  echo "ERROR: shiguredo_jpeg static library not found: $jpeg_lib"
  exit 1
fi

# Assertion 1 / 2: 未解決の jpeg_/jsimd_ 参照が残っていないこと。
# プレフィックス (`shiguredo_`) 付きのシンボルは正常に書き換え済みなので
# grep -v shiguredo_ で除外する。
# シンボル名は C 識別子のため `[A-Za-z0-9_]+` 末尾で受ける。`jpeg_nbits.c.o:` のような
# llvm-nm が間に挟むオブジェクトファイル名行 (末尾が `:`) を誤検出しないようにする。
check_unresolved() {
  local lib="$1"
  local label="$2"
  echo "checking $label for unresolved jpeg_/jsimd_ symbols..."
  # llvm-nm -u は未定義シンボルだけを列挙する。
  if "$LLVM_NM" -u "$lib" | grep -E '^_?(jpeg_|jsimd_)[A-Za-z0-9_]+$' | grep -v shiguredo_; then
    echo "ERROR: $label has unrewritten jpeg_/jsimd_ undefined references"
    exit 1
  fi
}

check_unresolved "$yuv_lib" "shiguredo_yuv"
check_unresolved "$jpeg_lib" "shiguredo_jpeg"

# Assertion 3: yuv 側に shiguredo_yuv_MJPG* シンボルが定義されている。
# macOS の Mach-O では先頭に `_` が付くため、`_?` で受ける。
echo "checking $yuv_lib for defined shiguredo_yuv_MJPG* symbols..."
defined=$("$LLVM_NM" --defined-only --extern-only "$yuv_lib")
expected_symbols=(MJPGSize MJPGToI420 MJPGToNV12 MJPGToNV21 MJPGToARGB)
missing=0
for sym in "${expected_symbols[@]}"; do
  if ! echo "$defined" | grep -E "(^|[^[:alnum:]_])_?shiguredo_yuv_${sym}([^[:alnum:]_]|$)" > /dev/null; then
    echo "ERROR: missing defined symbol shiguredo_yuv_$sym in $yuv_lib"
    missing=1
  fi
done

if [ "$missing" -ne 0 ]; then
  exit 1
fi

echo "OK: all symbol rewrite assertions passed"
