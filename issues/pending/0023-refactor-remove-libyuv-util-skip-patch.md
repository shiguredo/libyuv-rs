# libyuv util ツールビルド除外パッチを削除する

- Priority: Low
- Created: 2026-06-15
- Model: Opus 4.7
- Branch: feature/refactor-remove-libyuv-util-skip-patch

## 目的

`build.rs` の `patch_libyuv_skip_util_tools` / `strip_libyuv_util_tools` を削除し、
libyuv の `CMakeLists.txt` を改変せずにそのまま使えるようにする。

## 優先度根拠

Low。

- 現行パッチは静的ライブラリ `libyuv.a` の機能に影響しない (Rust 側から
  util ツールは参照していない)
- ビルド時間への副作用もほぼ無い
- 削除はあくまで上流改変を減らすクリーンアップで、ユーザー影響は無い

ただしパッチは libyuv 上流の `CMakeLists.txt` 構造に依存しており、上流の
変更でパターンが一致しなくなったときは build.rs が `panic!` で落ちる。
維持コストはゼロではないため、外部条件が整い次第削除したい。

## 現状

`build.rs` で libyuv を clone した直後に `patch_libyuv_skip_util_tools` を
呼び、libyuv の `CMakeLists.txt` から util ツール (`cpuid` /
`yuvconvert` / `yuvconstants`) 関連の `add_executable` /
`target_link_libraries` / `install` 行を取り除いてから cmake build に進む。

該当コード:

- `patch_libyuv_skip_util_tools` の呼び出し
- `patch_libyuv_skip_util_tools` 関数本体
- `strip_libyuv_util_tools` 関数本体

## パッチが必要な理由

libyuv commit `d23308a2a7442be8e559b1b471862fd7588d6a57`
(`add bmm detect and vdpphps in util/cpuid`, 2026-06-09) で `util/cpuid.c`
に AVX10.2 命令 `vdpphps` のインラインアセンブリによる実機テストが追加
された。

```c
__asm__ volatile("vdpphps %%xmm0, %%xmm0, %%xmm0" : : : "xmm0");
```

このコードは `#if defined(__i386__) || defined(__x86_64__) || defined(_M_IX86) || defined(_M_X64)`
配下にあるため、x86_64 ターゲットでのみコンパイル対象となる。

`vdpphps` mnemonic は GNU assembler (gas / binutils) 2.44 以降でしか
認識されない。CI で使う GitHub Actions ランナーの binutils バージョンは:

| ランナー | binutils | vdpphps 対応 |
|---|---|---|
| `ubuntu-22.04` | 2.38 | 不可 |
| `ubuntu-24.04` | 2.42 | 不可 |
| `ubuntu-22.04-arm` / `ubuntu-24.04-arm` | (関係なし) | x86_64 コードパス自体が無効化されるため無関係 |

x86_64 ランナーで libyuv 全体をビルドすると util/cpuid.c のアセンブルで
`Error: no such instruction: 'vdpphps %xmm0,%xmm0,%xmm0'` で停止する。

util ツールは libyuv の動作確認用 CLI で、Rust バインディングからは
リンクも参照もしていない。したがって util ツールをビルド対象から外せば
静的ライブラリ `libyuv.a` の関数群には影響せず CI が回復する、というのが
本パッチの理屈。

## 削除可能になる条件

GitHub Actions の x86_64 Linux ランナーで、ビルド時に呼ばれる `as`
(binutils) が 2.44 以降になっていること。具体的には以下のいずれか:

1. `ubuntu-24.04` LTS の `binutils` パッケージが 2.44 以降に更新される
   (LTS の通常更新では 2.42 系のまま据え置かれる可能性が高い)
2. GitHub Actions が `ubuntu-26.04` 以上のランナーを提供し、CI マトリクス
   から `ubuntu-22.04` / `ubuntu-24.04` を落とせる状態になる
3. CI マトリクスから x86_64 Linux を落とすか、コンテナベースの CI に
   切り替えて新しい distro を使う方針に変える (別議論)

## 検討した代替案 (採用しなかった)

調査結果として記録する。再検討の際の出発点にする。

### PPA で binutils 2.44 を入れる

`ubuntu-toolchain-r/test` PPA は GCC 専用で binutils は提供していない。
`savoury1/build-tools` PPA は build-tools を提供するが binutils は noble
標準の 2.42 のままで 2.44 は提供していない。Launchpad で Noble 向けの
binutils 2.44 を出している公的 PPA は見つからなかった。

### Ubuntu 25.04 (Plucky) の binutils 2.44 .deb を直接 dpkg 投入する

ローカル Docker で再現した結果:

- `ubuntu-24.04` (libc 2.39): Plucky の `binutils_2.44-3ubuntu1_amd64.deb`
  一式 (`libbinutils`, `libctf0`, `libctf-nobfd0`, `libgprofng0`,
  `binutils-common`, `binutils-x86-64-linux-gnu`) を `apt-get install` で
  投入でき、`as --version` が 2.44 になり vdpphps もアセンブル成功。
- `ubuntu-22.04` (libc 2.35): binutils 2.44 が libc 2.38 以上を要求するため
  依存解決に失敗 (`held broken packages`)。

Jammy を CI マトリクスから落とす覚悟があれば Noble だけ Plucky の .deb
投入で延命できるが、現時点ではマトリクス縮小はしない判断 (本 issue
着手見送りの理由)。

### binutils をソースからビルド

x86_64 ランナーで binutils 2.44 を `./configure && make` で入れれば全
ランナーで動かせるが、1 ランナーあたり 3〜5 分のビルド時間が増える。
CI 全体の体感悪化が大きく、現行の build.rs パッチ (実質ゼロコスト) と
比べてリスクとリターンが釣り合わない。

## 設計方針

### 削除対象

- `build.rs` の `patch_libyuv_skip_util_tools` 呼び出しおよびその前置コメント
- `build.rs` の `patch_libyuv_skip_util_tools` 関数本体
- `build.rs` の `strip_libyuv_util_tools` 関数本体

### 削除しないもの

無し。

### 検証

CI マトリクス全ランナーで `cargo test --workspace --features source-build`
が成功すること。特に `ubuntu-22.04` / `ubuntu-24.04` (x86_64) で libyuv の
`util/cpuid.c` のアセンブルが通ることを確認する。

## 完了条件

- `build.rs` から util ツール除外パッチ関連コードが削除されている
- CI マトリクス全ランナーで `cargo test --workspace --features source-build`
  がグリーン
- libyuv 上流の commit hash を bump しても CI が通る状態を維持

## pending にした理由

GitHub Actions の x86_64 Linux ランナーの binutils が 2.44 以降になるまで
着手不可能。我々の側で解消できる外部依存ではないため `issues/pending/` に
置く。
