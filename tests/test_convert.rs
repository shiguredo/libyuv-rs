//! フォーマット変換 API の単体テスト
//!
//! エラーパス・境界値と、既知解による正常系（Y 成分の抽出）を検証する。

use std::ffi::c_int;

use shiguredo_libyuv::{
    AbgrImageMut, Android420Image, ArgbImageMut, I420Image, I420ImageMut, ImageSize, Mm21Image,
    Mt2tImage, Nv12Image, P010ImageMut, UyvyImage, Yuy2Image, android420_to_abgr,
    android420_to_argb, android420_to_i420, i420_to_argb, mm21_to_i420, mt2t_to_p010, nv12_to_i420,
    uyvy_to_y, yuy2_to_y,
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

// 正常系: yuy2_to_y が stride == width で正しい出力を返すこと
#[test]
fn yuy2_to_y_extracts_y() {
    // libyuv の YUY2ToYRow は src の偶数インデックス（Y 成分）を dst に出力する。
    // src_stride == width * 2 かつ dst_stride_y == width のため、C 側の行合体（coalesce）
    // により全行が 1 回の行関数呼び出しで処理される
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

// 正常系: yuy2_to_y が奇数幅でも末尾ピクセルを正しく取り出すこと
#[test]
fn yuy2_to_y_odd_width() {
    // src_stride と dst_stride_y の両方を行合体（coalesce）の条件から外し、
    // 奇数幅の末尾ピクセル処理（SIMD では余り処理経路、C フォールバックでは width & 1 の
    // 分岐）を実行させる。src 側もパディングを持たせ、余り処理の読み越しをバッファ内に収める
    let width = 5;
    let height = 2;
    let src_stride = 16; // width * 2 = 10 より大きいパディング
    let dst_stride_y = 6; // width = 5 より大きいパディング
    let mut src_data = vec![0u8; src_stride * height];
    for r in 0..height {
        for i in 0..(width * 2) {
            src_data[r * src_stride + i] = (r * src_stride + i) as u8;
        }
    }
    let src = Yuy2Image {
        data: &src_data,
        stride: src_stride,
    };
    let mut dst_y = vec![0xFFu8; dst_stride_y * height]; // 未書き込み領域の検出用
    let size = ImageSize::new(width, height);

    yuy2_to_y(&src, &mut dst_y, dst_stride_y, size).expect("奇数幅の変換が成功すること");

    for r in 0..height {
        for x in 0..width {
            assert_eq!(
                dst_y[r * dst_stride_y + x],
                src_data[r * src_stride + x * 2],
                "行 {} の要素 {} は YUY2 の Y 成分と一致すること",
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

// 正常系: uyvy_to_y が奇数幅でも末尾ピクセルを正しく取り出すこと
#[test]
fn uyvy_to_y_odd_width() {
    // src_stride と dst_stride_y の両方を行合体（coalesce）の条件から外し、
    // 奇数幅の末尾ピクセル処理（SIMD では余り処理経路、C フォールバックでは width & 1 の
    // 分岐）を実行させる。src 側もパディングを持たせ、余り処理の読み越しをバッファ内に収める
    let width = 5;
    let height = 2;
    let src_stride = 16; // width * 2 = 10 より大きいパディング
    let dst_stride_y = 6; // width = 5 より大きいパディング
    let mut src_data = vec![0u8; src_stride * height];
    for r in 0..height {
        for i in 0..(width * 2) {
            src_data[r * src_stride + i] = (r * src_stride + i) as u8;
        }
    }
    let src = UyvyImage {
        data: &src_data,
        stride: src_stride,
    };
    let mut dst_y = vec![0xFFu8; dst_stride_y * height]; // 未書き込み領域の検出用
    let size = ImageSize::new(width, height);

    uyvy_to_y(&src, &mut dst_y, dst_stride_y, size).expect("奇数幅の変換が成功すること");

    for r in 0..height {
        for x in 0..width {
            assert_eq!(
                dst_y[r * dst_stride_y + x],
                src_data[r * src_stride + x * 2 + 1],
                "行 {} の要素 {} は UYVY の Y 成分と一致すること",
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

// 異常系: android420_to_i420 が pixel_stride_uv の 1 / 2 以外を Err にすること
#[test]
fn android420_to_i420_pixel_stride_uv_invalid() {
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
    let size = ImageSize::new(8, 4);

    let result = android420_to_i420(&src, 3, &mut dst, size);
    let err = result.expect_err("pixel_stride_uv が 1 / 2 以外では Err が返るべき");
    assert!(
        err.to_string().contains("pixel_stride_uv must be 1 or 2"),
        "pixel_stride_uv の値検証の reason が返るべき: {}",
        err
    );
}

// 正常系: android420_to_i420 が pixel_stride_uv == 1 で平面データを変換できること
#[test]
fn android420_to_i420_pixel_stride_uv_one_ok() {
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
    let size = ImageSize::new(8, 4);

    android420_to_i420(&src, 1, &mut dst, size).expect("pixel_stride_uv == 1 の変換が成功すること");
}

// 異常系: android420_to_i420 が pixel_stride_uv == 2 でインターリーブ幅未満の U / V ストライドを Err にすること
#[test]
fn android420_to_i420_interleaved_stride_boundary() {
    // 奇数幅（width = 5）ではインターリーブの行幅は 2 * ceil(width / 2) = 6 バイト。
    // stride == ceil(width / 2) = 3（平面の最小幅）も stride == width = 5 も
    // インターリーブには不足する
    let width = 5;
    let height = 3;
    let halfwidth = 3;
    let uv_height = 2;
    let y = vec![0u8; width * height];
    let u = vec![0u8; width * uv_height];
    let v = vec![0u8; width * uv_height];
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; halfwidth * uv_height];
    let mut v_dst = vec![0u8; halfwidth * uv_height];
    let size = ImageSize::new(width, height);

    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: halfwidth,
        v: &mut v_dst,
        v_stride: halfwidth,
    };

    // stride == halfwidth では Err
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: halfwidth,
        v: &v,
        v_stride: halfwidth,
    };
    let result = android420_to_i420(&src, 2, &mut dst, size);
    let err = result.expect_err("インターリーブ幅未満の U ストライドでは Err が返るべき");
    assert!(
        err.to_string()
            .contains("U stride smaller than interleaved chroma width"),
        "U 側のインターリーブ stride 不足の reason が返るべき: {}",
        err
    );

    // stride == width でも Err（検証式が 2 * ceil(width / 2) から width に退行すると
    // このケースを通過してしまうため、境界として検証する）
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let result = android420_to_i420(&src, 2, &mut dst, size);
    let err = result.expect_err("stride == width では Err が返るべき");
    assert!(
        err.to_string()
            .contains("U stride smaller than interleaved chroma width"),
        "U 側のインターリーブ stride 不足の reason が返るべき: {}",
        err
    );

    // V 側のストライドが不足する場合も Err
    let u_interleaved = vec![0u8; halfwidth * 2 * uv_height];
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &u_interleaved,
        u_stride: halfwidth * 2,
        v: &v,
        v_stride: halfwidth,
    };
    let result = android420_to_i420(&src, 2, &mut dst, size);
    let err = result.expect_err("インターリーブ幅未満の V ストライドでは Err が返るべき");
    assert!(
        err.to_string()
            .contains("V stride smaller than interleaved chroma width"),
        "V 側のインターリーブ stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: android420_to_i420 が pixel_stride_uv == 2 で偶数幅のインターリーブ境界を検証すること
#[test]
fn android420_to_i420_interleaved_even_width_stride_boundary() {
    // 偶数幅（width = 6）ではインターリーブの行幅は 2 * ceil(width / 2) = width になる。
    // stride == ceil(width / 2) = 3 は平面の最小幅であり、インターリーブには不足する
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
    let size = ImageSize::new(width, height);

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
    let result = android420_to_i420(&src, 2, &mut dst, size);
    let err = result.expect_err("インターリーブ幅未満の U ストライドでは Err が返るべき");
    assert!(
        err.to_string()
            .contains("U stride smaller than interleaved chroma width"),
        "U 側のインターリーブ stride 不足の reason が返るべき: {}",
        err
    );

    // バッファをインターリーブ幅（= width）に合わせ、同一バッファの連続領域で渡せば成功する
    let buf = vec![0u8; halfwidth * 2 * uv_height + 1];
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &buf[..buf.len() - 1],
        u_stride: halfwidth * 2,
        v: &buf[1..],
        v_stride: halfwidth * 2,
    };
    android420_to_i420(&src, 2, &mut dst, size)
        .expect("インターリーブ幅の U ストライドでは成功するべき");
}

// 正常系: android420_to_i420 が pixel_stride_uv == 2 で NV12 と同等の変換を行うこと
#[test]
fn android420_to_i420_interleaved_matches_nv12() {
    // pixel_stride_uv == 2 では libyuv は高速パス（NV12 変換）に入るため、
    // 同じデータを NV12 として変換した結果と一致する。
    // u / v は同一バッファの連続領域（vu_off == 1）で渡す。末尾 1 バイトは
    // v 側の検証（len >= stride * ceil(height / 2)）を満たすためのパディング
    let width: usize = 5;
    let height: usize = 3;
    let halfwidth = width.div_ceil(2);
    let uv_height = height.div_ceil(2);
    let uv_stride = halfwidth * 2;

    let y: Vec<u8> = (0..(width * height)).map(|i| i as u8).collect();
    let buf: Vec<u8> = (0..(uv_stride * uv_height + 1)).map(|i| i as u8).collect();
    let u = &buf[..buf.len() - 1];
    let v = &buf[1..];
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u,
        u_stride: uv_stride,
        v,
        v_stride: uv_stride,
    };
    let size = ImageSize::new(width, height);

    let mut nv12_y = vec![0u8; width * height];
    let mut nv12_u = vec![0u8; halfwidth * uv_height];
    let mut nv12_v = vec![0u8; halfwidth * uv_height];
    {
        let nv12_src = Nv12Image {
            y: &y,
            y_stride: width,
            uv: &buf[..uv_stride * uv_height],
            uv_stride,
        };
        let mut nv12_dst = I420ImageMut {
            y: &mut nv12_y,
            y_stride: width,
            u: &mut nv12_u,
            u_stride: halfwidth,
            v: &mut nv12_v,
            v_stride: halfwidth,
        };
        nv12_to_i420(&nv12_src, &mut nv12_dst, size).expect("NV12 変換が成功すること");
    }

    let mut and_y = vec![0u8; width * height];
    let mut and_u = vec![0u8; halfwidth * uv_height];
    let mut and_v = vec![0u8; halfwidth * uv_height];
    {
        let mut and_dst = I420ImageMut {
            y: &mut and_y,
            y_stride: width,
            u: &mut and_u,
            u_stride: halfwidth,
            v: &mut and_v,
            v_stride: halfwidth,
        };
        android420_to_i420(&src, 2, &mut and_dst, size)
            .expect("pixel_stride_uv == 2 の変換が成功すること");
    }

    assert_eq!(and_y, nv12_y, "Y プレーンが NV12 変換と一致すること");
    assert_eq!(and_u, nv12_u, "U プレーンが NV12 変換と一致すること");
    assert_eq!(and_v, nv12_v, "V プレーンが NV12 変換と一致すること");
}

// 正常系: android420_to_i420 が pixel_stride_uv == 2 でストライド不一致でも変換できること
#[test]
fn android420_to_i420_interleaved_unequal_strides_ok() {
    // 高速パスは u_stride == v_stride を要求する。不一致の場合は libyuv のフォールバック
    // （SplitPixels）に入るが、U/V の実効行幅は 2 * halfwidth のままであるため、
    // 検証は u / v それぞれのストライドに対して要求される。
    // u / v は同一バッファの連続領域で渡し、v 側のストライドを 1 バイト大きくする
    let width = 5;
    let height = 3;
    let halfwidth = 3;
    let uv_height = 2;
    let y = vec![0u8; width * height];
    let buf = vec![0u8; halfwidth * 2 * uv_height + 1 + uv_height];
    let u = &buf[..halfwidth * 2 * uv_height + 1];
    let v = &buf[1..];
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u,
        u_stride: halfwidth * 2,
        v,
        v_stride: halfwidth * 2 + 1,
    };
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; halfwidth * uv_height];
    let mut v_dst = vec![0u8; halfwidth * uv_height];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: halfwidth,
        v: &mut v_dst,
        v_stride: halfwidth,
    };
    let size = ImageSize::new(width, height);

    android420_to_i420(&src, 2, &mut dst, size).expect("ストライド不一致でも変換が成功すること");
}

// 異常系: android420_to_argb が pixel_stride_uv の 1 / 2 以外を Err にすること
#[test]
fn android420_to_argb_pixel_stride_uv_invalid() {
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
    let mut data = vec![0u8; 8 * 4 * 4];
    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: 8 * 4,
    };
    let size = ImageSize::new(8, 4);

    let result = android420_to_argb(&src, 3, &mut dst, size);
    let err = result.expect_err("pixel_stride_uv が 1 / 2 以外では Err が返るべき");
    assert!(
        err.to_string().contains("pixel_stride_uv must be 1 or 2"),
        "pixel_stride_uv の値検証の reason が返るべき: {}",
        err
    );
}

// 異常系: android420_to_argb が pixel_stride_uv == 2 でインターリーブ幅未満の U ストライドを Err にすること
#[test]
fn android420_to_argb_interleaved_stride_boundary() {
    // 奇数幅（width = 5）ではインターリーブの行幅は 2 * ceil(width / 2) = 6 バイト
    let width = 5;
    let height = 3;
    let halfwidth = 3;
    let uv_height = 2;
    let y = vec![0u8; width * height];
    let u = vec![0u8; halfwidth * uv_height];
    let v = vec![0u8; halfwidth * uv_height];
    let mut data = vec![0u8; width * height * 4];
    let size = ImageSize::new(width, height);

    let mut dst = ArgbImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: halfwidth,
        v: &v,
        v_stride: halfwidth,
    };
    let result = android420_to_argb(&src, 2, &mut dst, size);
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
    let result = android420_to_argb(&src, 2, &mut dst, size);
    let err = result.expect_err("インターリーブ幅未満の V ストライドでは Err が返るべき");
    assert!(
        err.to_string()
            .contains("V stride smaller than interleaved chroma width"),
        "V 側のインターリーブ stride 不足の reason が返るべき: {}",
        err
    );

    // バッファをインターリーブ幅に合わせ、同一バッファの連続領域で渡せば成功する
    let buf = vec![0u8; halfwidth * 2 * uv_height + 1];
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &buf[..buf.len() - 1],
        u_stride: halfwidth * 2,
        v: &buf[1..],
        v_stride: halfwidth * 2,
    };
    android420_to_argb(&src, 2, &mut dst, size)
        .expect("インターリーブ幅の U ストライドでは成功するべき");
}

