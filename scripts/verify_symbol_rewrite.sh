#!/usr/bin/env bash
# シンボル書き換え手順が生成した静的ライブラリがクリーンであることを検証する。
#
# Assertions（検証内容）:
#   1. libshiguredo_yuv.a に jpeg リネームマップが対象とする未書き換え参照が残っていないこと
#   2. libshiguredo_jpeg.a に jpeg リネームマップが対象とする未書き換え参照が残っていないこと
#   3. libshiguredo_yuv.a が shiguredo_yuv_MJPG* シンボルを定義していること
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

# Assertion 1 / 2: 未解決の書き換え対象シンボル参照が残っていないこと。
#
# 検査対象は build.rs が書き出す `symbol_rename_map_jpeg.txt` の第 1 フィールド
# （書き換え前シンボル名）と、`llvm-nm -u --format=just-symbols` で得た未定義シンボルの
# **完全一致**（集合所属）。書き換え対象は jpeg 側の定義済み外部シンボル全てであり、
# その集合はマップの左列に等しい。`jpeg_` / `jsimd_` 接頭辞だけで受ける旧ロジックは
# `jinit_*` / `jpeg12_*` 等の書き換え漏れを検出できなかったため、マップと突合する。
check_unresolved() {
  local lib="$1"
  local label="$2"
  local map_file="$3"
  echo "checking $label for unresolved references matching the jpeg rename map..."

  # マップファイルが無い・空なら検査をスキップせずエラーにする。CI / release の
  # Verify ステップは source-build 直後のためマップは必ず存在する。無い状態は
  # ビルドフローの異常なので黙って通過させない
  if [ ! -f "$map_file" ] || [ ! -s "$map_file" ]; then
    echo "ERROR: symbol rename map not found or empty: $map_file"
    exit 1
  fi

  # llvm-nm -u は未定義シンボルだけを列挙し、-u 指定時の macOS (darwin) は裸の
  # シンボル名 (先頭に `_`) だけを出力する。既定形式では Linux (ELF) / Windows (COFF)
  # が bsd 形式で行頭空白 + 型列 (U) を付けるため、--format=just-symbols で出力を
  # シンボル名のみに統一する (just-symbols でも macOS の `_` プレフィックスは維持される)。
  # Windows では出力に \r が混入することがあり、完全一致の集合突合では `\r` が
  # 付いた字句は一致しないため、tr -d '\r' で除去する (スクリプト冒頭の host 行の
  # CR 除去と同じ対策。理由を知らないと不要な防御として削除され得る)。
  # コマンド置換は末尾の改行を除去するため、printf で改行を復元してから処理する。
  # llvm-nm の失敗 (破損アーカイブ等) は set -e によりここでスクリプトが失敗する
  # (パイプラインの中に置くと grep 不発で検査が黙って通ってしまうため)。
  local nm_output
  nm_output=$("$LLVM_NM" -u --format=just-symbols "$lib")

  # マップは "旧名 新名" を空白 1 個区切りで 1 行に記述する。現行の libjpeg-turbo
  # 3.1.90 では数百行規模で、write_objcopy_rename_map が lines.join("\n") で書き出すため
  # **末尾に改行が無い**。`while read` で読むと最終行を落とすため、行単位処理は使わず
  # sed / sort / comm で全行を一行パイプラインで扱う。旧名は C 識別子なので空白を含まない。
  #
  # 突合は完全一致（集合所属）で行い、部分一致の grep -F は使わない（旧名
  # `jinit_master_decompress` は新名 `shiguredo_jpeg_jinit_master_decompress` の
  # 部分文字列であり、書き換え済みアーカイブの未定義には新名が残るため、部分一致は
  # 正常アーカイブを常に失敗させる）。第 2 フィールド（新名）は集合に入れない
  # （yuv → jpeg 公開 API の未定義参照や jpeg のオブジェクト間参照が新名で残るため、
  # 新名を含めると正常アーカイブが失敗する）。判定の方向は「未定義シンボルが
  # マップの旧名集合に属するか」のみ。
  #
  # macOS のマップ左列は `_jinit_*`、ELF / COFF の nm 出力は `jinit_*` のため、
  # 両側の先頭 `_` を高々 1 個落としてから突合する（旧 grep の `^_?` と同じ規則）。
  # nm の出力にはオブジェクトファイル名行 (例: `jpeg_nbits.c.o:`) も混じるため、
  # C 識別子のみに絞り込む（先頭 `_` 除去後に alphabetic または `_` で始まり、
  # alphanumeric / `_` のみで構成される行がシンボル名）。
  #
  # マップの総行数（非空行）を先に数える。抽出した旧名集合の件数と突き合わせ、
  # 1 行でも抽出から脱落したら形式のずれ（タブ区切り・第 1 フィールドが C 識別子で
  # ない行等）とみなして FAIL する。実マップは全行 "旧名 新名" の空白 1 個区切りで、
  # 第 1 フィールドが全て C 識別子である契約を前提とする。`awk` は末尾改行の無い
  # 最終行も数える（write_objcopy_rename_map は lines.join("\n") で改行なしで書き出す）
  # ため、`wc -l` は使わない。
  # 件数照合は旧名の一意性（build.rs の HashMap キー由来で構造的に保証）を前提とする。
  # 万一重複行が入った場合は sort -u による件数差が「正常ビルドを落とす方向」
  # （fail-closed）になり、検査を逃れる方向には使えない。なお件数を保ったままの
  # 形式崩れ（例: 旧名と新名の反転行）は検出できないが、「旧名 新名」の空白 1 個
  # 区切りは build.rs の format!("{old} {new}") で強制されており生成経路に存在しない
  local map_total
  # awk の count に + 0 をしないと、NF のある行が 1 つも無い場合（-s 検査を抜ける
  # 「空行のみの非空ファイル」等）に count が未初期化の空文字になり、続く
  # `[ "$map_total" -eq 0 ]` が `integer expression expected` のシェルエラーを返して
  # 条件が偽扱いになり、専用の ERROR を出さずに検査が黙って通り抜ける。+ 0 で
  # 数値 0 を出力してこれを防ぐ
  map_total=$(awk 'NF { count++ } END { print count + 0 }' "$map_file")
  if [ "$map_total" -eq 0 ]; then
    echo "ERROR: rename map has no non-empty lines: $map_file"
    exit 1
  fi
  local old_names
  old_names=$(
    sed -E 's/^_//; s/ .*$//' "$map_file" |
      # マップ側は build.rs の fs::write (LF のみ) が生成するため \r は混入経路が
      # 無いが、nm 出力側と同じ正規化で統一する保険として残す
      tr -d '\r' |
      grep -E '^([A-Za-z_])([A-Za-z0-9_]+)?$' |
      LC_ALL=C sort -u
  ) || true
  # 上記パイプラインは grep の「マッチなし = exit 1」を set -e で拾わせないため
  # `|| true` を付けており、件数照合で空・部分脱落を判定する。この `|| true` が
  # 意味を持つのは set -o pipefail により grep の exit 1 がパイプライン全体の
  # ステータスへ伝播するためである。pipefail 下では sed / tr / sort の実障害も
  # 握りつぶされるため、実障害が部分出力を残す場合は下の件数照合で検出する
  local old_count
  old_count=$(printf '%s\n' "$old_names" | awk 'NF { count++ } END { print count + 0 }')
  if [ "$old_count" -ne "$map_total" ]; then
    echo "ERROR: extracted $old_count / $map_total names from rename map: $map_file"
    echo "       (rename map format drift)"
    exit 1
  fi
  local unresolved
  unresolved=$(
    printf '%s\n' "$nm_output" |
      tr -d '\r' |
      sed -E 's/^_//' |
      grep -E '^([A-Za-z_])([A-Za-z0-9_]+)?$' |
      LC_ALL=C sort -u
  ) || true
  # 同上（grep のマッチなしを pipefail 経由で拾わないための `|| true`。理由は上の
  # old_names 側のコメント参照）。ただし old_names 側と違い、unresolved 側には件数の
  # ベースラインが無いため、grep 以外の実障害（sed / tr / sort）が部分出力を残した
  # 場合は検出できない。テキスト処理のみのパイプラインで実害はほぼ無いが、この
  # 限界は知っておくこと。nm -u の出力が空は本来あり得ない（静的ライブラリは
  # libc 等への未定義参照を必ず持つ）ため、異常として落とす
  if [ -z "$unresolved" ]; then
    echo "ERROR: llvm-nm reported no undefined symbols for $lib"
    exit 1
  fi
  # comm -12 は両集合の共通部分（= 書き換え対象なのに未書き換えで残っている未定義参照）
  local leftover
  leftover=$(LC_ALL=C comm -12 <(printf '%s\n' "$old_names") <(printf '%s\n' "$unresolved"))
  if [ -n "$leftover" ]; then
    echo "ERROR: $label has unrewritten undefined references:"
    echo "$leftover"
    exit 1
  fi
}

check_unresolved "$yuv_lib" "shiguredo_yuv" "$OUT_DIR/symbol_rename_map_jpeg.txt"
check_unresolved "$jpeg_lib" "shiguredo_jpeg" "$OUT_DIR/symbol_rename_map_jpeg.txt"

# Assertion 3: yuv 側に shiguredo_yuv_MJPG* シンボルが定義されている。
# macOS の Mach-O では先頭に `_` が付くため、`_?` で受ける。なお Assertion 1 / 2 とは
# 違い形式統一や \r 除去が不要なのは、両端の文字クラス ([^[:alnum:]_]) が bsd 形式の
# 型列と Windows の \r 混入を吸収するためである。
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
