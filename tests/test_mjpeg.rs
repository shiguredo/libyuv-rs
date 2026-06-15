//! MJPEG 変換 API の単体テスト
//!
//! `src/test_data/` 配下の固定 JPEG バイト列を用いて、正常系・異常系の挙動を網羅する。
//! 入力 JPEG はサブサンプリング (4:2:0 / 4:2:2 / 4:4:4) ごとに 1 ファイル用意してある。

use std::ffi::c_int;

use shiguredo_libyuv::{
    ArgbImageMut, I420ImageMut, ImageSize, Nv12ImageMut, Nv21ImageMut, mjpeg_size, mjpeg_to_argb,
    mjpeg_to_i420, mjpeg_to_nv12, mjpeg_to_nv21,
};

// テスト対象の JPEG ファイル (バイト列ごと埋め込む)
const JPEG_8X8_YUV420: &[u8] = include_bytes!("../src/test_data/mjpeg_8x8_yuv420.jpg");
const JPEG_16X16_YUV422: &[u8] = include_bytes!("../src/test_data/mjpeg_16x16_yuv422.jpg");
const JPEG_64X48_YUV444: &[u8] = include_bytes!("../src/test_data/mjpeg_64x48_yuv444.jpg");

// (JPEG バイト列, 期待される画像サイズ) の組
const SAMPLES: &[(&[u8], ImageSize)] = &[
    (JPEG_8X8_YUV420, ImageSize::new(8, 8)),
    (JPEG_16X16_YUV422, ImageSize::new(16, 16)),
    (JPEG_64X48_YUV444, ImageSize::new(64, 48)),
];

// I420 の chroma 幅 / 高さ (4:2:0 サブサンプリング)
fn i420_uv_dims(size: ImageSize) -> (usize, usize) {
    (size.width.div_ceil(2), size.height.div_ceil(2))
}

// I420 出力バッファ + ストライド (libyuv が要求する最小サイズで確保する)
fn alloc_i420(size: ImageSize) -> (Vec<u8>, Vec<u8>, Vec<u8>, usize, usize) {
    let (uv_w, uv_h) = i420_uv_dims(size);
    let y = vec![0u8; size.width * size.height];
    let u = vec![0u8; uv_w * uv_h];
    let v = vec![0u8; uv_w * uv_h];
    (y, u, v, size.width, uv_w)
}

// NV12 / NV21 出力バッファ + ストライド (UV はインターリーブなので最小ストライドは ceil(w/2)*2)
fn alloc_nv(size: ImageSize) -> (Vec<u8>, Vec<u8>, usize, usize) {
    let (uv_w, uv_h) = i420_uv_dims(size);
    let uv_stride = uv_w * 2;
    let y = vec![0u8; size.width * size.height];
    let uv = vec![0u8; uv_stride * uv_h];
    (y, uv, size.width, uv_stride)
}

// ARGB 出力バッファ + ストライド (4 bytes / pixel)
fn alloc_argb(size: ImageSize) -> (Vec<u8>, usize) {
    let stride = size.width * 4;
    (vec![0u8; stride * size.height], stride)
}

// ----------------------------------------------------------------
// 正常系テスト
// ----------------------------------------------------------------

#[test]
fn mjpeg_size_returns_expected_dimensions() {
    // 各サブサンプリングのテストデータについて、JPEG ヘッダから読み取った
    // 幅・高さがコミット済みのファイル名と一致することを確認する
    for (jpeg, expected) in SAMPLES {
        let size = mjpeg_size(jpeg).expect("mjpeg_size should succeed for known good JPEG");
        assert_eq!(
            size.width, expected.width,
            "幅がファイル名と一致しません: 期待 {} 実測 {}",
            expected.width, size.width
        );
        assert_eq!(
            size.height, expected.height,
            "高さがファイル名と一致しません: 期待 {} 実測 {}",
            expected.height, size.height
        );
    }
}