// 異常系: android420_to_abgr が pixel_stride_uv の 1 / 2 以外を Err にすること
#[test]
fn android420_to_abgr_pixel_stride_uv_invalid() {
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
    let mut data = vec![0u8; 8 * 4 * 4];
    let mut dst = AbgrImageMut {
        data: &mut data,
        stride: 8 * 4,
    };
    let size = ImageSize::new(8, 4);

    let result = android420_to_abgr(&src, 3, &mut dst, size);
    let err = result.expect_err("pixel_stride_uv が 1 / 2 以外では Err が返るべき");
    assert!(
        err.to_string().contains("pixel_stride_uv must be 1 or 2"),
        "pixel_stride_uv の値検証の reason が返るべき: {}",
        err
    );
}

// 異常系: android420_to_abgr が pixel_stride_uv == 2 でインターリーブ幅未満の U ストライドを Err にすること
#[test]
fn android420_to_abgr_interleaved_stride_boundary() {
    // 奇数幅（width = 5）ではインターリーブの行幅は 2 * ceil(width / 2) = 6 バイト
    let width = 5;
    let height = 3;
    let halfwidth = 3;
    let uv_height = 2;
    let y = vec![0u8; width * height];
    let u = vec![0u8; halfwidth * uv_height];
    let v = vec![0u8; halfwidth * uv_height];
    let mut data = vec![0u8; width * height * 4];
    let size = ImageSize::new(width, height);

    let mut dst = AbgrImageMut {
        data: &mut data,
        stride: width * 4,
    };
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: halfwidth,
        v: &v,
        v_stride: halfwidth,
    };
    let result = android420_to_abgr(&src, 2, &mut dst, size);
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
    let result = android420_to_abgr(&src, 2, &mut dst, size);
    let err = result.expect_err("インターリーブ幅未満の V ストライドでは Err が返るべき");
    assert!(
        err.to_string()
            .contains("V stride smaller than interleaved chroma width"),
        "V 側のインターリーブ stride 不足の reason が返るべき: {}",
        err
    );

    // バッファをインターリーブ幅に合わせ、同一バッファの連続領域で渡せば成功する
    let buf = vec![0u8; halfwidth * 2 * uv_height + 1];
    let src = Android420Image {
        y: &y,
        y_stride: width,
        u: &buf[..buf.len() - 1],
        u_stride: halfwidth * 2,
        v: &buf[1..],
        v_stride: halfwidth * 2,
    };
    android420_to_abgr(&src, 2, &mut dst, size)
        .expect("インターリーブ幅の U ストライドでは成功するべき");
}

