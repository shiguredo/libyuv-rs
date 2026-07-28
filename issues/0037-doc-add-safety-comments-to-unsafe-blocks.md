# 全 unsafe ブロックに SAFETY コメントを追加する

- Priority: High
- Created: 2026-07-08
- Completed: {YYYY-MM-DD}
- Model: DeepSeek V4 Pro
- Branch: feature/update-add-safety-comments
- Polished: 2026-07-29
- Reporter: @voluntas

## 目的

コードベース全体の unsafe ブロック (計 357 箇所) に `// SAFETY:` コメントを追加し、各 FFI 呼び出しの安全性の前提条件を文書化する。将来の保守で検証コードと FFI 呼び出しの依存関係が失われるリスクを低減する。

## 優先度根拠

High。Rust の unsafe コードガイドラインで強く推奨されており、安全性監査・コードレビューの効率に直結する。現状すべての unsafe ブロック (357 箇所) に安全性の根拠が文書化されておらず、将来のリファクタリングで検証コードが誤って削除・変更された場合に未定義動作のリスクがある。

## 現状

全 unsafe ブロックに SAFETY コメントが存在しない。ファイル別内訳:

| ファイル | unsafe ブロック数 |
|---|---|
| `src/convert.rs` | 243 |
| `src/planar.rs` | 67 |
| `src/rotate.rs` | 20 |
| `src/scale.rs` | 18 |
| `src/compare.rs` | 9 |
| **合計** | **357** |

各 unsafe ブロックの先行検証コードは大きく以下のパターンに分類される:

- **パターン A (バリデーション済み FFI)**: `.validate()` または `require_c_int` + `checked_buf_size` + stride チェックで安全性保証 (約 350 箇所)
- **パターン B (スカラ FFI)**: 整数値のみを渡す FFI (ポインタ・バッファなし)。`sum_square_error_to_psnr`, `hash_djb2` (2 箇所)
- **パターン C (検証不足)**: 検証が欠落している関数内の unsafe ブロック。`mirror_plane` 等。検証追加は issue 0027 に委譲 (数箇所)

## 設計方針

パターン別に SAFETY コメントを使い分ける:

### パターン A: バリデーション済み FFI (~350 箇所)

```rust
// SAFETY: 全ポインタは事前のバリデーションで有効性が保証された Rust スライスから取得している。
// .validate() / require_c_int により width/height/stride は c_int 範囲内と確認済み。
// checked_buf_size または .validate() によりバッファサイズは stride*height 以上と確認済み。
unsafe {
    sys::SomeFunction(...)
}
```

.validate() を経由する関数では、「.validate() が全前提条件を検査済み」に簡略化する。

### パターン B: スカラ FFI (2 箇所)

```rust
// SAFETY: libyuv の HashDjb2 は src.as_ptr() から src.len() バイトまでしか読み込まず、
// 純粋な数値計算であり不正な入力に対しても未定義動作を起こさない。
unsafe {
    sys::HashDjb2(src.as_ptr(), src.len() as u64, seed)
}
```

### パターン C: 検証不足

```rust
// SAFETY: 注意 — この unsafe ブロックの前に十分な入力検証がない。
// 呼び出し元が c_int 範囲内の width/height/stride と適切なバッファサイズを保証する責任を負う。
// 検証追加は issue 0027 で対応予定。
unsafe {
    sys::MirrorPlane(...)
}
```

## 完了条件

- 全 357 unsafe ブロックに SAFETY コメントが追加されていること
- 各コメントが実際の検証パターンと一致していること（目視レビューで確認）
- `cargo fmt --all --check` が成功すること
- `cargo clippy --all-targets --all-features -- -D warnings` が成功すること
- `cargo test --workspace` が成功すること
- `CHANGES.md` の `## develop` → `### misc` セクションに `[UPDATE] 全 unsafe ブロック (357 箇所) に SAFETY コメントを追加する` を追記すること (@voluntas 署名付き)

## 解決方法

1. 各ソースファイルの unsafe ブロックにパターン A/B/C のいずれかの SAFETY コメントを追加する
2. パターン A の大部分 (~330 箇所) は機械的に追加可能。パターン B (2 箇所) とパターン C (数箇所) は手動で記述する
3. .validate() 経由の関数では「.validate() が全前提条件を検査済み」の簡略形を使用する
4. 16bit 系関数のコメントでは stride/バッファサイズの単位が要素数であることに注意する
5. 回転系関数では dst の width/height が入れ替わるため、個別に確認する
6. SAFETY コメントの正しさを目視でレビューする
7. CHANGES.md にエントリを追加する

### 0036 との依存関係

0036 (convert.rs 分割) が先に実施された場合、243 unsafe ブロックが 10 サブモジュールに分散する。その場合は各サブモジュールファイルに対して本 issue の作業を実施する。0036 と 0037 のいずれが先でも作業は可能だが、0036 → 0037 の順が望ましい（SAFETY コメントの追記を 1 回で済ませられるため）。
