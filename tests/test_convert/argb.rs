//! argb サブモジュールの代表関数の単体テスト
//!
//! `src/convert/argb.rs` に対応する。代表関数として
//! `ar30_to_argb` / `argb_to_ar30` (10bit パック変換) と `i420_to_rgba` (色変換 + チャンネル入替) を
//! 選定し、ground truth 方式の正常系とエラーパス (バッファ不足・stride 不足・c_int 超過) を検証する。

use std::ffi::c_int;

use shiguredo_libyuv::{
    Ar30Image, Ar30ImageMut, ArgbImageMut, I420Image, ImageSize, RgbaImageMut, ar30_to_argb,
    argb_to_ar30, i420_to_rgba,
};

use super::helpers;

// ============================================================
// ar30_to_argb
// ============================================================

// 正常系: ar30_to_argb が既知 AR30 から期待 ARGB を出力すること
#[test]
fn ar30_to_argb_known_values() {
    let width = 4;
    let height = 1;
    // リトルエンディアンの AR30 (A2bit R10 G10 B10)
    let ar30 = vec![
        0x00u8, 0x00, 0x00, 0xc0, // 黒 A=3
        0x00, 0x00, 0x03, 0xc0, // G=0x300
        0x00, 0x0c, 0x00, 0xc0, // R=0x300
        0xff, 0x03, 0x00, 0xc0, // B=0x3ff, G=0x300
    ];
    let src = Ar30Image {
        data: &ar30,
        stride: width * 4,
    };
    let mut argb = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut argb,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    ar30_to_argb(&src, &mut dst, size).expect("AR30 から ARGB への変換が成功すること");

    for x in 0..width {
        let le = [
            ar30[x * 4],
            ar30[x * 4 + 1],
            ar30[x * 4 + 2],
            ar30[x * 4 + 3],
        ];
        let expected = helpers::ar30_to_argb_pixel(le);
        let actual = [
            argb[x * 4],
            argb[x * 4 + 1],
            argb[x * 4 + 2],
            argb[x * 4 + 3],
        ];
        assert_eq!(
            actual, expected,
            "x={} の画素が期待値と一致すること: 実測 {:?} 期待 {:?}",
            x, actual, expected
        );
    }
}

// 異常系: ar30_to_argb の src のバッファ不足で Err が返ること
#[test]
fn ar30_to_argb_src_buffer_too_small() {
    let width = 8;
    let height = 4;
    let ar30 = vec![0u8; width * height * 4 - 1];
    let src = Ar30Image {
        data: &ar30,
        stride: width * 4,
    };
    let mut argb = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut argb,
        stride: width * 4,
    };
    let result = ar30_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("src バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source buffer too small"),
        "src 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: ar30_to_argb の dst の stride 不足で Err が返ること
#[test]
fn ar30_to_argb_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let ar30 = vec![0u8; width * height * 4];
    let src = Ar30Image {
        data: &ar30,
        stride: width * 4,
    };
    let mut argb = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut argb,
        stride: width * 4 - 1,
    };
    let result = ar30_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: ar30_to_argb の dst の stride の c_int 超過で Err が返ること
#[test]
fn ar30_to_argb_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let ar30 = vec![0u8; width * height * 4];
    let src = Ar30Image {
        data: &ar30,
        stride: width * 4,
    };
    let mut argb = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut argb,
        stride: c_int::MAX as usize + 1,
    };
    let result = ar30_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// argb_to_ar30
// ============================================================