#[test]
fn mjpeg_to_i420_decodes_into_non_zero_buffer() {
    // テストデータは gradient:red-blue で生成されているため、Y プレーンの中央付近は
    // 必ず非ゼロになる。デコード後にバッファが全 0 のままなら libyuv が何も書いていない
    // (= 戻り値 Ok だが実際は失敗) ことを意味するので、その状態を検出する。
    for (jpeg, size) in SAMPLES {
        let (mut y, mut u, mut v, y_stride, uv_stride) = alloc_i420(*size);
        let mut dst = I420ImageMut {
            y: &mut y,
            y_stride,
            u: &mut u,
            u_stride: uv_stride,
            v: &mut v,
            v_stride: uv_stride,
        };
        mjpeg_to_i420(jpeg, &mut dst, *size).expect("mjpeg_to_i420 succeeds for known good JPEG");
        assert!(
            y.iter().any(|&b| b != 0),
            "Y プレーンが全 0: デコードが実際には行われていない可能性 (size = {} x {})",
            size.width,
            size.height
        );
    }
}

#[test]
fn mjpeg_to_nv12_decodes_into_non_zero_buffer() {
    for (jpeg, size) in SAMPLES {
        let (mut y, mut uv, y_stride, uv_stride) = alloc_nv(*size);
        let mut dst = Nv12ImageMut {
            y: &mut y,
            y_stride,
            uv: &mut uv,
            uv_stride,
        };
        mjpeg_to_nv12(jpeg, &mut dst, *size).expect("mjpeg_to_nv12 succeeds for known good JPEG");
        assert!(
            y.iter().any(|&b| b != 0),
            "Y プレーンが全 0: NV12 デコードが行われていない可能性"
        );
    }
}

#[test]
fn mjpeg_to_nv21_decodes_into_non_zero_buffer() {
    for (jpeg, size) in SAMPLES {
        let (mut y, mut uv, y_stride, uv_stride) = alloc_nv(*size);
        let mut dst = Nv21ImageMut {
            y: &mut y,
            y_stride,
            uv: &mut uv,
            uv_stride,
        };
        mjpeg_to_nv21(jpeg, &mut dst, *size).expect("mjpeg_to_nv21 succeeds for known good JPEG");
        assert!(
            y.iter().any(|&b| b != 0),
            "Y プレーンが全 0: NV21 デコードが行われていない可能性"
        );
    }
}

#[test]
fn mjpeg_to_argb_decodes_with_alpha_channel_set() {
    for (jpeg, size) in SAMPLES {
        let (mut data, stride) = alloc_argb(*size);
        let mut dst = ArgbImageMut {
            data: &mut data,
            stride,
        };
        mjpeg_to_argb(jpeg, &mut dst, *size).expect("mjpeg_to_argb succeeds for known good JPEG");
        // libyuv の MJPGToARGB は alpha channel に 0xFF を書く。最初の 4 バイトの
        // alpha byte が 0xFF なら ARGB が実際に書かれている (リトルエンディアン上
        // ARGB のメモリ表現は B, G, R, A の順)
        assert_eq!(
            data[3], 0xFF,
            "ARGB のアルファチャンネルが 0xFF に書かれていない (デコードが行われていない可能性)"
        );
    }
}

// ----------------------------------------------------------------
// 異常系テスト
// ----------------------------------------------------------------

#[test]
fn mjpeg_size_rejects_empty_buffer() {
    // 空入力は MJPGSize 呼び出し前に弾かれること
    assert!(mjpeg_size(&[]).is_err(), "空入力を Ok にしてはいけない");
}

#[test]
fn mjpeg_size_rejects_corrupted_soi_marker() {
    // 先頭 2 バイトを 0 に書き換えた JPEG は SOI マーカー (0xFF 0xD8) が壊れているので、
    // mjpeg_size 自体がエラーを返すか、戻り値ゼロを検出して Err にすべき。
    // libyuv の MJPGSize は失敗時に負の値を返す挙動だが、念のため最終的に Err になることを検査する。
    let mut corrupted = JPEG_8X8_YUV420.to_vec();
    corrupted[0] = 0x00;
    corrupted[1] = 0x00;
    assert!(
        mjpeg_size(&corrupted).is_err(),
        "SOI 破壊 JPEG を mjpeg_size が受理してはいけない"
    );
}

