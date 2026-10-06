//! packed サブモジュールの代表関数の単体テスト
//!
//! `src/convert/packed.rs` に対応する。代表関数として
//! `rgb565_to_argb` / `argb_to_rgb565` (ビットパック変換) と `p010_to_nv12` (10bit→8bit) を
//! 選定し、ground truth 方式の正常系とエラーパス (バッファ不足・stride 不足・c_int 超過) を検証する。

use std::ffi::c_int;

use shiguredo_libyuv::{
    ArgbImage, ArgbImageMut, ImageSize, Nv12ImageMut, P010Image, Rgb565Image, Rgb565ImageMut,
    argb_to_rgb565, p010_to_nv12, rgb565_to_argb,
};

use super::helpers;

// ============================================================
// rgb565_to_argb
// ============================================================

// 正常系: rgb565_to_argb が既知 RGB565 から期待 ARGB を出力すること
#[test]
fn rgb565_to_argb_known_colors() {
    let width = 4;
    let height = 1;
    let rgb565 = vec![
        0x00u8, 0x00, // 黒
        0x00, 0xf8, // 赤
        0xe0, 0x07, // 緑
        0xff, 0xff, // 白
    ];
    let src = Rgb565Image {
        data: &rgb565,
        stride: width * 2,
    };
    let mut argb = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut argb,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    rgb565_to_argb(&src, &mut dst, size).expect("RGB565 から ARGB への変換が成功すること");

    for x in 0..width {
        let expected = helpers::rgb565_to_argb_pixel([rgb565[x * 2], rgb565[x * 2 + 1]]);
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

// 異常系: rgb565_to_argb の src のバッファ不足で Err が返ること
#[test]
fn rgb565_to_argb_src_buffer_too_small() {
    let width = 8;
    let height = 4;
    let rgb565 = vec![0u8; width * height * 2 - 1];
    let src = Rgb565Image {
        data: &rgb565,
        stride: width * 2,
    };
    let mut argb = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut argb,
        stride: width * 4,
    };
    let result = rgb565_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("src バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source buffer too small"),
        "src 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: rgb565_to_argb の dst の stride 不足で Err が返ること
#[test]
fn rgb565_to_argb_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let rgb565 = vec![0u8; width * height * 2];
    let src = Rgb565Image {
        data: &rgb565,
        stride: width * 2,
    };
    let mut argb = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut argb,
        stride: width * 4 - 1,
    };
    let result = rgb565_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: rgb565_to_argb の dst の stride の c_int 超過で Err が返ること
#[test]
fn rgb565_to_argb_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let rgb565 = vec![0u8; width * height * 2];
    let src = Rgb565Image {
        data: &rgb565,
        stride: width * 2,
    };
    let mut argb = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut argb,
        stride: c_int::MAX as usize + 1,
    };
    let result = rgb565_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// argb_to_rgb565
// ============================================================

// 正常系: argb_to_rgb565 が既知 ARGB から期待 RGB565 を出力すること
#[test]
fn argb_to_rgb565_known_colors() {
    let width = 4;
    let height = 1;
    let argb = vec![
        0u8, 0, 0, 255, // 黒
        0, 0, 255, 255, // 赤
        0, 255, 0, 255, // 緑
        255, 255, 255, 255, // 白
    ];
    let src = ArgbImage {
        data: &argb,
        stride: width * 4,
    };
    let mut rgb565 = vec![0u8; width * height * 2];
    let mut dst = Rgb565ImageMut {
        data: &mut rgb565,
        stride: width * 2,
    };
    let size = ImageSize::new(width, height);

    argb_to_rgb565(&src, &mut dst, size).expect("ARGB から RGB565 への変換が成功すること");

    for x in 0..width {
        let px = [
            argb[x * 4],
            argb[x * 4 + 1],
            argb[x * 4 + 2],
            argb[x * 4 + 3],
        ];
        let expected = helpers::argb_to_rgb565_pixel(px);
        let actual = [rgb565[x * 2], rgb565[x * 2 + 1]];
        assert_eq!(
            actual, expected,
            "x={} の画素が期待値と一致すること: 実測 {:02x?} 期待 {:02x?}",
            x, actual, expected
        );
    }
}

