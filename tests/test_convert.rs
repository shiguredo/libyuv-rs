//! フォーマット変換 API の単体テスト
//!
//! エラーパス・境界値を検証する。正常系のプロパティ検証は PBT でカバーする。

use shiguredo_libyuv::{
    ArgbImageMut, I420Image, I420ImageMut, ImageSize, Nv12Image, i420_to_argb, nv12_to_i420,
};

// 異常系: i420_to_argb のバッファ不足で Err が返ること
#[test]
fn i420_to_argb_buffer_too_small() {
    let y = vec![0u8; 64];
    let u = vec![0u8; 16];
    let v = vec![0u8; 16];
    let src = I420Image {
        y: &y,
        y_stride: 8,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    // dst バッファが不足 (8x8 ARGB = 256 バイト必要だが 100 しか用意しない)
    let mut data = vec![0u8; 100];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: 32,
    };
    let size = ImageSize::new(8, 8);

    let result = i420_to_argb(&src, &mut dst, size);
    assert!(result.is_err(), "バッファ不足では Err が返るべき");
}

// 異常系: i420_to_argb の stride 不足で Err が返ること
#[test]
fn i420_to_argb_stride_too_small() {
    let y = vec![0u8; 64];
    let u = vec![0u8; 16];
    let v = vec![0u8; 16];
    let src = I420Image {
        y: &y,
        y_stride: 4, // width=8 に対して stride=4 は不足
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut data = vec![0u8; 256];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: 32,
    };
    let size = ImageSize::new(8, 8);

    let result = i420_to_argb(&src, &mut dst, size);
    assert!(result.is_err(), "stride 不足では Err が返るべき");
}

// 異常系: nv12_to_i420 のバッファ不足で Err が返ること
#[test]
fn nv12_to_i420_buffer_too_small() {
    let y = vec![0u8; 64];
    let uv = vec![0u8; 32];
    let src = Nv12Image {
        y: &y,
        y_stride: 8,
        uv: &uv,
        uv_stride: 8,
    };
    // dst の Y バッファが不足
    let mut y_dst = vec![0u8; 10];
    let mut u_dst = vec![0u8; 16];
    let mut v_dst = vec![0u8; 16];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: 8,
        u: &mut u_dst,
        u_stride: 4,
        v: &mut v_dst,
        v_stride: 4,
    };
    let size = ImageSize::new(8, 8);

    let result = nv12_to_i420(&src, &mut dst, size);
    assert!(result.is_err(), "バッファ不足では Err が返るべき");
}

// 異常系: nv12_to_i420 の stride 不足で Err が返ること
#[test]
fn nv12_to_i420_stride_too_small() {
    let y = vec![0u8; 64];
    let uv = vec![0u8; 32];
    let src = Nv12Image {
        y: &y,
        y_stride: 4, // width=8 に対して stride=4 は不足
        uv: &uv,
        uv_stride: 8,
    };
    let mut y_dst = vec![0u8; 64];
    let mut u_dst = vec![0u8; 16];
    let mut v_dst = vec![0u8; 16];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: 8,
        u: &mut u_dst,
        u_stride: 4,
        v: &mut v_dst,
        v_stride: 4,
    };
    let size = ImageSize::new(8, 8);

    let result = nv12_to_i420(&src, &mut dst, size);
    assert!(result.is_err(), "stride 不足では Err が返るべき");
}