#[test]
fn mjpeg_to_all_reject_empty_buffer() {
    // 空入力は MJPGTo* 系すべてで Err になること
    let size = ImageSize::new(8, 8);
    let (mut y, mut u, mut v, y_stride, uv_stride) = alloc_i420(size);
    let mut i420 = I420ImageMut {
        y: &mut y,
        y_stride,
        u: &mut u,
        u_stride: uv_stride,
        v: &mut v,
        v_stride: uv_stride,
    };
    assert!(mjpeg_to_i420(&[], &mut i420, size).is_err());

    let (mut y, mut uv, y_stride, uv_stride) = alloc_nv(size);
    let mut nv12 = Nv12ImageMut {
        y: &mut y,
        y_stride,
        uv: &mut uv,
        uv_stride,
    };
    assert!(mjpeg_to_nv12(&[], &mut nv12, size).is_err());

    let (mut y, mut uv, y_stride, uv_stride) = alloc_nv(size);
    let mut nv21 = Nv21ImageMut {
        y: &mut y,
        y_stride,
        uv: &mut uv,
        uv_stride,
    };
    assert!(mjpeg_to_nv21(&[], &mut nv21, size).is_err());

    let (mut data, stride) = alloc_argb(size);
    let mut argb = ArgbImageMut {
        data: &mut data,
        stride,
    };
    assert!(mjpeg_to_argb(&[], &mut argb, size).is_err());
}

#[test]
fn mjpeg_all_reject_corrupted_soi_marker() {
    // 先頭 2 バイト (SOI マーカー 0xFF 0xD8) を破壊した JPEG で MJPGTo* 全関数が Err になること
    let mut corrupted = JPEG_8X8_YUV420.to_vec();
    corrupted[0] = 0x00;
    corrupted[1] = 0x00;
    let size = ImageSize::new(8, 8);

    let (mut y, mut u, mut v, y_stride, uv_stride) = alloc_i420(size);
    let mut i420 = I420ImageMut {
        y: &mut y,
        y_stride,
        u: &mut u,
        u_stride: uv_stride,
        v: &mut v,
        v_stride: uv_stride,
    };
    assert!(
        mjpeg_to_i420(&corrupted, &mut i420, size).is_err(),
        "SOI 破壊 JPEG を MJPGToI420 が受理してはいけない"
    );

    let (mut y, mut uv, y_stride, uv_stride) = alloc_nv(size);
    let mut nv12 = Nv12ImageMut {
        y: &mut y,
        y_stride,
        uv: &mut uv,
        uv_stride,
    };
    assert!(mjpeg_to_nv12(&corrupted, &mut nv12, size).is_err());

    let (mut y, mut uv, y_stride, uv_stride) = alloc_nv(size);
    let mut nv21 = Nv21ImageMut {
        y: &mut y,
        y_stride,
        uv: &mut uv,
        uv_stride,
    };
    assert!(mjpeg_to_nv21(&corrupted, &mut nv21, size).is_err());

    let (mut data, stride) = alloc_argb(size);
    let mut argb = ArgbImageMut {
        data: &mut data,
        stride,
    };
    assert!(mjpeg_to_argb(&corrupted, &mut argb, size).is_err());
}

#[test]
fn mjpeg_to_i420_rejects_eoi_truncated_jpeg() {
    // 末尾の EOI マーカー (0xFF 0xD9) を含めて末尾数バイトを削った JPEG は完全デコードできない。
    // mjpeg_size はヘッダのみで成功する可能性があるが、MJPGToI420 はエントロピー復号が
    // 中途で終わるため失敗する。
    let truncated = &JPEG_8X8_YUV420[..JPEG_8X8_YUV420.len() - 4];
    let size = ImageSize::new(8, 8);
    let (mut y, mut u, mut v, y_stride, uv_stride) = alloc_i420(size);
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride,
        u: &mut u,
        u_stride: uv_stride,
        v: &mut v,
        v_stride: uv_stride,
    };
    assert!(
        mjpeg_to_i420(truncated, &mut dst, size).is_err(),
        "EOI 欠落 JPEG を MJPGToI420 が受理してはいけない"
    );
}

