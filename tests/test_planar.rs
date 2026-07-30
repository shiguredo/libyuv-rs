//! プレーン操作 API の単体テスト
//!
//! エラーパス・境界値を検証する。正常系のプロパティ検証は PBT でカバーする。

use shiguredo_libyuv::{ImageSize, copy_plane, split_uv_plane};

// 異常系: copy_plane のバッファ不足で Err が返ること
#[test]
fn copy_plane_buffer_too_small() {
    let src = vec![0u8; 64];
    let mut dst = vec![0u8; 10]; // 64 バイト必要だが 10 しか用意しない
    let size = ImageSize::new(8, 8);

    let result = copy_plane(&src, 8, &mut dst, 8, size);
    assert!(result.is_err(), "バッファ不足では Err が返るべき");
}

// 異常系: copy_plane の stride 不足で Err が返ること
#[test]
fn copy_plane_stride_too_small() {
    let src = vec![0u8; 64];
    let mut dst = vec![0u8; 64];
    let size = ImageSize::new(8, 8);

    let result = copy_plane(&src, 4, &mut dst, 8, size); // src stride=4 < width=8
    assert!(result.is_err(), "stride 不足では Err が返るべき");
}

// 異常系: split_uv_plane のバッファ不足で Err が返ること
#[test]
fn split_uv_plane_buffer_too_small() {
    let uv = vec![0u8; 128];
    let mut u = vec![0u8; 10]; // 32 バイト必要だが 10 しか用意しない
    let mut v = vec![0u8; 32];
    let size = ImageSize::new(8, 8);

    let result = split_uv_plane(&uv, 16, &mut u, 8, &mut v, 8, size);
    assert!(result.is_err(), "バッファ不足では Err が返るべき");
}

// 異常系: split_uv_plane の stride 不足で Err が返ること
#[test]
fn split_uv_plane_stride_too_small() {
    let uv = vec![0u8; 128];
    let mut u = vec![0u8; 32];
    let mut v = vec![0u8; 32];
    let size = ImageSize::new(8, 8);

    // uv stride=8 < width*2=16
    let result = split_uv_plane(&uv, 8, &mut u, 8, &mut v, 8, size);
    assert!(result.is_err(), "stride 不足では Err が返るべき");
}
