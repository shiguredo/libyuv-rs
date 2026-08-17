//! subsampling サブモジュールの代表関数の単体テスト
//!
//! `src/convert/subsampling.rs` に対応する。代表関数として
//! `i444_to_rgb24`・`i422_to_rgb24` (色変換) を選定し、
//! ground truth 方式の正常系とエラーパス (バッファ不足・stride 不足・c_int 超過) を検証する。
//! `i422_to_i444` のアップサンプリングは libyuv のスケーリング補間が SIMD 実装に依存し、
//! ground truth をプラットフォーム非依存で固定できないため選定しない。

use std::ffi::c_int;

use shiguredo_libyuv::{
    I422Image, I444Image, ImageSize, Rgb24ImageMut, i422_to_rgb24, i444_to_rgb24,
};

use super::helpers;

// ============================================================
// i444_to_rgb24
// ============================================================

// 正常系: i444_to_rgb24 が既知 YUV から期待 RGB24 を出力すること
///
/// libyuv の RGB24 出力はメモリ順 B,G,R (I444ToRGB24Row_C が YuvPixel の
/// b/g/r 出力を先頭から書くため)。
#[test]
fn i444_to_rgb24_known_colors() {
    let width = 2;
    let height = 1;
    let y = vec![81u8, 145];
    let u = vec![90u8, 54];
    let v = vec![240u8, 34];
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let mut rgb24 = vec![0u8; width * height * 3];
    let mut dst = Rgb24ImageMut {
        data: &mut rgb24,
        stride: width * 3,
    };
    let size = ImageSize::new(width, height);

    i444_to_rgb24(&src, &mut dst, size).expect("I444 から RGB24 への変換が成功すること");

    for x in 0..width {
        let argb = helpers::yuv601_to_argb_pixel(y[x], u[x], v[x]);
        let expected = [argb[0], argb[1], argb[2]]; // B,G,R
        let actual = [rgb24[x * 3], rgb24[x * 3 + 1], rgb24[x * 3 + 2]];
        assert_eq!(
            actual, expected,
            "x={} の画素が期待値と一致すること: 実測 {:?} 期待 {:?}",
            x, actual, expected
        );
    }
}

// 異常系: i444_to_rgb24 の src の Y バッファ不足で Err が返ること
#[test]
fn i444_to_rgb24_src_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height - 1];
    let u = vec![0u8; width * height];
    let v = vec![0u8; width * height];
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let mut rgb24 = vec![0u8; width * height * 3];
    let mut dst = Rgb24ImageMut {
        data: &mut rgb24,
        stride: width * 3,
    };
    let result = i444_to_rgb24(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i444_to_rgb24 の dst の stride 不足で Err が返ること
#[test]
fn i444_to_rgb24_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; width * height];
    let v = vec![0u8; width * height];
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let mut rgb24 = vec![0u8; width * height * 3];
    let mut dst = Rgb24ImageMut {
        data: &mut rgb24,
        stride: width * 3 - 1,
    };
    let result = i444_to_rgb24(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i444_to_rgb24 の dst の stride の c_int 超過で Err が返ること
#[test]
fn i444_to_rgb24_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; width * height];
    let v = vec![0u8; width * height];
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let mut rgb24 = vec![0u8; width * height * 3];
    let mut dst = Rgb24ImageMut {
        data: &mut rgb24,
        stride: c_int::MAX as usize + 1,
    };
    let result = i444_to_rgb24(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// i422_to_rgb24
// ============================================================

// 正常系: i422_to_rgb24 が既知 YUV から期待 RGB24 を出力すること
///
/// I422 は 4:2:2 のため、U/V は 2 画素ごとに共有される。
#[test]
fn i422_to_rgb24_known_colors() {
    let width = 4;
    let height = 1;
    let y = vec![81u8, 145, 81, 145];
    let u = vec![90u8, 90];
    let v = vec![240u8, 240];
    let src = I422Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let mut rgb24 = vec![0u8; width * height * 3];
    let mut dst = Rgb24ImageMut {
        data: &mut rgb24,
        stride: width * 3,
    };
    let size = ImageSize::new(width, height);

    i422_to_rgb24(&src, &mut dst, size).expect("I422 から RGB24 への変換が成功すること");

    for x in 0..width {
        let ui = u[x / 2];
        let vi = v[x / 2];
        let argb = helpers::yuv601_to_argb_pixel(y[x], ui, vi);
        let expected = [argb[0], argb[1], argb[2]]; // B,G,R
        let actual = [rgb24[x * 3], rgb24[x * 3 + 1], rgb24[x * 3 + 2]];
        assert_eq!(
            actual, expected,
            "x={} の画素が期待値と一致すること: 実測 {:?} 期待 {:?}",
            x, actual, expected
        );
    }
}

// 異常系: i422_to_rgb24 の src の U バッファ不足で Err が返ること
#[test]
fn i422_to_rgb24_src_u_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 4 - 1];
    let v = vec![0u8; 4 * 4];
    let src = I422Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut rgb24 = vec![0u8; width * height * 3];
    let mut dst = Rgb24ImageMut {
        data: &mut rgb24,
        stride: width * 3,
    };
    let result = i422_to_rgb24(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("U バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source U buffer too small"),
        "U 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i422_to_rgb24 の src の V stride 不足で Err が返ること
#[test]
fn i422_to_rgb24_src_v_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 4];
    let v = vec![0u8; 4 * 4];
    let src = I422Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 3, // 4 未満
    };
    let mut rgb24 = vec![0u8; width * height * 3];
    let mut dst = Rgb24ImageMut {
        data: &mut rgb24,
        stride: width * 3,
    };
    let result = i422_to_rgb24(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("V stride 不足では Err が返るべき");
    assert!(
        err.to_string()
            .contains("V stride smaller than chroma width"),
        "V 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i422_to_rgb24 の src の V stride の c_int 超過で Err が返ること
#[test]
fn i422_to_rgb24_src_v_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 4];
    let v = vec![0u8; 4 * 4];
    let src = I422Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: c_int::MAX as usize + 1,
    };
    let mut rgb24 = vec![0u8; width * height * 3];
    let mut dst = Rgb24ImageMut {
        data: &mut rgb24,
        stride: width * 3,
    };
    let result = i422_to_rgb24(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("V stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