// 異常系: mm21_to_i420 がタイル配置の必要サイズを下回る Y バッファを Err にすること
#[test]
fn mm21_to_i420_tiled_y_buffer_boundary() {
    // width=5, height=3: タイル行幅 16（16 の倍数に切り上げ）、タイル行数 1。
    // Y 必要サイズ = 16 * 32 = 512（線形サイズ 5 * 3 = 15 より過大）
    let width = 5;
    let height = 3;
    let uv = vec![0u8; 256];
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; 3 * 2];
    let mut v_dst = vec![0u8; 3 * 2];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: 3,
        v: &mut v_dst,
        v_stride: 3,
    };
    let size = ImageSize::new(width, height);

    let y = vec![0u8; 511]; // 1 バイト不足
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 6,
    };
    let result = mm21_to_i420(&src, &mut dst, size);
    let err = result.expect_err("タイル配置の必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のタイル必要サイズ不足の reason が返るべき: {}",
        err
    );

    // 必要サイズちょうどでは成功する
    let y = vec![0u8; 512];
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 6,
    };
    mm21_to_i420(&src, &mut dst, size).expect("必要サイズちょうどで成功するべき");
}

// 異常系: mm21_to_i420 がタイル配置の必要サイズを下回る UV バッファを Err にすること
#[test]
fn mm21_to_i420_tiled_uv_buffer_boundary() {
    // UV 必要サイズ = 16 * 16 = 256（タイル行幅 16、タイル高 16、タイル行数 1）
    let width = 5;
    let height = 3;
    let y = vec![0u8; 512];
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; 3 * 2];
    let mut v_dst = vec![0u8; 3 * 2];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: 3,
        v: &mut v_dst,
        v_stride: 3,
    };
    let size = ImageSize::new(width, height);

    let uv = vec![0u8; 255]; // 1 バイト不足
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 6,
    };
    let result = mm21_to_i420(&src, &mut dst, size);
    let err = result.expect_err("タイル配置の必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source UV buffer too small"),
        "UV 側のタイル必要サイズ不足の reason が返るべき: {}",
        err
    );

    // 必要サイズちょうどでは成功する
    let uv = vec![0u8; 256];
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 6,
    };
    mm21_to_i420(&src, &mut dst, size).expect("必要サイズちょうどで成功するべき");
}

