//! 回転 API の単体テスト
//!
//! エラーパス・境界値を検証する。正常系のプロパティ検証は PBT でカバーする。

use shiguredo_libyuv::{
    Android420Image, I420Image, I420ImageMut, ImageSize, RotationMode, android420_to_i420_rotate,
    i420_rotate,
};

// 異常系: i420_rotate のバッファ不足で Err が返ること
#[test]
fn i420_rotate_buffer_too_small() {
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

    let result = i420_rotate(&src, size, &mut dst, size, RotationMode::Rotate180);
    assert!(result.is_err(), "バッファ不足では Err が返るべき");
}

// 異常系: i420_rotate の stride 不足で Err が返ること
#[test]
fn i420_rotate_stride_too_small() {
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

    let result = i420_rotate(&src, size, &mut dst, size, RotationMode::Rotate180);
    assert!(result.is_err(), "stride 不足では Err が返るべき");
}

// 異常系: android420_to_i420_rotate が pixel_stride_uv の 1 / 2 以外を Err にすること
#[test]
fn android420_to_i420_rotate_pixel_stride_uv_invalid() {
    let y = vec![0u8; 8 * 4];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = Android420Image {
        y: &y,
        y_stride: 8,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut y_dst = vec![0u8; 8 * 4];
    let mut u_dst = vec![0u8; 4 * 2];
    let mut v_dst = vec![0u8; 4 * 2];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: 8,
        u: &mut u_dst,
        u_stride: 4,
        v: &mut v_dst,
        v_stride: 4,
    };
    let src_size = ImageSize::new(8, 4);

    let result =
        android420_to_i420_rotate(&src, src_size, 3, &mut dst, src_size, RotationMode::None);
    let err = result.expect_err("pixel_stride_uv が 1 / 2 以外では Err が返るべき");
    assert!(
        err.to_string().contains("pixel_stride_uv must be 1 or 2"),
        "pixel_stride_uv の値検証の reason が返るべき: {}",
        err
    );
}

// 異常系: android420_to_i420_rotate が pixel_stride_uv == 2 でインターリーブ幅未満の U ストライドを Err にすること
#[test]
fn android420_to_i420_rotate_interleaved_stride_boundary() {
    // 奇数幅（width = 5）ではインターリーブの行幅は 2 * ceil(width / 2) = 6 バイト
    let width = 5;
    let height = 3;
    let halfwidth = 3;
    let uv_height = 2;
    let y = vec![0u8; width * height];
    let u = vec![0u8; halfwidth * uv_height];
    let v = vec![0u8; halfwidth * uv_height];
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; halfwidth * uv_height];
    let mut v_dst = vec![0u8; halfwidth * uv_height];
    let src_size = ImageSize::new(width, height);

    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: halfwidth,
        v: &mut v_dst,
        v_stride: halfwidth,
    };
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: halfwidth,
        v: &v,
        v_stride: halfwidth,
    };
    let result =
        android420_to_i420_rotate(&src, src_size, 2, &mut dst, src_size, RotationMode::None);
    let err = result.expect_err("インターリーブ幅未満の U ストライドでは Err が返るべき");
    assert!(
        err.to_string()
            .contains("U stride smaller than interleaved chroma width"),
        "U 側のインターリーブ stride 不足の reason が返るべき: {}",
        err
    );

    // V 側のストライドが不足する場合も Err になる
    let u_interleaved = vec![0u8; halfwidth * 2 * uv_height];
    let v_small = vec![0u8; halfwidth * uv_height];
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &u_interleaved,
        u_stride: halfwidth * 2,
        v: &v_small,
        v_stride: halfwidth,
    };
    let result =
        android420_to_i420_rotate(&src, src_size, 2, &mut dst, src_size, RotationMode::None);
    let err = result.expect_err("インターリーブ幅未満の V ストライドでは Err が返るべき");
    assert!(
        err.to_string()
            .contains("V stride smaller than interleaved chroma width"),
        "V 側のインターリーブ stride 不足の reason が返るべき: {}",
        err
    );

    // バッファをインターリーブ幅に合わせ、同一バッファの連続領域（v は u の 1 バイト後）で
    // 渡せば成功する。末尾 1 バイトは v 側の検証（len >= stride * ceil(height / 2)）を
    // 満たすためのパディング
    let buf = vec![0u8; halfwidth * 2 * uv_height + 1];
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &buf[..buf.len() - 1],
        u_stride: halfwidth * 2,
        v: &buf[1..],
        v_stride: halfwidth * 2,
    };
    android420_to_i420_rotate(&src, src_size, 2, &mut dst, src_size, RotationMode::None)
        .expect("インターリーブ幅の U ストライドでは成功するべき");
}

// 異常系: android420_to_i420_rotate が pixel_stride_uv == 2 で偶数幅のインターリーブ境界を検証すること
#[test]
fn android420_to_i420_rotate_interleaved_even_width_stride_boundary() {
    // 偶数幅（width = 6）ではインターリーブの行幅は 2 * ceil(width / 2) = width になる
    let width = 6;
    let height = 3;
    let halfwidth = 3;
    let uv_height = 2;
    let y = vec![0u8; width * height];
    let u = vec![0u8; halfwidth * uv_height];
    let v = vec![0u8; halfwidth * uv_height];
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; halfwidth * uv_height];
    let mut v_dst = vec![0u8; halfwidth * uv_height];
    let src_size = ImageSize::new(width, height);

    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: halfwidth,
        v: &mut v_dst,
        v_stride: halfwidth,
    };
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: halfwidth,
        v: &v,
        v_stride: halfwidth,
    };
    let result =
        android420_to_i420_rotate(&src, src_size, 2, &mut dst, src_size, RotationMode::None);
    let err = result.expect_err("インターリーブ幅未満の U ストライドでは Err が返るべき");
    assert!(
        err.to_string()
            .contains("U stride smaller than interleaved chroma width"),
        "U 側のインターリーブ stride 不足の reason が返るべき: {}",
        err
    );

    // バッファをインターリーブ幅（= width）に合わせれば成功する
    let buf = vec![0u8; halfwidth * 2 * uv_height + 1];
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &buf[..buf.len() - 1],
        u_stride: halfwidth * 2,
        v: &buf[1..],
        v_stride: halfwidth * 2,
    };
    android420_to_i420_rotate(&src, src_size, 2, &mut dst, src_size, RotationMode::None)
        .expect("インターリーブ幅の U ストライドでは成功するべき");
}