// 正常系: argb_to_ar30 が既知 ARGB から期待 AR30 を出力すること
#[test]
fn argb_to_ar30_known_values() {
    let width = 4;
    let height = 1;
    let argb = vec![
        0u8, 0, 0, 255, // 黒
        0, 0, 255, 255, // 赤
        0, 255, 0, 255, // 緑
        255, 255, 255, 255, // 白
    ];
    // argb_to_ar30 は src が ArgbImage で受ける
    let src = shiguredo_libyuv::ArgbImage {
        data: &argb,
        stride: width * 4,
    };
    let mut ar30 = vec![0u8; width * height * 4];
    let mut dst = Ar30ImageMut {
        data: &mut ar30,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    argb_to_ar30(&src, &mut dst, size).expect("ARGB から AR30 への変換が成功すること");

    for x in 0..width {
        let px = [
            argb[x * 4],
            argb[x * 4 + 1],
            argb[x * 4 + 2],
            argb[x * 4 + 3],
        ];
        let expected = helpers::argb_to_ar30_pixel(px);
        let actual = [
            ar30[x * 4],
            ar30[x * 4 + 1],
            ar30[x * 4 + 2],
            ar30[x * 4 + 3],
        ];
        assert_eq!(
            actual, expected,
            "x={} の画素が期待値と一致すること: 実測 {:02x?} 期待 {:02x?}",
            x, actual, expected
        );
    }
}

// 異常系: argb_to_ar30 の src のバッファ不足で Err が返ること
#[test]
fn argb_to_ar30_src_buffer_too_small() {
    let width = 8;
    let height = 4;
    let argb = vec![0u8; width * height * 4 - 1];
    let src = shiguredo_libyuv::ArgbImage {
        data: &argb,
        stride: width * 4,
    };
    let mut ar30 = vec![0u8; width * height * 4];
    let mut dst = Ar30ImageMut {
        data: &mut ar30,
        stride: width * 4,
    };
    let result = argb_to_ar30(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("src バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source buffer too small"),
        "src 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: argb_to_ar30 の dst の stride 不足で Err が返ること
#[test]
fn argb_to_ar30_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let argb = vec![0u8; width * height * 4];
    let src = shiguredo_libyuv::ArgbImage {
        data: &argb,
        stride: width * 4,
    };
    let mut ar30 = vec![0u8; width * height * 4];
    let mut dst = Ar30ImageMut {
        data: &mut ar30,
        stride: width * 4 - 1,
    };
    let result = argb_to_ar30(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: argb_to_ar30 の dst の stride の c_int 超過で Err が返ること
#[test]
fn argb_to_ar30_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let argb = vec![0u8; width * height * 4];
    let src = shiguredo_libyuv::ArgbImage {
        data: &argb,
        stride: width * 4,
    };
    let mut ar30 = vec![0u8; width * height * 4];
    let mut dst = Ar30ImageMut {
        data: &mut ar30,
        stride: c_int::MAX as usize + 1,
    };
    let result = argb_to_ar30(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// i420_to_rgba
// ============================================================

// 正常系: i420_to_rgba が既知 YUV から期待 RGBA を出力すること
///
/// RGBA のメモリ順は A,B,G,R (ARGB の 4 バイトをローテートした順)。
#[test]
fn i420_to_rgba_known_colors() {
    let width = 4;
    let height = 1;
    let y = vec![16u8, 81, 145, 235];
    let u = vec![128u8, 90];
    let v = vec![128u8, 240];
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let mut rgba = vec![0u8; width * height * 4];
    let mut dst = RgbaImageMut {
        data: &mut rgba,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    i420_to_rgba(&src, &mut dst, size).expect("I420 から RGBA への変換が成功すること");

    for x in 0..width {
        let ui = if x < 2 { u[0] } else { u[1] };
        let vi = if x < 2 { v[0] } else { v[1] };
        let argb = helpers::yuv601_to_argb_pixel(y[x], ui, vi);
        let expected = helpers::argb_to_rgba_pixel(argb);
        let actual = [
            rgba[x * 4],
            rgba[x * 4 + 1],
            rgba[x * 4 + 2],
            rgba[x * 4 + 3],
        ];
        assert_eq!(
            actual, expected,
            "x={} の画素が期待値と一致すること: 実測 {:?} 期待 {:?}",
            x, actual, expected
        );
    }
}

// 異常系: i420_to_rgba の src の U バッファ不足で Err が返ること
#[test]
fn i420_to_rgba_src_u_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2 - 1];
    let v = vec![0u8; 4 * 2];
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut rgba = vec![0u8; width * height * 4];
    let mut dst = RgbaImageMut {
        data: &mut rgba,
        stride: width * 4,
    };
    let result = i420_to_rgba(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("U バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source U buffer too small"),
        "U 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i420_to_rgba の dst の stride 不足で Err が返ること
#[test]
fn i420_to_rgba_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut rgba = vec![0u8; width * height * 4];
    let mut dst = RgbaImageMut {
        data: &mut rgba,
        stride: width * 4 - 1,
    };
    let result = i420_to_rgba(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i420_to_rgba の dst の stride の c_int 超過で Err が返ること
#[test]
fn i420_to_rgba_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut rgba = vec![0u8; width * height * 4];
    let mut dst = RgbaImageMut {
        data: &mut rgba,
        stride: c_int::MAX as usize + 1,
    };
    let result = i420_to_rgba(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
