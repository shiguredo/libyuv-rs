//! i420 サブモジュールの代表関数の単体テスト
//!
//! `src/convert/i420.rs` に対応する。代表関数として
//! `i400_to_argb` (グレー→ARGB)・`i420_to_i400` (Y 抽出)・`i420_to_i010` (8bit→10bit) を選定し、
//! ground truth 方式の正常系とエラーパス (バッファ不足・stride 不足・c_int 超過) を検証する。
//! alpha 系は alpha.rs で扱う。

use std::ffi::c_int;

use shiguredo_libyuv::{
    ArgbImageMut, I010ImageMut, I400Image, I400ImageMut, I420Image, ImageSize, i400_to_argb,
    i420_to_i010, i420_to_i400,
};

use super::helpers;

// ============================================================
// i400_to_argb (グレースケール → ARGB)
// ============================================================

// 正常系: i400_to_argb が既知グレー値から期待 ARGB を出力すること
#[test]
fn i400_to_argb_known_values() {
    let width = 4;
    let height = 1;
    let y = vec![0u8, 85, 170, 255];
    let src = I400Image {
        y: &y,
        y_stride: width,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    i400_to_argb(&src, &mut dst, size).expect("グレースケールから ARGB への変換が成功すること");

    for x in 0..width {
        let expected = helpers::y601_to_argb_pixel(y[x]);
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

// 異常系: i400_to_argb の src のバッファ不足で Err が返ること
#[test]
fn i400_to_argb_src_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height - 1];
    let src = I400Image {
        y: &y,
        y_stride: width,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = i400_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("src バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "src 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i400_to_argb の dst の stride 不足で Err が返ること
#[test]
fn i400_to_argb_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let src = I400Image {
        y: &y,
        y_stride: width,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4 - 1,
    };
    let result = i400_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i400_to_argb の dst の stride の c_int 超過で Err が返ること
#[test]
fn i400_to_argb_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let src = I400Image {
        y: &y,
        y_stride: width,
    };
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: c_int::MAX as usize + 1,
    };
    let result = i400_to_argb(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// i420_to_i400 (I420 → グレースケール Y 抽出)
// ============================================================

// 正常系: i420_to_i400 が Y プレーンをそのまま出力すること
#[test]
fn i420_to_i400_extracts_y() {
    let width = 4;
    let height = 2;
    let y: Vec<u8> = (0..(width * height)).map(|i| i as u8).collect();
    let u = vec![0u8; 2];
    let v = vec![0u8; 2];
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let mut y_out = vec![0u8; width * height];
    let mut dst = I400ImageMut {
        y: &mut y_out,
        y_stride: width,
    };
    let size = ImageSize::new(width, height);

    i420_to_i400(&src, &mut dst, size).expect("I420 から I400 への変換が成功すること");

    assert_eq!(y_out, y, "Y プレーンがそのまま出力されること");
}

// 異常系: i420_to_i400 の dst のバッファ不足で Err が返ること
#[test]
fn i420_to_i400_dst_buffer_too_small() {
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
    let mut y_out = vec![0u8; width * height - 1];
    let mut dst = I400ImageMut {
        y: &mut y_out,
        y_stride: width,
    };
    let result = i420_to_i400(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination Y buffer too small"),
        "dst 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i420_to_i400 の dst の stride 不足で Err が返ること
#[test]
fn i420_to_i400_dst_stride_too_small() {
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
    let mut y_out = vec![0u8; width * height];
    let mut dst = I400ImageMut {
        y: &mut y_out,
        y_stride: width - 1,
    };
    let result = i420_to_i400(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i420_to_i400 の src の Y stride の c_int 超過で Err が返ること
#[test]
fn i420_to_i400_src_y_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 2];
    let v = vec![0u8; 4 * 2];
    let src = I420Image {
        y: &y,
        y_stride: c_int::MAX as usize + 1,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let mut y_out = vec![0u8; width * height];
    let mut dst = I400ImageMut {
        y: &mut y_out,
        y_stride: width,
    };
    let result = i420_to_i400(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("Y stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// i420_to_i010 (8bit → 10bit)
// ============================================================

// 正常系: i420_to_i010 が 8bit 値を 10bit に正しく拡張すること
#[test]
fn i420_to_i010_known_values() {
    let width = 4;
    let height = 1;
    let y = vec![0u8, 64, 128, 255];
    let u = vec![0u8, 128];
    let v = vec![0u8, 128];
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let mut y10 = vec![0u16; width * height];
    let mut u10 = vec![0u16; 2];
    let mut v10 = vec![0u16; 2];
    let mut dst = I010ImageMut {
        y: &mut y10,
        y_stride: width,
        u: &mut u10,
        u_stride: 2,
        v: &mut v10,
        v_stride: 2,
    };
    let size = ImageSize::new(width, height);

    i420_to_i010(&src, &mut dst, size).expect("I420 から I010 への変換が成功すること");

    for x in 0..width {
        assert_eq!(
            y10[x],
            helpers::u8_to_u10(y[x]),
            "Y[{}] が期待値と一致すること",
            x
        );
    }
    for x in 0..2 {
        assert_eq!(
            u10[x],
            helpers::u8_to_u10(u[x]),
            "U[{}] が期待値と一致すること",
            x
        );
        assert_eq!(
            v10[x],
            helpers::u8_to_u10(v[x]),
            "V[{}] が期待値と一致すること",
            x
        );
    }
}

// 異常系: i420_to_i010 の dst の Y バッファ不足で Err が返ること
#[test]
fn i420_to_i010_dst_y_buffer_too_small() {
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
    let mut y10 = vec![0u16; width * height - 1];
    let mut u10 = vec![0u16; 4 * 2];
    let mut v10 = vec![0u16; 4 * 2];
    let mut dst = I010ImageMut {
        y: &mut y10,
        y_stride: width,
        u: &mut u10,
        u_stride: 4,
        v: &mut v10,
        v_stride: 4,
    };
    let result = i420_to_i010(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst の Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination Y buffer too small"),
        "dst の Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i420_to_i010 の dst の U stride 不足で Err が返ること
#[test]
fn i420_to_i010_dst_u_stride_too_small() {
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
    let mut y10 = vec![0u16; width * height];
    let mut u10 = vec![0u16; 4 * 2];
    let mut v10 = vec![0u16; 4 * 2];
    let mut dst = I010ImageMut {
        y: &mut y10,
        y_stride: width,
        u: &mut u10,
        u_stride: 3, // 4 未満
        v: &mut v10,
        v_stride: 4,
    };
    let result = i420_to_i010(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst の U stride 不足では Err が返るべき");
    assert!(
        err.to_string()
            .contains("U stride smaller than chroma width"),
        "dst の U 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i420_to_i010 の dst の V stride の c_int 超過で Err が返ること
#[test]
fn i420_to_i010_dst_v_stride_exceeds_c_int() {
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
    let mut y10 = vec![0u16; width * height];
    let mut u10 = vec![0u16; 4 * 2];
    let mut v10 = vec![0u16; 4 * 2];
    let mut dst = I010ImageMut {
        y: &mut y10,
        y_stride: width,
        u: &mut u10,
        u_stride: 4,
        v: &mut v10,
        v_stride: c_int::MAX as usize + 1,
    };
    let result = i420_to_i010(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("V stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
