//! alpha 系変換の代表関数の単体テスト
//!
//! `src/convert/i420.rs` の alpha 系 7 関数について、0050 の委譲どおり
//! 正常系・バッファ不足テストを追加する (stride 境界は 0050 で実装済み)。

use shiguredo_libyuv::{
    AbgrImageMut, ArgbImage, ArgbImageMut, I420Image, I420ImageMut, I422Image, I444Image,
    ImageSize, argb_to_i420_alpha, i420_alpha_to_abgr, i420_alpha_to_argb, i422_alpha_to_abgr,
    i422_alpha_to_argb, i444_alpha_to_abgr, i444_alpha_to_argb,
};

// ============================================================
// 共通ヘルパー
// ============================================================

// 4x2 の I420 画像を構築する
fn build_i420(width: usize, height: usize) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let uv_w = width.div_ceil(2);
    let uv_h = height.div_ceil(2);
    let y: Vec<u8> = (0..(width * height)).map(|i| (i * 7 % 256) as u8).collect();
    let u: Vec<u8> = (0..(uv_w * uv_h)).map(|i| (i * 11 % 256) as u8).collect();
    let v: Vec<u8> = (0..(uv_w * uv_h)).map(|i| (i * 13 % 256) as u8).collect();
    (y, u, v)
}

// ============================================================
// i420_alpha_to_argb
// ============================================================

// 正常系: i420_alpha_to_argb がアルファ値をそのままコピーすること
///
/// `attenuate=false` では RGB は減衰されず、アルファ値だけが src_a からコピーされる
/// (I422AlphaToARGBRow_C が rgb_buf[3] = src_a[x] と書くため)。
/// 全画素のアルファ値が入力と一致することを検証する。
#[test]
fn i420_alpha_to_argb_blends() {
    let width = 4;
    let height = 1;
    let y = vec![81u8, 81, 81, 81];
    let u = vec![90u8, 90];
    let v = vec![240u8, 240];
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    // アルファ 0 / 85 / 170 / 255
    let alpha = vec![0u8, 85, 170, 255];
    let mut data = vec![0xFFu8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    i420_alpha_to_argb(&src, &alpha, width, &mut dst, size, false)
        .expect("アルファ合成が成功すること");

    for x in 0..width {
        assert_eq!(
            data[x * 4 + 3],
            alpha[x],
            "x={} のアルファが入力と一致すること: 実測 {} 期待 {}",
            x,
            data[x * 4 + 3],
            alpha[x]
        );
    }
}

// 異常系: i420_alpha_to_argb のアルファバッファ不足で Err が返ること
#[test]
fn i420_alpha_to_argb_alpha_buffer_too_small() {
    let width = 8;
    let height = 4;
    let (y, u, v) = build_i420(width, height);
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let alpha = vec![0u8; width * height - 1];
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = i420_alpha_to_argb(
        &src,
        &alpha,
        width,
        &mut dst,
        ImageSize::new(width, height),
        false,
    );
    let err = result.expect_err("アルファバッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("alpha buffer too small"),
        "アルファバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i420_alpha_to_argb の dst のバッファ不足で Err が返ること
#[test]
fn i420_alpha_to_argb_dst_buffer_too_small() {
    let width = 8;
    let height = 4;
    let (y, u, v) = build_i420(width, height);
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let alpha = vec![0u8; width * height];
    let mut data = vec![0u8; width * height * 4 - 1];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = i420_alpha_to_argb(
        &src,
        &alpha,
        width,
        &mut dst,
        ImageSize::new(width, height),
        false,
    );
    let err = result.expect_err("dst バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination buffer too small"),
        "dst バッファ不足の reason が返るべき: {}",
        err
    );
}

// ============================================================
// i420_alpha_to_abgr
// ============================================================

// 正常系: i420_alpha_to_abgr がアルファ値をそのままコピーすること
///
/// `attenuate=false` では RGB は減衰されず、アルファ値だけが src_a からコピーされる
/// (I422AlphaToARGBRow_C が rgb_buf[3] = src_a[x] と書くため)。
/// 全画素のアルファ値が入力と一致することを検証する。
#[test]
fn i420_alpha_to_abgr_blends() {
    let width = 4;
    let height = 1;
    let y = vec![81u8, 81, 81, 81];
    let u = vec![90u8, 90];
    let v = vec![240u8, 240];
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let alpha = vec![0u8, 85, 170, 255];
    let mut data = vec![0xFFu8; width * height * 4];
    let mut dst = AbgrImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    i420_alpha_to_abgr(&src, &alpha, width, &mut dst, size, false)
        .expect("アルファ合成が成功すること");

    for x in 0..width {
        assert_eq!(
            data[x * 4 + 3],
            alpha[x],
            "x={} のアルファが入力と一致すること: 実測 {} 期待 {}",
            x,
            data[x * 4 + 3],
            alpha[x]
        );
    }
}

// 異常系: i420_alpha_to_abgr のアルファバッファ不足で Err が返ること
#[test]
fn i420_alpha_to_abgr_alpha_buffer_too_small() {
    let width = 8;
    let height = 4;
    let (y, u, v) = build_i420(width, height);
    let src = I420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let alpha = vec![0u8; width * height - 1];
    let mut data = vec![0u8; width * height * 4];
    let mut dst = AbgrImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = i420_alpha_to_abgr(
        &src,
        &alpha,
        width,
        &mut dst,
        ImageSize::new(width, height),
        false,
    );
    let err = result.expect_err("アルファバッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("alpha buffer too small"),
        "アルファバッファ不足の reason が返るべき: {}",
        err
    );
}

// ============================================================
// i422_alpha_to_argb / i422_alpha_to_abgr
// ============================================================

// 正常系: i422_alpha_to_argb がアルファ値をそのままコピーすること
#[test]
fn i422_alpha_to_argb_blends() {
    let width = 4;
    let height = 1;
    let y = vec![81u8, 81, 81, 81];
    let u = vec![90u8, 90];
    let v = vec![240u8, 240];
    let src = I422Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let alpha = vec![0u8, 85, 170, 255];
    let mut data = vec![0xFFu8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    i422_alpha_to_argb(&src, &alpha, width, &mut dst, size, false)
        .expect("アルファ合成が成功すること");

    for x in 0..width {
        assert_eq!(
            data[x * 4 + 3],
            alpha[x],
            "x={} のアルファが入力と一致すること: 実測 {} 期待 {}",
            x,
            data[x * 4 + 3],
            alpha[x]
        );
    }
}

// 異常系: i422_alpha_to_argb のアルファバッファ不足で Err が返ること
#[test]
fn i422_alpha_to_argb_alpha_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 4];
    let v = vec![0u8; 4 * 4];
    let src = I422Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let alpha = vec![0u8; width * height - 1];
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = i422_alpha_to_argb(
        &src,
        &alpha,
        width,
        &mut dst,
        ImageSize::new(width, height),
        false,
    );
    let err = result.expect_err("アルファバッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("alpha buffer too small"),
        "アルファバッファ不足の reason が返るべき: {}",
        err
    );
}

// 正常系: i422_alpha_to_abgr がアルファ値をそのままコピーすること
#[test]
fn i422_alpha_to_abgr_blends() {
    let width = 4;
    let height = 1;
    let y = vec![81u8, 81, 81, 81];
    let u = vec![90u8, 90];
    let v = vec![240u8, 240];
    let src = I422Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 2,
        v: &v,
        v_stride: 2,
    };
    let alpha = vec![0u8, 85, 170, 255];
    let mut data = vec![0xFFu8; width * height * 4];
    let mut dst = AbgrImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    i422_alpha_to_abgr(&src, &alpha, width, &mut dst, size, false)
        .expect("アルファ合成が成功すること");

    for x in 0..width {
        assert_eq!(
            data[x * 4 + 3],
            alpha[x],
            "x={} のアルファが入力と一致すること: 実測 {} 期待 {}",
            x,
            data[x * 4 + 3],
            alpha[x]
        );
    }
}