// 異常系: mm21_to_i420 がタイル高非倍数 × 幅 16 倍数のタイル必要サイズを検証すること
#[test]
fn mm21_to_i420_tiled_buffer_multi_tile_row_boundary() {
    // width=16, height=33: タイル行幅 16、タイル行数 2（33 は 32 の倍数でない）。
    // Y 必要サイズ = (2 - 1) * 16 * 32 + 16 * 32 = 1024（線形サイズ 16 * 33 = 528 より過大）
    let width = 16;
    let height = 33;
    let uv = vec![0u8; 512];
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; 8 * 17];
    let mut v_dst = vec![0u8; 8 * 17];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: 8,
        v: &mut v_dst,
        v_stride: 8,
    };
    let size = ImageSize::new(width, height);

    let y = vec![0u8; 1023]; // 1 バイト不足
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let result = mm21_to_i420(&src, &mut dst, size);
    let err = result.expect_err("タイル配置の必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のタイル必要サイズ不足の reason が返るべき: {}",
        err
    );

    // 必要サイズちょうどでは成功する
    let y = vec![0u8; 1024];
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    mm21_to_i420(&src, &mut dst, size).expect("必要サイズちょうどで成功するべき");

    // UV 側もタイル行数 2 の境界を検証する。
    // UV 必要サイズ = (2 - 1) * 16 * 16 + 16 * 16 = 512
    let uv = vec![0u8; 511]; // 1 バイト不足
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let result = mm21_to_i420(&src, &mut dst, size);
    let err = result.expect_err("タイル配置の必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source UV buffer too small"),
        "UV 側のタイル必要サイズ不足の reason が返るべき: {}",
        err
    );

    // 必要サイズちょうどでは成功する
    let uv = vec![0u8; 512];
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    mm21_to_i420(&src, &mut dst, size).expect("必要サイズちょうどで成功するべき");
}

