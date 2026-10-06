//! スケーリング API の単体テスト
//!
//! エラーパス・境界値を検証する。正常系のプロパティ検証は PBT でカバーする。

use shiguredo_libyuv::{FilterMode, I420Image, I420ImageMut, ImageSize, i420_scale};

// 異常系: i420_scale のバッファ不足で Err が返ること
#[test]
fn i420_scale_buffer_too_small() {
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
    // dst バッファが不足
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

    let result = i420_scale(&src, size, &mut dst, size, FilterMode::None);
    assert!(result.is_err(), "バッファ不足では Err が返るべき");
}

// 異常系: i420_scale の stride 不足で Err が返ること
#[test]
fn i420_scale_stride_too_small() {
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

    let result = i420_scale(&src, size, &mut dst, size, FilterMode::None);
    assert!(result.is_err(), "stride 不足では Err が返るべき");
}