// 異常系: argb_to_rgb565 の dst のバッファ不足で Err が返ること
#[test]
fn argb_to_rgb565_dst_buffer_too_small() {
    let width = 8;
    let height = 4;
    let argb = vec![0u8; width * height * 4];
    let src = ArgbImage {
        data: &argb,
        stride: width * 4,
    };
    let mut rgb565 = vec![0u8; width * height * 2 - 1];
    let mut dst = Rgb565ImageMut {
        data: &mut rgb565,
        stride: width * 2,
    };
    let result = argb_to_rgb565(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination buffer too small"),
        "dst 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: argb_to_rgb565 の dst の stride 不足で Err が返ること
#[test]
fn argb_to_rgb565_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let argb = vec![0u8; width * height * 4];
    let src = ArgbImage {
        data: &argb,
        stride: width * 4,
    };
    let mut rgb565 = vec![0u8; width * height * 2];
    let mut dst = Rgb565ImageMut {
        data: &mut rgb565,
        stride: width * 2 - 1,
    };
    let result = argb_to_rgb565(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: argb_to_rgb565 の dst の stride の c_int 超過で Err が返ること
#[test]
fn argb_to_rgb565_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let argb = vec![0u8; width * height * 4];
    let src = ArgbImage {
        data: &argb,
        stride: width * 4,
    };
    let mut rgb565 = vec![0u8; width * height * 2];
    let mut dst = Rgb565ImageMut {
        data: &mut rgb565,
        stride: c_int::MAX as usize + 1,
    };
    let result = argb_to_rgb565(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// p010_to_nv12 (10bit → 8bit)
// ============================================================

// 正常系: p010_to_nv12 が 16bit 値を 8bit に正しく縮小すること
#[test]
fn p010_to_nv12_known_values() {
    let width = 4;
    let height = 2;
    let y: Vec<u16> = (0..(width * height)).map(|i| (i * 100) as u16).collect();
    let uv: Vec<u16> = (0..(width)).map(|i| (i * 200) as u16).collect();
    let src = P010Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let mut y8 = vec![0u8; width * height];
    let mut uv8 = vec![0u8; width];
    let mut dst = Nv12ImageMut {
        y: &mut y8,
        y_stride: width,
        uv: &mut uv8,
        uv_stride: width,
    };
    let size = ImageSize::new(width, height);

    p010_to_nv12(&src, &mut dst, size).expect("P010 から NV12 への変換が成功すること");

    for i in 0..(width * height) {
        assert_eq!(
            y8[i],
            helpers::u16_to_u8(y[i]),
            "Y[{}] が期待値と一致すること",
            i
        );
    }
    for i in 0..(width) {
        assert_eq!(
            uv8[i],
            helpers::u16_to_u8(uv[i]),
            "UV[{}] が期待値と一致すること",
            i
        );
    }
}

// 異常系: p010_to_nv12 の src の Y バッファ不足で Err が返ること
#[test]
fn p010_to_nv12_src_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u16; width * height - 1];
    let uv = vec![0u16; width * 2];
    let src = P010Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let mut y8 = vec![0u8; width * height];
    let mut uv8 = vec![0u8; width * 2];
    let mut dst = Nv12ImageMut {
        y: &mut y8,
        y_stride: width,
        uv: &mut uv8,
        uv_stride: width,
    };
    let result = p010_to_nv12(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: p010_to_nv12 の dst の UV バッファ不足で Err が返ること
#[test]
fn p010_to_nv12_dst_uv_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u16; width * height];
    let uv = vec![0u16; width * 2];
    let src = P010Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let mut y8 = vec![0u8; width * height];
    let mut uv8 = vec![0u8; width * 2 - 1];
    let mut dst = Nv12ImageMut {
        y: &mut y8,
        y_stride: width,
        uv: &mut uv8,
        uv_stride: width,
    };
    let result = p010_to_nv12(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("UV バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination UV buffer too small"),
        "UV 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: p010_to_nv12 の dst の UV stride の c_int 超過で Err が返ること
#[test]
fn p010_to_nv12_dst_uv_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u16; width * height];
    let uv = vec![0u16; width * 2];
    let src = P010Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let mut y8 = vec![0u8; width * height];
    let mut uv8 = vec![0u8; width * 2];
    let mut dst = Nv12ImageMut {
        y: &mut y8,
        y_stride: width,
        uv: &mut uv8,
        uv_stride: c_int::MAX as usize + 1,
    };
    let result = p010_to_nv12(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("UV stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
