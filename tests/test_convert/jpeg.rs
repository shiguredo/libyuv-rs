//! jpeg サブモジュールの代表関数の単体テスト
//!
//! `src/convert/jpeg.rs` に対応する。代表関数として
//! `j420_to_argb` (BT.601 full range) と `argb_to_j420` (RGB→BT.601 full range) を選定し、
//! ground truth 方式の正常系とエラーパス (バッファ不足・stride 不足・c_int 超過) を検証する。

use std::ffi::c_int;

use shiguredo_libyuv::{
    ArgbImage, ArgbImageMut, ImageSize, J420Image, J420ImageMut, argb_to_j420, j420_to_argb,
};

use super::helpers;

// ============================================================
// j420_to_argb (BT.601 full range)
// ============================================================

// 正常系: j420_to_argb が既知 YUV から期待 ARGB を出力すること
#[test]
fn j420_to_argb_known_colors() {
    // グレーのみでなく色差を持つ値を含め、色変換係数 (YG / VR 等) の誤りを検出できるようにする
    let width = 4;
    let height = 1;
    let y = vec![0u8, 81, 145, 255];
    let u = vec![90u8, 240];
    let v = vec![240u8, 90];
    let src = J420Image {
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

    j420_to_argb(&src, &mut dst, size).expect("BT.601 full range の変換が成功すること");

    for x in 0..width {
        let ui = if x < 2 { u[0] } else { u[1] };
        let vi = if x < 2 { v[0] } else { v[1] };
        let expected = helpers::yuv_jpeg_to_argb_pixel(y[x], ui, vi);
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

// 異常系: j420_to_argb の src の U バッファ不足で Err が返ること
#[test]
fn j420_to_argb_src_u_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2 - 1];
    let v = vec![0u8; 4 * 2];
    let src = J420Image {
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
    let result = j420_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("U バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source U buffer too small"),
        "U 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: j420_to_argb の src の V stride 不足で Err が返ること
#[test]
fn j420_to_argb_src_v_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = J420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 3, // 4 未満
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = j420_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("V stride 不足では Err が返るべき");
    assert!(
        err.to_string()
            .contains("V stride smaller than chroma width"),
        "V 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: j420_to_argb の src の V stride の c_int 超過で Err が返ること
#[test]
fn j420_to_argb_src_v_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = J420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: c_int::MAX as usize + 1,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = j420_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("V stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// argb_to_j420 (ARGB → BT.601 full range YUV)
// ============================================================

// 正常系: argb_to_j420 が既知 ARGB から期待 YUV を出力すること
#[test]
fn argb_to_j420_known_colors() {
    let width = 4;
    let height = 2;
    let data: Vec<u8> = (0..(width * height * 4))
        .map(|i| (i * 11 % 256) as u8)
        .collect();
    let src = ArgbImage {
        data: &data,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut u = vec![0u8; (width / 2) * (height / 2)];
    let mut v = vec![0u8; (width / 2) * (height / 2)];
    let mut dst = J420ImageMut {
        y: &mut y,
        y_stride: width,
        u: &mut u,
        u_stride: width / 2,
        v: &mut v,
        v_stride: width / 2,
    };
    let size = ImageSize::new(width, height);

    argb_to_j420(&src, &mut dst, size).expect("ARGB から BT.601 full range への変換が成功すること");

    // Y はピクセルごと
    for i in 0..(width * height) {
        let argb = [
            data[i * 4],
            data[i * 4 + 1],
            data[i * 4 + 2],
            data[i * 4 + 3],
        ];
        let expected = helpers::argb_to_yuv_jpeg_pixel(argb);
        assert_eq!(y[i], expected.0, "Y[{}] が期待値と一致すること", i);
    }
    // U/V は 2x2 ブロックの平均
    for i in 0..(width / 2) * (height / 2) {
        let x = i % (width / 2);
        let yy = i / (width / 2);
        let block = [
            [
                data[(yy * 2 * width + x * 2) * 4],
                data[(yy * 2 * width + x * 2) * 4 + 1],
                data[(yy * 2 * width + x * 2) * 4 + 2],
                data[(yy * 2 * width + x * 2) * 4 + 3],
            ],
            [
                data[(yy * 2 * width + x * 2 + 1) * 4],
                data[(yy * 2 * width + x * 2 + 1) * 4 + 1],
                data[(yy * 2 * width + x * 2 + 1) * 4 + 2],
                data[(yy * 2 * width + x * 2 + 1) * 4 + 3],
            ],
            [
                data[((yy * 2 + 1) * width + x * 2) * 4],
                data[((yy * 2 + 1) * width + x * 2) * 4 + 1],
                data[((yy * 2 + 1) * width + x * 2) * 4 + 2],
                data[((yy * 2 + 1) * width + x * 2) * 4 + 3],
            ],
            [
                data[((yy * 2 + 1) * width + x * 2 + 1) * 4],
                data[((yy * 2 + 1) * width + x * 2 + 1) * 4 + 1],
                data[((yy * 2 + 1) * width + x * 2 + 1) * 4 + 2],
                data[((yy * 2 + 1) * width + x * 2 + 1) * 4 + 3],
            ],
        ];
        let expected = helpers::argb_block_to_yuv_jpeg_uv(block);
        assert_eq!(u[i], expected.0, "U[{}] が期待値と一致すること", i);
        assert_eq!(v[i], expected.1, "V[{}] が期待値と一致すること", i);
    }
}

// 異常系: argb_to_j420 の src のバッファ不足で Err が返ること
#[test]
fn argb_to_j420_src_buffer_too_small() {
    let width = 8;
    let height = 4;
    let data = vec![0u8; width * height * 4 - 1];
    let src = ArgbImage {
        data: &data,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut u = vec![0u8; 4 * 2];
    let mut v = vec![0u8; 4 * 2];
    let mut dst = J420ImageMut {
        y: &mut y,
        y_stride: width,
        u: &mut u,
        u_stride: 4,
        v: &mut v,
        v_stride: 4,
    };
    let result = argb_to_j420(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("src バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source buffer too small"),
        "src 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: argb_to_j420 の src の stride 不足で Err が返ること
#[test]
fn argb_to_j420_src_stride_too_small() {
    let width = 8;
    let height = 4;
    let data = vec![0u8; width * height * 4];
    let src = ArgbImage {
        data: &data,
        stride: width * 4 - 1,
    };
    let mut y = vec![0u8; width * height];
    let mut u = vec![0u8; 4 * 2];
    let mut v = vec![0u8; 4 * 2];
    let mut dst = J420ImageMut {
        y: &mut y,
        y_stride: width,
        u: &mut u,
        u_stride: 4,
        v: &mut v,
        v_stride: 4,
    };
    let result = argb_to_j420(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("src stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "src 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: argb_to_j420 の src の stride の c_int 超過で Err が返ること
#[test]
fn argb_to_j420_src_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let data = vec![0u8; width * height * 4];
    let src = ArgbImage {
        data: &data,
        stride: c_int::MAX as usize + 1,
    };
    let mut y = vec![0u8; width * height];
    let mut u = vec![0u8; 4 * 2];
    let mut v = vec![0u8; 4 * 2];
    let mut dst = J420ImageMut {
        y: &mut y,
        y_stride: width,
        u: &mut u,
        u_stride: 4,
        v: &mut v,
        v_stride: 4,
    };
    let result = argb_to_j420(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "src 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