// 異常系: mm21_to_i420 がパディング付きストライドのタイル行間隔を検証すること
#[test]
fn mm21_to_i420_tiled_buffer_padded_stride_boundary() {
    // width=16, height=33, y_stride=32（パディング付き）:
    // Y 必要サイズ = (2 - 1) * 32 * 32 + 16 * 32 = 1536
    // （y_stride == width なら (2 - 1) * 16 * 32 + 512 = 1024 になり、
    //  stride がタイル行間隔に使われることを直接検証できる）
    let width = 16;
    let height = 33;
    let y_stride = 32;
    let uv = vec![0u8; 512];
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; 8 * 17];
    let mut v_dst = vec![0u8; 8 * 17];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: 8,
        v: &mut v_dst,
        v_stride: 8,
    };
    let size = ImageSize::new(width, height);

    let y = vec![0u8; 1535]; // 1 バイト不足
    let src = Mm21Image {
        y: &y,
        y_stride,
        uv: &uv,
        uv_stride: width,
    };
    let result = mm21_to_i420(&src, &mut dst, size);
    let err = result.expect_err("タイル配置の必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のタイル必要サイズ不足の reason が返るべき: {}",
        err
    );

    // 必要サイズちょうどでは成功する
    let y = vec![0u8; 1536];
    let src = Mm21Image {
        y: &y,
        y_stride,
        uv: &uv,
        uv_stride: width,
    };
    mm21_to_i420(&src, &mut dst, size).expect("必要サイズちょうどで成功するべき");
}