// 異常系: i422_alpha_to_abgr のアルファバッファ不足で Err が返ること
#[test]
fn i422_alpha_to_abgr_alpha_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; 4 * 4];
    let v = vec![0u8; 4 * 4];
    let src = I422Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: 4,
        v: &v,
        v_stride: 4,
    };
    let alpha = vec![0u8; width * height - 1];
    let mut data = vec![0u8; width * height * 4];
    let mut dst = AbgrImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = i422_alpha_to_abgr(
        &src,
        &alpha,
        width,
        &mut dst,
        ImageSize::new(width, height),
        false,
    );
    let err = result.expect_err("アルファバッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("alpha buffer too small"),
        "アルファバッファ不足の reason が返るべき: {}",
        err
    );
}

// ============================================================
// i444_alpha_to_argb / i444_alpha_to_abgr
// ============================================================

// 正常系: i444_alpha_to_argb がアルファ値をそのままコピーすること
#[test]
fn i444_alpha_to_argb_blends() {
    let width = 4;
    let height = 1;
    let y = vec![81u8, 81, 81, 81];
    let u = vec![90u8, 90, 90, 90];
    let v = vec![240u8, 240, 240, 240];
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let alpha = vec![0u8, 85, 170, 255];
    let mut data = vec![0xFFu8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    i444_alpha_to_argb(&src, &alpha, width, &mut dst, size, false)
        .expect("アルファ合成が成功すること");

    for x in 0..width {
        assert_eq!(
            data[x * 4 + 3],
            alpha[x],
            "x={} のアルファが入力と一致すること: 実測 {} 期待 {}",
            x,
            data[x * 4 + 3],
            alpha[x]
        );
    }
}

// 異常系: i444_alpha_to_argb のアルファバッファ不足で Err が返ること
#[test]
fn i444_alpha_to_argb_alpha_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; width * height];
    let v = vec![0u8; width * height];
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let alpha = vec![0u8; width * height - 1];
    let mut data = vec![0u8; width * height * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = i444_alpha_to_argb(
        &src,
        &alpha,
        width,
        &mut dst,
        ImageSize::new(width, height),
        false,
    );
    let err = result.expect_err("アルファバッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("alpha buffer too small"),
        "アルファバッファ不足の reason が返るべき: {}",
        err
    );
}