#[test]
fn mjpeg_to_i420_rejects_size_mismatch() {
    // JPEG ヘッダの実サイズ (8x8) と size 引数 (16x16) が不一致な場合は libyuv が
    // 戻り値でエラーを返すこと
    let wrong_size = ImageSize::new(16, 16);
    let (mut y, mut u, mut v, y_stride, uv_stride) = alloc_i420(wrong_size);
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride,
        u: &mut u,
        u_stride: uv_stride,
        v: &mut v,
        v_stride: uv_stride,
    };
    assert!(
        mjpeg_to_i420(JPEG_8X8_YUV420, &mut dst, wrong_size).is_err(),
        "サイズ不一致を MJPGToI420 が受理してはいけない"
    );
}

#[test]
fn mjpeg_to_i420_rejects_insufficient_destination_buffer() {
    // dst バッファサイズが size に対して不足するとき Err になること
    let size = ImageSize::new(8, 8);
    // Y プレーンを 1 バイト不足させる
    let mut y = vec![0u8; size.width * size.height - 1];
    let (uv_w, uv_h) = i420_uv_dims(size);
    let mut u = vec![0u8; uv_w * uv_h];
    let mut v = vec![0u8; uv_w * uv_h];
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride: size.width,
        u: &mut u,
        u_stride: uv_w,
        v: &mut v,
        v_stride: uv_w,
    };
    assert!(
        mjpeg_to_i420(JPEG_8X8_YUV420, &mut dst, size).is_err(),
        "Y バッファ不足を MJPGToI420 が受理してはいけない"
    );
}

#[test]
fn mjpeg_to_argb_rejects_zero_size() {
    // 呼び出し側が size = (0, 0) で MJPGTo* を呼んだら Err になること
    // (mjpeg_size 自体は src のみを引数に取るので、ここでは MJPGToARGB を検査)
    let zero = ImageSize::new(0, 0);
    let mut data = [0u8; 16];
    let mut argb = ArgbImageMut {
        data: &mut data,
        stride: 4,
    };
    assert!(
        mjpeg_to_argb(JPEG_8X8_YUV420, &mut argb, zero).is_err(),
        "ゼロサイズを MJPGToARGB が受理してはいけない"
    );
}

#[test]
fn mjpeg_to_argb_rejects_width_overflow() {
    // size.width が c_int 範囲を超えた場合の保護 (require_c_int(size.width) 経路)
    let overflow = ImageSize::new(c_int::MAX as usize + 1, 1);
    let mut data = [0u8; 16];
    let mut argb = ArgbImageMut {
        data: &mut data,
        stride: 4,
    };
    assert!(
        mjpeg_to_argb(JPEG_8X8_YUV420, &mut argb, overflow).is_err(),
        "c_int を超える幅を MJPGToARGB が受理してはいけない"
    );
}

#[test]
fn mjpeg_to_i420_rejects_height_overflow() {
    // size.height が c_int 範囲を超えた場合の保護 (require_c_int(size.height) 経路)
    let overflow = ImageSize::new(1, c_int::MAX as usize + 1);
    let mut y = [0u8; 16];
    let mut u = [0u8; 16];
    let mut v = [0u8; 16];
    let mut dst = I420ImageMut {
        y: &mut y,
        y_stride: 1,
        u: &mut u,
        u_stride: 1,
        v: &mut v,
        v_stride: 1,
    };
    assert!(
        mjpeg_to_i420(JPEG_8X8_YUV420, &mut dst, overflow).is_err(),
        "c_int を超える高さを MJPGToI420 が受理してはいけない"
    );
}