// 異常系: mm21_to_i420 がタイル高倍数のタイル必要サイズを検証すること
#[test]
fn mm21_to_i420_tiled_buffer_tile_height_multiple_boundary() {
    // height = 32（タイル高 32 ちょうどの倍数）でも、幅が 16 の倍数でないと
    // タイル行幅 16 の必要サイズになる。Y 必要サイズ = 16 * 32 = 512
    // （線形サイズ 5 * 32 = 160 より過大）。
    // 5x3 と同じ境界だが、タイル行数が ceil(32 / 32) = 1 になることを
    // 検証する（タイル高 32 の定数が 16 に退行すると必要サイズが変わり検出される）
    let width = 5;
    let height = 32;
    let uv = vec![0u8; 256];
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; 3 * 16];
    let mut v_dst = vec![0u8; 3 * 16];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: 3,
        v: &mut v_dst,
        v_stride: 3,
    };
    let size = ImageSize::new(width, height);

    let y = vec![0u8; 511]; // 1 バイト不足
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 6,
    };
    let result = mm21_to_i420(&src, &mut dst, size);
    let err = result.expect_err("タイル配置の必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のタイル必要サイズ不足の reason が返るべき: {}",
        err
    );

    // 必要サイズちょうどでは成功する
    let y = vec![0u8; 512];
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 6,
    };
    mm21_to_i420(&src, &mut dst, size).expect("必要サイズちょうどで成功するべき");

    // 幅が 16 の倍数でタイル行数 1 の場合は、タイル必要サイズが線形サイズと一致する。
    // Y 必要サイズ = 16 * 32 = 512 = 線形サイズ
    let width = 16;
    let height = 32;
    let mut y_dst = vec![0u8; width * height];
    let mut u_dst = vec![0u8; 8 * 16];
    let mut v_dst = vec![0u8; 8 * 16];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: width,
        u: &mut u_dst,
        u_stride: 8,
        v: &mut v_dst,
        v_stride: 8,
    };
    let size = ImageSize::new(width, height);
    let y = vec![0u8; 511]; // 1 バイト不足
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let result = mm21_to_i420(&src, &mut dst, size);
    let err = result.expect_err("タイル配置の必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のタイル必要サイズ不足の reason が返るべき: {}",
        err
    );

    // 必要サイズちょうどでは成功する
    let y = vec![0u8; 512];
    let src = Mm21Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    mm21_to_i420(&src, &mut dst, size).expect("必要サイズちょうどで成功するべき");
}

