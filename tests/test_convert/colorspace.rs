//! colorspace サブモジュールの代表関数の単体テスト
//!
//! `src/convert/colorspace.rs` に対応する。代表関数として
//! `h420_to_argb` (BT.709 limited) と `u420_to_argb` (BT.2020 limited) を選定し、
//! ground truth 方式の正常系とエラーパス (バッファ不足・stride 不足・c_int 超過) を検証する。

use std::ffi::c_int;

use shiguredo_libyuv::{ArgbImageMut, H420Image, ImageSize, U420Image, h420_to_argb, u420_to_argb};

use super::helpers;

// ============================================================
// h420_to_argb (BT.709 limited)
// ============================================================

// 正常系: h420_to_argb が既知 YUV から期待 ARGB を出力すること
#[test]
fn h420_to_argb_known_colors() {
    // 4x1 の H420 画像。U/V は 2 画素ごとに共有される (4:2:0)。
    // グレーのみでなく色差を持つ値を含め、色変換係数 (YG / VR 等) の誤りを検出できるようにする
    let width = 4;
    let height = 1;
    let y = vec![16u8, 81, 145, 235];
    let u = vec![90u8, 240];
    let v = vec![240u8, 90];
    let src = H420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    h420_to_argb(&src, &mut dst, size).expect("BT.709 の変換が成功すること");

    for x in 0..width {
        let ui = if x < 2 { u[0] } else { u[1] };
        let vi = if x < 2 { v[0] } else { v[1] };
        let expected = helpers::yuv_h709_to_argb_pixel(y[x], ui, vi);
        let actual = [
            data[x * 4],
            data[x * 4 + 1],
            data[x * 4 + 2],
            data[x * 4 + 3],
        ];
        assert_eq!(
            actual, expected,
            "x={} の画素が期待値と一致すること: 実測 {:?} 期待 {:?}",
            x, actual, expected
        );
    }
}

// 異常系: h420_to_argb の src の Y バッファ不足で Err が返ること
#[test]
fn h420_to_argb_src_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height - 1]; // 1 バイト不足
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = H420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = h420_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: h420_to_argb の src の Y stride 不足で Err が返ること
#[test]
fn h420_to_argb_src_y_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = H420Image {
        y: &y,
        y_stride: width - 1, // width 未満
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = h420_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("Y stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("Y stride smaller than width"),
        "Y 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: h420_to_argb の src の Y stride の c_int 超過で Err が返ること
#[test]
fn h420_to_argb_src_y_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = H420Image {
        y: &y,
        y_stride: c_int::MAX as usize + 1,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = h420_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("Y stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// u420_to_argb (BT.2020 limited)
// ============================================================

// 正常系: u420_to_argb が既知 YUV から期待 ARGB を出力すること
#[test]
fn u420_to_argb_known_colors() {
    // グレーのみでなく色差を持つ値を含め、色変換係数 (YG / VR 等) の誤りを検出できるようにする
    let width = 4;
    let height = 1;
    let y = vec![16u8, 81, 145, 235];
    let u = vec![90u8, 240];
    let v = vec![240u8, 90];
    let src = U420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    u420_to_argb(&src, &mut dst, size).expect("BT.2020 の変換が成功すること");

    for x in 0..width {
        let ui = if x < 2 { u[0] } else { u[1] };
        let vi = if x < 2 { v[0] } else { v[1] };
        let expected = helpers::yuv_2020_to_argb_pixel(y[x], ui, vi);
        let actual = [
            data[x * 4],
            data[x * 4 + 1],
            data[x * 4 + 2],
            data[x * 4 + 3],
        ];
        assert_eq!(
            actual, expected,
            "x={} の画素が期待値と一致すること: 実測 {:?} 期待 {:?}",
            x, actual, expected
        );
    }
}

// 異常系: u420_to_argb の src の Y バッファ不足で Err が返ること
#[test]
fn u420_to_argb_src_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height - 1];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = U420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = u420_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: u420_to_argb の dst の stride 不足で Err が返ること
#[test]
fn u420_to_argb_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = U420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4 - 1, // width * 4 未満
    };
    let result = u420_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: u420_to_argb の dst の stride の c_int 超過で Err が返ること
#[test]
fn u420_to_argb_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = U420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: c_int::MAX as usize + 1,
    };
    let result = u420_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
