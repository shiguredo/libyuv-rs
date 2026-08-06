//! フォーマット変換 API の単体テスト
//!
//! エラーパス・境界値と、既知解による正常系（Y 成分の抽出）を検証する。

use std::ffi::c_int;

use shiguredo_libyuv::{
    ArgbImageMut, I420Image, I420ImageMut, ImageSize, Nv12Image, UyvyImage, Yuy2Image,
    i420_to_argb, nv12_to_i420, uyvy_to_y, yuy2_to_y,
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

// 正常系: yuy2_to_y が YUY2 の Y 成分を正しく取り出すこと
#[test]
fn yuy2_to_y_extracts_y() {
    // libyuv の YUY2ToYRow は src の偶数インデックス（Y 成分）を dst に出力する
    let width = 8;
    let height = 4;
    let src_stride = width * 2;
    let src_data: Vec<u8> = (0..(src_stride * height)).map(|i| i as u8).collect();
    let src = Yuy2Image {
        data: &src_data,
        stride: src_stride,
    };
    let mut dst_y = vec![0u8; width * height];
    let size = ImageSize::new(width, height);

    yuy2_to_y(&src, &mut dst_y, width, size).expect("YUY2 から Y 成分の抽出が成功すること");

    for r in 0..height {
        for x in 0..width {
            assert_eq!(
                dst_y[r * width + x],
                src_data[r * src_stride + x * 2],
                "行 {} の要素 {} は YUY2 の Y 成分と一致すること",
                r,
                x
            );
        }
    }
}

// 正常系: yuy2_to_y がパディング付きデスティネーションで行配置を正しく行うこと
#[test]
fn yuy2_to_y_padded_stride_row_placement() {
    // dst_stride_y > width では C 側の行合体（coalesce）が効かないため、行ごとの stride 送りが検証される
    let width = 8;
    let height = 4;
    let src_stride = width * 2;
    let dst_stride_y = 12; // パディング 4 要素
    let src_data: Vec<u8> = (0..(src_stride * height)).map(|i| i as u8).collect();
    let src = Yuy2Image {
        data: &src_data,
        stride: src_stride,
    };
    let mut dst_y = vec![0xFFu8; dst_stride_y * height]; // 未書き込み領域の検出用
    let size = ImageSize::new(width, height);

    yuy2_to_y(&src, &mut dst_y, dst_stride_y, size).expect("パディング付き変換が成功すること");

    for r in 0..height {
        for x in 0..width {
            assert_eq!(
                dst_y[r * dst_stride_y + x],
                src_data[r * src_stride + x * 2],
                "行 {} の要素 {} が正しく配置されること",
                r,
                x
            );
        }
        // libyuv は 1 行あたり width バイトだけ書き込むため、パディング領域は書き換わらない
        for x in width..dst_stride_y {
            assert_eq!(
                dst_y[r * dst_stride_y + x],
                0xFF,
                "パディング領域は書き換わらないこと"
            );
        }
    }
}

// 異常系: yuy2_to_y の dst_stride_y が width 未満で Err が返ること
#[test]
fn yuy2_to_y_dst_stride_too_small() {
    let src_data = vec![0u8; 16 * 4];
    let src = Yuy2Image {
        data: &src_data,
        stride: 16,
    };
    let mut dst_y = vec![0u8; 8 * 4];
    let size = ImageSize::new(8, 4);

    let result = yuy2_to_y(&src, &mut dst_y, 4, size);
    let err = result.expect_err("dst_stride_y < width では Err が返るべき");
    assert!(
        err.to_string()
            .contains("destination stride smaller than width"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: yuy2_to_y の dst_stride_y が c_int の範囲を超えると Err が返ること
#[test]
fn yuy2_to_y_dst_stride_exceeds_c_int() {
    let src_data = vec![0u8; 16 * 4];
    let src = Yuy2Image {
        data: &src_data,
        stride: 16,
    };
    let mut dst_y = vec![0u8; 8 * 4];
    let size = ImageSize::new(8, 4);

    let result = yuy2_to_y(&src, &mut dst_y, c_int::MAX as usize + 1, size);
    let err = result.expect_err("dst_stride_y が c_int を超える場合は Err が返るべき");
    assert!(
        err.to_string()
            .contains("destination stride exceeds c_int range"),
        "dst 側の c_int 超過の reason が返るべき: {}",
        err
    );
}

// 異常系: yuy2_to_y のデスティネーションバッファ不足で Err が返ること
#[test]
fn yuy2_to_y_dst_buffer_too_small() {
    let src_data = vec![0u8; 16 * 4];
    let src = Yuy2Image {
        data: &src_data,
        stride: 16,
    };
    let mut dst_y = vec![0u8; 8 * 4 - 1]; // 1 バイト不足
    let size = ImageSize::new(8, 4);

    let result = yuy2_to_y(&src, &mut dst_y, 8, size);
    let err = result.expect_err("dst バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination Y buffer too small"),
        "dst バッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: yuy2_to_y がゼロサイズ入力で Err を返すこと
#[test]
fn yuy2_to_y_zero_size() {
    // Rust 側の検証はゼロサイズを通過する。Err は libyuv 側の width <= 0 / height == 0
    // ガードが -1 を返すことによる
    let src_data = vec![0u8; 16];
    let src = Yuy2Image {
        data: &src_data,
        stride: 16,
    };
    let mut dst_y = vec![0u8; 8];
    let size = ImageSize::new(0, 1);

    let result = yuy2_to_y(&src, &mut dst_y, 8, size);
    assert!(result.is_err(), "width == 0 では Err が返るべき");

    let size = ImageSize::new(1, 0);
    let result = yuy2_to_y(&src, &mut dst_y, 8, size);
    assert!(result.is_err(), "height == 0 では Err が返るべき");
}

// 正常系: yuy2_to_y が奇数幅でも末尾ピクセルを正しく取り出すこと
#[test]
fn yuy2_to_y_odd_width() {
    // libyuv の YUY2ToYRow は奇数幅の末尾ピクセルを別分岐で処理する
    let width = 5;
    let height = 2;
    let src_stride = width * 2;
    let src_data: Vec<u8> = (0..(src_stride * height)).map(|i| i as u8).collect();
    let src = Yuy2Image {
        data: &src_data,
        stride: src_stride,
    };
    let mut dst_y = vec![0u8; width * height];
    let size = ImageSize::new(width, height);

    yuy2_to_y(&src, &mut dst_y, width, size).expect("奇数幅の変換が成功すること");

    for r in 0..height {
        for x in 0..width {
            assert_eq!(
                dst_y[r * width + x],
                src_data[r * src_stride + x * 2],
                "行 {} の要素 {} は YUY2 の Y 成分と一致すること",
                r,
                x
            );
        }
    }
}

// 正常系: uyvy_to_y が UYVY の Y 成分を正しく取り出すこと
#[test]
fn uyvy_to_y_extracts_y() {
    // libyuv の UYVYToYRow は src の奇数インデックス（Y 成分）を dst に出力する
    let width = 8;
    let height = 4;
    let src_stride = width * 2;
    let src_data: Vec<u8> = (0..(src_stride * height)).map(|i| i as u8).collect();
    let src = UyvyImage {
        data: &src_data,
        stride: src_stride,
    };
    let mut dst_y = vec![0u8; width * height];
    let size = ImageSize::new(width, height);

    uyvy_to_y(&src, &mut dst_y, width, size).expect("UYVY から Y 成分の抽出が成功すること");

    for r in 0..height {
        for x in 0..width {
            assert_eq!(
                dst_y[r * width + x],
                src_data[r * src_stride + x * 2 + 1],
                "行 {} の要素 {} は UYVY の Y 成分と一致すること",
                r,
                x
            );
        }
    }
}

// 正常系: uyvy_to_y がパディング付きデスティネーションで行配置を正しく行うこと
#[test]
fn uyvy_to_y_padded_stride_row_placement() {
    // dst_stride_y > width では C 側の行合体（coalesce）が効かないため、行ごとの stride 送りが検証される
    let width = 8;
    let height = 4;
    let src_stride = width * 2;
    let dst_stride_y = 12; // パディング 4 要素
    let src_data: Vec<u8> = (0..(src_stride * height)).map(|i| i as u8).collect();
    let src = UyvyImage {
        data: &src_data,
        stride: src_stride,
    };
    let mut dst_y = vec![0xFFu8; dst_stride_y * height]; // 未書き込み領域の検出用
    let size = ImageSize::new(width, height);

    uyvy_to_y(&src, &mut dst_y, dst_stride_y, size).expect("パディング付き変換が成功すること");

    for r in 0..height {
        for x in 0..width {
            assert_eq!(
                dst_y[r * dst_stride_y + x],
                src_data[r * src_stride + x * 2 + 1],
                "行 {} の要素 {} が正しく配置されること",
                r,
                x
            );
        }
        // libyuv は 1 行あたり width バイトだけ書き込むため、パディング領域は書き換わらない
        for x in width..dst_stride_y {
            assert_eq!(
                dst_y[r * dst_stride_y + x],
                0xFF,
                "パディング領域は書き換わらないこと"
            );
        }
    }
}

// 異常系: uyvy_to_y の dst_stride_y が width 未満で Err が返ること
#[test]
fn uyvy_to_y_dst_stride_too_small() {
    let src_data = vec![0u8; 16 * 4];
    let src = UyvyImage {
        data: &src_data,
        stride: 16,
    };
    let mut dst_y = vec![0u8; 8 * 4];
    let size = ImageSize::new(8, 4);

    let result = uyvy_to_y(&src, &mut dst_y, 4, size);
    let err = result.expect_err("dst_stride_y < width では Err が返るべき");
    assert!(
        err.to_string()
            .contains("destination stride smaller than width"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: uyvy_to_y の dst_stride_y が c_int の範囲を超えると Err が返ること
#[test]
fn uyvy_to_y_dst_stride_exceeds_c_int() {
    let src_data = vec![0u8; 16 * 4];
    let src = UyvyImage {
        data: &src_data,
        stride: 16,
    };
    let mut dst_y = vec![0u8; 8 * 4];
    let size = ImageSize::new(8, 4);

    let result = uyvy_to_y(&src, &mut dst_y, c_int::MAX as usize + 1, size);
    let err = result.expect_err("dst_stride_y が c_int を超える場合は Err が返るべき");
    assert!(
        err.to_string()
            .contains("destination stride exceeds c_int range"),
        "dst 側の c_int 超過の reason が返るべき: {}",
        err
    );
}

// 異常系: uyvy_to_y のデスティネーションバッファ不足で Err が返ること
#[test]
fn uyvy_to_y_dst_buffer_too_small() {
    let src_data = vec![0u8; 16 * 4];
    let src = UyvyImage {
        data: &src_data,
        stride: 16,
    };
    let mut dst_y = vec![0u8; 8 * 4 - 1]; // 1 バイト不足
    let size = ImageSize::new(8, 4);

    let result = uyvy_to_y(&src, &mut dst_y, 8, size);
    let err = result.expect_err("dst バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination Y buffer too small"),
        "dst バッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: uyvy_to_y がゼロサイズ入力で Err を返すこと
#[test]
fn uyvy_to_y_zero_size() {
    // Rust 側の検証はゼロサイズを通過する。Err は libyuv 側の width <= 0 / height == 0
    // ガードが -1 を返すことによる
    let src_data = vec![0u8; 16];
    let src = UyvyImage {
        data: &src_data,
        stride: 16,
    };
    let mut dst_y = vec![0u8; 8];
    let size = ImageSize::new(0, 1);

    let result = uyvy_to_y(&src, &mut dst_y, 8, size);
    assert!(result.is_err(), "width == 0 では Err が返るべき");

    let size = ImageSize::new(1, 0);
    let result = uyvy_to_y(&src, &mut dst_y, 8, size);
    assert!(result.is_err(), "height == 0 では Err が返るべき");
}

// 正常系: uyvy_to_y が奇数幅でも末尾ピクセルを正しく取り出すこと
#[test]
fn uyvy_to_y_odd_width() {
    // libyuv の UYVYToYRow は奇数幅の末尾ピクセルを別分岐で処理する
    let width = 5;
    let height = 2;
    let src_stride = width * 2;
    let src_data: Vec<u8> = (0..(src_stride * height)).map(|i| i as u8).collect();
    let src = UyvyImage {
        data: &src_data,
        stride: src_stride,
    };
    let mut dst_y = vec![0u8; width * height];
    let size = ImageSize::new(width, height);

    uyvy_to_y(&src, &mut dst_y, width, size).expect("奇数幅の変換が成功すること");

    for r in 0..height {
        for x in 0..width {
            assert_eq!(
                dst_y[r * width + x],
                src_data[r * src_stride + x * 2 + 1],
                "行 {} の要素 {} は UYVY の Y 成分と一致すること",
                r,
                x
            );
        }
    }
}