// 異常系: mt2t_to_p010 が 10bit パックのタイル必要サイズを下回るバッファを Err にすること
#[test]
fn mt2t_to_p010_tiled_buffer_boundary() {
    // width=5, height=3: タイル行幅 16、タイル行数 1。
    // 10bit パック（10/8 倍）で Y 必要サイズ = 16 * 32 * 10 / 8 = 640、
    // UV 必要サイズ = 16 * 16 * 10 / 8 = 320（MM21 の 512 / 256 より過大）
    let width = 5;
    let height = 3;
    let mut y_dst = vec![0u16; width * height];
    let mut uv_dst = vec![0u16; 6 * 2];
    let mut dst = P010ImageMut {
        y: &mut y_dst,
        y_stride: width,
        uv: &mut uv_dst,
        uv_stride: 6,
    };
    let size = ImageSize::new(width, height);

    let y = vec![0u8; 639]; // 1 バイト不足
    let uv = vec![0u8; 320];
    let src = Mt2tImage {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 6,
    };
    let result = mt2t_to_p010(&src, &mut dst, size);
    let err = result.expect_err("10bit パックの必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側の 10bit 必要サイズ不足の reason が返るべき: {}",
        err
    );

    // Y 必要サイズちょうど、UV が 1 バイト不足
    let y = vec![0u8; 640];
    let uv = vec![0u8; 319];
    let src = Mt2tImage {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 6,
    };
    let result = mt2t_to_p010(&src, &mut dst, size);
    let err = result.expect_err("UV の 10bit 必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source UV buffer too small"),
        "UV 側の 10bit 必要サイズ不足の reason が返るべき: {}",
        err
    );

    // 必要サイズちょうどでは成功する
    let uv = vec![0u8; 320];
    let src = Mt2tImage {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 6,
    };
    mt2t_to_p010(&src, &mut dst, size).expect("必要サイズちょうどで成功するべき");
}

