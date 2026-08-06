//! プレーン操作 API の単体テスト
//!
//! エラーパス・境界値と、既知解による正常系（stride の行配置）を検証する。

use std::ffi::c_int;

use shiguredo_libyuv::{ImageSize, copy_plane, half_float_plane, split_uv_plane};

// 異常系: copy_plane のバッファ不足で Err が返ること
#[test]
fn copy_plane_buffer_too_small() {
    let src = vec![0u8; 64];
    let mut dst = vec![0u8; 10]; // 64 バイト必要だが 10 しか用意しない
    let size = ImageSize::new(8, 8);

    let result = copy_plane(&src, 8, &mut dst, 8, size);
    assert!(result.is_err(), "バッファ不足では Err が返るべき");
}

// 異常系: copy_plane の stride 不足で Err が返ること
#[test]
fn copy_plane_stride_too_small() {
    let src = vec![0u8; 64];
    let mut dst = vec![0u8; 64];
    let size = ImageSize::new(8, 8);

    let result = copy_plane(&src, 4, &mut dst, 8, size); // src stride=4 < width=8
    assert!(result.is_err(), "stride 不足では Err が返るべき");
}

// 異常系: split_uv_plane のバッファ不足で Err が返ること
#[test]
fn split_uv_plane_buffer_too_small() {
    let uv = vec![0u8; 128];
    let mut u = vec![0u8; 10]; // 32 バイト必要だが 10 しか用意しない
    let mut v = vec![0u8; 32];
    let size = ImageSize::new(8, 8);

    let result = split_uv_plane(&uv, 16, &mut u, 8, &mut v, 8, size);
    assert!(result.is_err(), "バッファ不足では Err が返るべき");
}

// 異常系: split_uv_plane の stride 不足で Err が返ること
#[test]
fn split_uv_plane_stride_too_small() {
    let uv = vec![0u8; 128];
    let mut u = vec![0u8; 32];
    let mut v = vec![0u8; 32];
    let size = ImageSize::new(8, 8);

    // uv stride=8 < width*2=16
    let result = split_uv_plane(&uv, 8, &mut u, 8, &mut v, 8, size);
    assert!(result.is_err(), "stride 不足では Err が返るべき");
}

// 正常系: half_float_plane が stride == width で正しい出力を返すこと
#[test]
fn half_float_plane_stride_equal_width() {
    // scale = 1.0 では 2 の冪の入力は全バックエンド（C / NEON / AVX2 / F16C / SVE2）で
    // 変換後も厳密に一致する（仮数が 0 のため丸めが発生しない）。
    // 出力は f16 のビットパターンで、2^k は (k + 15) << 10 になる
    let width = 8;
    let height = 4;
    let src: Vec<u16> = (0..(width * height)).map(|i| 1u16 << (i % 16)).collect();
    let expected: Vec<u16> = (0..(width * height))
        .map(|i| (((i % 16) + 15) << 10) as u16)
        .collect();
    let mut dst = vec![0u16; width * height];
    let size = ImageSize::new(width, height);

    half_float_plane(&src, width, &mut dst, width, 1.0, size)
        .expect("stride == width の変換が成功すること");

    assert_eq!(
        dst, expected,
        "2 の冪の入力は f16 ビットパターンに厳密に一致するべき"
    );
}

// 正常系: half_float_plane が stride > width（パディング付き）で行配置を正しく行うこと
#[test]
fn half_float_plane_padded_stride_row_placement() {
    // stride > width では C 側の行合体（coalesce）が効かないため、行ごとの stride 送りが検証される。
    // 行 r を 2 の冪 2^r で埋め、出力の各行が 2^r の f16 ビットパターン ((r + 15) << 10) に
    // なることを確認する。stride を要素数のままバイト単位として渡すと C 側の行送りが
    // 半分になり、行が重なり合ってこのテストは失敗する
    let width = 8;
    let height = 4;
    let src_stride = 12; // パディング 4 要素
    let dst_stride = 12;
    let mut src = vec![0u16; src_stride * height];
    for r in 0..height {
        for i in 0..src_stride {
            src[r * src_stride + i] = 1u16 << r;
        }
    }
    let mut dst = vec![0xFFFFu16; dst_stride * height]; // 未書き込み領域の検出用
    let size = ImageSize::new(width, height);

    half_float_plane(&src, src_stride, &mut dst, dst_stride, 1.0, size)
        .expect("パディング付きの変換が成功すること");

    for r in 0..height {
        let expected = ((r as u16) + 15) << 10;
        for i in 0..width {
            assert_eq!(
                dst[r * dst_stride + i],
                expected,
                "行 {} の要素 {} が正しく配置されること",
                r,
                i
            );
        }
        // libyuv は 1 行あたり width 要素だけ書き込むため、パディング領域は書き換わらない
        for i in width..dst_stride {
            assert_eq!(
                dst[r * dst_stride + i],
                0xFFFF,
                "パディング領域は書き換わらないこと"
            );
        }
    }
}

// 異常系: half_float_plane の stride × 2（バイト単位変換後）が c_int の範囲を超えると Err が返ること
#[test]
fn half_float_plane_stride_bytes_exceeds_c_int() {
    // 要素数単位の stride は c_int の範囲内でも、バイト単位に変換すると範囲を超える。
    // 変換後の検証はバッファサイズ検証より前にあるため、巨大なバッファは必要ない。
    // src / dst それぞれの変換後 stride が検証されることを確認する
    let src = vec![0u16; 1];
    let mut dst = vec![0u16; 1];
    let size = ImageSize::new(1, 1);
    let max_stride = c_int::MAX as usize;

    let result = half_float_plane(&src, max_stride, &mut dst, 1, 1.0, size);
    let err = result.expect_err("src 側の stride × 2 超過では Err が返るべき");
    assert!(
        err.to_string()
            .contains("source stride (bytes) exceeds c_int range"),
        "src 側の変換後 stride 超過の reason が返るべき: {}",
        err
    );

    let result = half_float_plane(&src, 1, &mut dst, max_stride, 1.0, size);
    let err = result.expect_err("dst 側の stride × 2 超過では Err が返るべき");
    assert!(
        err.to_string()
            .contains("destination stride (bytes) exceeds c_int range"),
        "dst 側の変換後 stride 超過の reason が返るべき: {}",
        err
    );
}

// 異常系: half_float_plane の src_stride が width 未満で Err が返ること
#[test]
fn half_float_plane_src_stride_too_small() {
    let src = vec![0u16; 8];
    let mut dst = vec![0u16; 16];
    let size = ImageSize::new(8, 1);

    let result = half_float_plane(&src, 4, &mut dst, 8, 1.0, size);
    assert!(result.is_err(), "src_stride < width では Err が返るべき");
}
