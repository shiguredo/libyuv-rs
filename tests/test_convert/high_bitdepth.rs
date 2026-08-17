//! high_bitdepth サブモジュールの代表関数の単体テスト
//!
//! `src/convert/high_bitdepth.rs` に対応する。代表関数として
//! `i010_to_argb` (10bit→ARGB) と `i012_to_i420` (12bit→8bit) を選定し、
//! ground truth 方式の正常系とエラーパス (バッファ不足・stride 不足・c_int 超過) を検証する。

use std::ffi::c_int;

use shiguredo_libyuv::{
    ArgbImageMut, I010Image, I012Image, I420ImageMut, ImageSize, i010_to_argb, i012_to_i420,
};

use super::helpers;

// ============================================================
// i010_to_argb (10bit I420 → ARGB, BT.601 limited)
// ============================================================

// 正常系: i010_to_argb が既知 10bit YUV から期待 ARGB を出力すること
#[test]
fn i010_to_argb_known_colors() {
    let width = 4;
    let height = 1;
    let y = vec![64u16, 512, 900, 1023];
    let u = vec![512u16, 128];
    let v = vec![512u16, 128];
    let src = I010Image {
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

    i010_to_argb(&src, &mut dst, size).expect("10bit の変換が成功すること");

    for x in 0..width {
        let ui = if x < 2 { u[0] } else { u[1] };
        let vi = if x < 2 { v[0] } else { v[1] };
        let expected = helpers::yuv601_10_to_argb_pixel(y[x], ui, vi);
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

// 異常系: i010_to_argb の src の Y バッファ不足で Err が返ること
#[test]
fn i010_to_argb_src_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u16; width * height - 1];
    let u = vec![0u16; 4 * 2];
    let v = vec![0u16; 4 * 2];
    let src = I010Image {
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
    let result = i010_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i010_to_argb の src の Y stride 不足で Err が返ること
#[test]
fn i010_to_argb_src_y_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u16; width * height];
    let u = vec![0u16; 4 * 2];
    let v = vec![0u16; 4 * 2];
    let src = I010Image {
        y: &y,
        y_stride: width - 1,
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
    let result = i010_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("Y stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("Y stride smaller than width"),
        "Y 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i010_to_argb の src の Y stride の c_int 超過で Err が返ること
#[test]
fn i010_to_argb_src_y_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u16; width * height];
    let u = vec![0u16; 4 * 2];
    let v = vec![0u16; 4 * 2];
    let src = I010Image {
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
    let result = i010_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("Y stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// i012_to_i420 (12bit I420 → 8bit I420)
// ============================================================

// 正常系: i012_to_i420 が 12bit 値を 8bit に正しく縮小すること
#[test]
fn i012_to_i420_known_values() {
    let width = 4;
    let height = 1;
    let y = vec![0u16, 4096, 8192, 16383];
    let u = vec![0u16, 8192];
    let v = vec![0u16, 8192];
    let src = I012Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let mut y8 = vec![0u8; width * height];
    let mut u8 = vec![0u8; 2];
    let mut v8 = vec![0u8; 2];
    let mut dst = I420ImageMut {
        y: &mut y8,
        y_stride: width,
        u: &mut u8,
        u_stride: 2,
        v: &mut v8,
        v_stride: 2,
    };
    let size = ImageSize::new(width, height);

    i012_to_i420(&src, &mut dst, size).expect("12bit から 8bit への変換が成功すること");

    for x in 0..width {
        assert_eq!(
            y8[x],
            helpers::u12_to_u8(y[x]),
            "Y[{}] が期待値と一致すること",
            x
        );
    }
    for x in 0..2 {
        assert_eq!(
            u8[x],
            helpers::u12_to_u8(u[x]),
            "U[{}] が期待値と一致すること",
            x
        );
        assert_eq!(
            v8[x],
            helpers::u12_to_u8(v[x]),
            "V[{}] が期待値と一致すること",
            x
        );
    }
}

// 異常系: i012_to_i420 の dst の Y バッファ不足で Err が返ること
#[test]
fn i012_to_i420_dst_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u16; width * height];
    let u = vec![0u16; 4 * 2];
    let v = vec![0u16; 4 * 2];
    let src = I012Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut y8 = vec![0u8; width * height - 1];
    let mut u8 = vec![0u8; 4 * 2];
    let mut v8 = vec![0u8; 4 * 2];
    let mut dst = I420ImageMut {
        y: &mut y8,
        y_stride: width,
        u: &mut u8,
        u_stride: 4,
        v: &mut v8,
        v_stride: 4,
    };
    let result = i012_to_i420(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst の Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination Y buffer too small"),
        "dst の Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i012_to_i420 の dst の U stride 不足で Err が返ること
#[test]
fn i012_to_i420_dst_u_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u16; width * height];
    let u = vec![0u16; 4 * 2];
    let v = vec![0u16; 4 * 2];
    let src = I012Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut y8 = vec![0u8; width * height];
    let mut u8 = vec![0u8; 4 * 2];
    let mut v8 = vec![0u8; 4 * 2];
    let mut dst = I420ImageMut {
        y: &mut y8,
        y_stride: width,
        u: &mut u8,
        u_stride: 3, // 4 未満
        v: &mut v8,
        v_stride: 4,
    };
    let result = i012_to_i420(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst の U stride 不足では Err が返るべき");
    assert!(
        err.to_string()
            .contains("U stride smaller than chroma width"),
        "dst の U 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i012_to_i420 の dst の V stride の c_int 超過で Err が返ること
#[test]
fn i012_to_i420_dst_v_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u16; width * height];
    let u = vec![0u16; 4 * 2];
    let v = vec![0u16; 4 * 2];
    let src = I012Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut y8 = vec![0u8; width * height];
    let mut u8 = vec![0u8; 4 * 2];
    let mut v8 = vec![0u8; 4 * 2];
    let mut dst = I420ImageMut {
        y: &mut y8,
        y_stride: width,
        u: &mut u8,
        u_stride: 4,
        v: &mut v8,
        v_stride: c_int::MAX as usize + 1,
    };
    let result = i012_to_i420(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("V stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