// 異常系: mt2t_to_p010 が 10bit パック × 複数タイル行の必要サイズを検証すること
#[test]
fn mt2t_to_p010_tiled_multi_tile_row_boundary() {
    // width=16, height=33: タイル行数 2。
    // Y 必要サイズ = (2 - 1) * 16 * 32 + 16 * 32 * 10 / 8 = 512 + 640 = 1152
    // UV 必要サイズ = (2 - 1) * 16 * 16 + 16 * 16 * 10 / 8 = 256 + 320 = 576
    let width = 16;
    let height = 33;
    let mut y_dst = vec![0u16; width * height];
    let mut uv_dst = vec![0u16; 16 * 17];
    let mut dst = P010ImageMut {
        y: &mut y_dst,
        y_stride: width,
        uv: &mut uv_dst,
        uv_stride: width,
    };
    let size = ImageSize::new(width, height);

    let y = vec![0u8; 1151]; // 1 バイト不足
    let uv = vec![0u8; 576];
    let src = Mt2tImage {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let result = mt2t_to_p010(&src, &mut dst, size);
    let err = result.expect_err("10bit パックの必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側の 10bit 必要サイズ不足の reason が返るべき: {}",
        err
    );

    // Y 必要サイズちょうど、UV が 1 バイト不足
    let y = vec![0u8; 1152];
    let uv = vec![0u8; 575];
    let src = Mt2tImage {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let result = mt2t_to_p010(&src, &mut dst, size);
    let err = result.expect_err("UV の 10bit 必要サイズ未満では Err が返るべき");
    assert!(
        err.to_string().contains("source UV buffer too small"),
        "UV 側の 10bit 必要サイズ不足の reason が返るべき: {}",
        err
    );

    // 必要サイズちょうどでは成功する
    let uv = vec![0u8; 576];
    let src = Mt2tImage {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    mt2t_to_p010(&src, &mut dst, size).expect("必要サイズちょうどで成功するべき");
}

// 異常系: mm21_to_i420 がゼロサイズ入力で Err を返すこと
#[test]
fn mm21_to_i420_zero_size() {
    // width == 0 / height == 0 のどちらでも同一の reason で Err になること
    let y = vec![0u8; 512];
    let uv = vec![0u8; 256];
    let mut y_dst = vec![0u8; 5 * 3];
    let mut u_dst = vec![0u8; 3 * 2];
    let mut v_dst = vec![0u8; 3 * 2];
    let mut dst = I420ImageMut {
        y: &mut y_dst,
        y_stride: 5,
        u: &mut u_dst,
        u_stride: 3,
        v: &mut v_dst,
        v_stride: 3,
    };

    let src = Mm21Image {
        y: &y,
        y_stride: 5,
        uv: &uv,
        uv_stride: 6,
    };
    let size = ImageSize::new(0, 3);
    let result = mm21_to_i420(&src, &mut dst, size);
    let err = result.expect_err("width == 0 では Err が返るべき");
    assert!(
        err.to_string()
            .contains("width and height must be greater than 0"),
        "ゼロサイズの reason が返るべき: {}",
        err
    );

    let size = ImageSize::new(5, 0);
    let result = mm21_to_i420(&src, &mut dst, size);
    let err = result.expect_err("height == 0 では Err が返るべき");
    assert!(
        err.to_string()
            .contains("width and height must be greater than 0"),
        "ゼロサイズの reason が返るべき: {}",
        err
    );
}