// 正常系: i444_alpha_to_abgr がアルファ値をそのままコピーすること
#[test]
fn i444_alpha_to_abgr_blends() {
    let width = 4;
    let height = 1;
    let y = vec![81u8, 81, 81, 81];
    let u = vec![90u8, 90, 90, 90];
    let v = vec![240u8, 240, 240, 240];
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let alpha = vec![0u8, 85, 170, 255];
    let mut data = vec![0xFFu8; width * height * 4];
    let mut dst = AbgrImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let size = ImageSize::new(width, height);

    i444_alpha_to_abgr(&src, &alpha, width, &mut dst, size, false)
        .expect("アルファ合成が成功すること");

    for x in 0..width {
        assert_eq!(
            data[x * 4 + 3],
            alpha[x],
            "x={} のアルファが入力と一致すること: 実測 {} 期待 {}",
            x,
            data[x * 4 + 3],
            alpha[x]
        );
    }
}

// 異常系: i444_alpha_to_abgr のアルファバッファ不足で Err が返ること
#[test]
fn i444_alpha_to_abgr_alpha_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; width * height];
    let v = vec![0u8; width * height];
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let alpha = vec![0u8; width * height - 1];
    let mut data = vec![0u8; width * height * 4];
    let mut dst = AbgrImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let result = i444_alpha_to_abgr(
        &src,
        &alpha,
        width,
        &mut dst,
        ImageSize::new(width, height),
        false,
    );
    let err = result.expect_err("アルファバッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("alpha buffer too small"),
        "アルファバッファ不足の reason が返るべき: {}",
        err
    );
}

// ============================================================
// argb_to_i420_alpha
// ============================================================

// 正常系: argb_to_i420_alpha がアルファ抽出して期待 I420 を出力すること
#[test]
fn argb_to_i420_alpha_extracts() {
    let width = 4;
    let height = 1;
    // 各ピクセルは B,G,R,A の順
    let data = vec![
        0u8, 0, 0, 0, // 黒 A=0
        81, 81, 81, 85, // A=85
        81, 81, 81, 170, // A=170
        81, 81, 81, 255, // A=255
    ];
    let src = ArgbImage {
        data: &data,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut u = vec![0u8; 2];
    let mut v = vec![0u8; 2];
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride: width,
        u: &mut u,
        u_stride: 2,
        v: &mut v,
        v_stride: 2,
    };
    let mut alpha = vec![0xFFu8; width * height];
    let size = ImageSize::new(width, height);

    argb_to_i420_alpha(&src, &mut dst, &mut alpha, width, size)
        .expect("ARGB から I420+アルファへの変換が成功すること");

    // アルファは ARGB の 4 バイト目がそのまま出る
    for x in 0..width {
        assert_eq!(
            alpha[x],
            data[x * 4 + 3],
            "アルファ[{}] が期待値と一致すること",
            x
        );
    }
}

// 異常系: argb_to_i420_alpha のアルファバッファ不足で Err が返ること
#[test]
fn argb_to_i420_alpha_alpha_buffer_too_small() {
    let width = 8;
    let height = 4;
    let data = vec![0u8; width * height * 4];
    let src = ArgbImage {
        data: &data,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut u = vec![0u8; 4 * 2];
    let mut v = vec![0u8; 4 * 2];
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride: width,
        u: &mut u,
        u_stride: 4,
        v: &mut v,
        v_stride: 4,
    };
    let mut alpha = vec![0u8; width * height - 1];
    let result = argb_to_i420_alpha(
        &src,
        &mut dst,
        &mut alpha,
        width,
        ImageSize::new(width, height),
    );
    let err = result.expect_err("アルファバッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("alpha buffer too small"),
        "アルファバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: argb_to_i420_alpha の dst の Y バッファ不足で Err が返ること
#[test]
fn argb_to_i420_alpha_dst_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let data = vec![0u8; width * height * 4];
    let src = ArgbImage {
        data: &data,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height - 1];
    let mut u = vec![0u8; 4 * 2];
    let mut v = vec![0u8; 4 * 2];
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride: width,
        u: &mut u,
        u_stride: 4,
        v: &mut v,
        v_stride: 4,
    };
    let mut alpha = vec![0u8; width * height];
    let result = argb_to_i420_alpha(
        &src,
        &mut dst,
        &mut alpha,
        width,
        ImageSize::new(width, height),
    );
    let err = result.expect_err("dst の Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination Y buffer too small"),
        "dst の Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}
