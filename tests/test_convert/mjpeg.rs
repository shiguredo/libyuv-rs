//! mjpeg サブモジュールの代表関数の単体テスト
//!
//! `src/convert/mjpeg.rs` に対応する。代表関数として `mjpeg_to_i420` を選定し、
//! 既知 JPEG のデコード結果を ground truth として固定して正常系を検証する。
//! エラーパス (バッファ不足・stride 不足・c_int 超過) も検証する。
//!
//! ground truth は libjpeg-turbo 3.1.90 (build.rs で固定) のデコード結果であり、
//! libyuv の MJPG デコードは既定の整数 IDCT を使うため SIMD / 非 SIMD で同一出力になる。
//! テストデータは `src/test_data/mjpeg_8x8_yuv420.jpg` (8x8, 4:2:0, gradient:red-blue) を使う。

use std::ffi::c_int;

use shiguredo_libyuv::{I420ImageMut, ImageSize, mjpeg_to_i420};

// テスト対象の JPEG ファイル (バイト列ごと埋め込む)
const JPEG_8X8_YUV420: &[u8] = include_bytes!("../../src/test_data/mjpeg_8x8_yuv420.jpg");
// 8x8 4:2:0 のデコード結果 (libjpeg-turbo 3.1.90 の実測値)
// Y: 8x8 (64 要素)。上から下へ行ごとに 8 要素ずつ
const EXPECTED_Y: [u8; 64] = [
    76, 76, 76, 76, 76, 76, 76, 76, //
    71, 71, 71, 71, 71, 71, 71, 71, //
    63, 63, 63, 63, 63, 63, 63, 63, //
    56, 56, 56, 56, 56, 56, 56, 56, //
    49, 49, 49, 49, 49, 49, 49, 49, //
    42, 42, 42, 42, 42, 42, 42, 42, //
    34, 34, 34, 34, 34, 34, 34, 34, //
    29, 29, 29, 29, 29, 29, 29, 29, //
];
// U: 4x4 (16 要素)
const EXPECTED_U: [u8; 16] = [
    97, 97, 97, 97, //
    146, 146, 146, 146, //
    194, 194, 194, 194, //
    244, 244, 244, 244, //
];
// V: 4x4 (16 要素)
const EXPECTED_V: [u8; 16] = [
    246, 246, 246, 246, //
    201, 201, 201, 201, //
    160, 160, 160, 160, //
    115, 115, 115, 115, //
];

// 正常系: mjpeg_to_i420 が既知 JPEG から期待 I420 を出力すること
#[test]
fn mjpeg_to_i420_known_jpeg() {
    let size = ImageSize::new(8, 8);
    let mut y = vec![0u8; 8 * 8];
    let mut u = vec![0u8; 4 * 4];
    let mut v = vec![0u8; 4 * 4];
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride: 8,
        u: &mut u,
        u_stride: 4,
        v: &mut v,
        v_stride: 4,
    };

    mjpeg_to_i420(JPEG_8X8_YUV420, &mut dst, size).expect("既知 JPEG のデコードが成功すること");

    assert_eq!(
        y.as_slice(),
        &EXPECTED_Y,
        "Y プレーンが期待値と一致すること"
    );
    assert_eq!(
        u.as_slice(),
        &EXPECTED_U,
        "U プレーンが期待値と一致すること"
    );
    assert_eq!(
        v.as_slice(),
        &EXPECTED_V,
        "V プレーンが期待値と一致すること"
    );
}

// 異常系: mjpeg_to_i420 の src が空で Err が返ること
#[test]
fn mjpeg_to_i420_src_empty() {
    let size = ImageSize::new(8, 8);
    let mut y = vec![0u8; 8 * 8];
    let mut u = vec![0u8; 4 * 4];
    let mut v = vec![0u8; 4 * 4];
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride: 8,
        u: &mut u,
        u_stride: 4,
        v: &mut v,
        v_stride: 4,
    };
    let result = mjpeg_to_i420(&[], &mut dst, size);
    let err = result.expect_err("空入力では Err が返るべき");
    assert!(
        err.to_string().contains("src is empty"),
        "空入力の reason が返るべき: {}",
        err
    );
}

// 異常系: mjpeg_to_i420 の dst の Y バッファ不足で Err が返ること
#[test]
fn mjpeg_to_i420_dst_y_buffer_too_small() {
    let size = ImageSize::new(8, 8);
    let mut y = vec![0u8; 8 * 8 - 1];
    let mut u = vec![0u8; 4 * 4];
    let mut v = vec![0u8; 4 * 4];
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride: 8,
        u: &mut u,
        u_stride: 4,
        v: &mut v,
        v_stride: 4,
    };
    let result = mjpeg_to_i420(JPEG_8X8_YUV420, &mut dst, size);
    let err = result.expect_err("Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination Y buffer too small"),
        "Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: mjpeg_to_i420 の size が c_int の範囲を超えると Err が返ること
#[test]
fn mjpeg_to_i420_size_exceeds_c_int() {
    let size = ImageSize::new(c_int::MAX as usize + 1, 1);
    let mut y = vec![0u8; 16];
    let mut u = vec![0u8; 16];
    let mut v = vec![0u8; 16];
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride: 1,
        u: &mut u,
        u_stride: 1,
        v: &mut v,
        v_stride: 1,
    };
    let result = mjpeg_to_i420(JPEG_8X8_YUV420, &mut dst, size);
    let err = result.expect_err("c_int 範囲を超えるサイズでは Err が返るべき");
    assert!(
        err.to_string().contains("width exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
