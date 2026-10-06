//! nv サブモジュールの代表関数の単体テスト
//!
//! `src/convert/nv.rs` に対応する。代表関数として
//! `nv12_to_raw` (Y 抽出)・`nv21_to_yuv24` (NV21→YUV24)・`i444_to_nv12` (UV パック) を選定し、
//! ground truth 方式の正常系とエラーパス (バッファ不足・stride 不足・c_int 超過) を検証する。
//! `nv12_to_nv24` / `nv16_to_nv24` は 0048 のスコープのため選定しない。

use std::ffi::c_int;

use shiguredo_libyuv::{
    I444Image, ImageSize, Nv12Image, Nv12ImageMut, Nv21Image, RawImageMut, Yuv24ImageMut,
    i444_to_nv12, nv12_to_raw, nv21_to_yuv24,
};

use super::helpers;

// ============================================================
// nv12_to_raw (Y 抽出 + RGB 変換)
// ============================================================

// 正常系: nv12_to_raw が既知 NV12 から期待 RAW を出力すること
///
/// RAW は 3 bytes/pixel でメモリ順 R,G,B (NV12ToRAW は NV21ToRGB24Matrix 経由で
/// ARGBToRGB24Row_C と同じ R,G,B 順に書く)。
#[test]
fn nv12_to_raw_known_colors() {
    let width = 4;
    let height = 1;
    let y = vec![16u8, 81, 145, 235];
    let uv = vec![128u8, 128, 90, 240]; // U,V インターリーブ (U が偶数番目)
    let src = Nv12Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: 4,
    };
    let mut raw = vec![0u8; width * height * 3];
    let mut dst = RawImageMut {
        data: &mut raw,
        stride: width * 3,
    };
    let size = ImageSize::new(width, height);

    nv12_to_raw(&src, &mut dst, size).expect("NV12 から RAW への変換が成功すること");

    for x in 0..width {
        let u = uv[x / 2 * 2];
        let v = uv[x / 2 * 2 + 1];
        let argb = helpers::yuv601_to_argb_pixel(y[x], u, v);
        // RAW はメモリ順 R,G,B (ARGB の R,G,B を取り出す)
        let expected = [argb[2], argb[1], argb[0]];
        let actual = [raw[x * 3], raw[x * 3 + 1], raw[x * 3 + 2]];
        assert_eq!(
            actual, expected,
            "x={} の画素が期待値と一致すること: 実測 {:?} 期待 {:?}",
            x, actual, expected
        );
    }
}

// 異常系: nv12_to_raw の src の Y バッファ不足で Err が返ること
#[test]
fn nv12_to_raw_src_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height - 1];
    let uv = vec![0u8; width * 2];
    let src = Nv12Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let mut raw = vec![0u8; width * height * 3];
    let mut dst = RawImageMut {
        data: &mut raw,
        stride: width * 3,
    };
    let result = nv12_to_raw(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: nv12_to_raw の dst の stride 不足で Err が返ること
#[test]
fn nv12_to_raw_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let uv = vec![0u8; width * 2];
    let src = Nv12Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let mut raw = vec![0u8; width * height * 3];
    let mut dst = RawImageMut {
        data: &mut raw,
        stride: width * 3 - 1,
    };
    let result = nv12_to_raw(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: nv12_to_raw の dst の stride の c_int 超過で Err が返ること
#[test]
fn nv12_to_raw_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let uv = vec![0u8; width * 2];
    let src = Nv12Image {
        y: &y,
        y_stride: width,
        uv: &uv,
        uv_stride: width,
    };
    let mut raw = vec![0u8; width * height * 3];
    let mut dst = RawImageMut {
        data: &mut raw,
        stride: c_int::MAX as usize + 1,
    };
    let result = nv12_to_raw(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// nv21_to_yuv24 (NV21 → YUV24)
// ============================================================

// 正常系: nv21_to_yuv24 が既知 NV21 から期待 YUV24 を出力すること
#[test]
fn nv21_to_yuv24_known_data() {
    let width = 4;
    let height = 1;
    let y = vec![10u8, 20, 30, 40];
    let vu = vec![100u8, 101, 200, 201]; // V,U インターリーブ
    let src = Nv21Image {
        y: &y,
        y_stride: width,
        uv: &vu,
        uv_stride: 4,
    };
    let mut yuv24 = vec![0u8; width * height * 3];
    let mut dst = Yuv24ImageMut {
        data: &mut yuv24,
        stride: width * 3,
    };
    let size = ImageSize::new(width, height);

    nv21_to_yuv24(&src, &mut dst, size).expect("NV21 から YUV24 への変換が成功すること");

    // 2 ピクセルで VU 1 ペアを使い回す
    for i in 0..(width / 2) {
        let vu_pair = [vu[i * 2], vu[i * 2 + 1]];
        let expected = helpers::nv21_to_yuv24_pixel(y[i * 2], y[i * 2 + 1], vu_pair);
        let actual = [
            yuv24[i * 6],
            yuv24[i * 6 + 1],
            yuv24[i * 6 + 2],
            yuv24[i * 6 + 3],
            yuv24[i * 6 + 4],
            yuv24[i * 6 + 5],
        ];
        assert_eq!(
            actual, expected,
            "ペア {} が期待値と一致すること: 実測 {:?} 期待 {:?}",
            i, actual, expected
        );
    }
}

// 異常系: nv21_to_yuv24 の src の Y バッファ不足で Err が返ること
#[test]
fn nv21_to_yuv24_src_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height - 1];
    let vu = vec![0u8; width * 2];
    let src = Nv21Image {
        y: &y,
        y_stride: width,
        uv: &vu,
        uv_stride: width,
    };
    let mut yuv24 = vec![0u8; width * height * 3];
    let mut dst = Yuv24ImageMut {
        data: &mut yuv24,
        stride: width * 3,
    };
    let result = nv21_to_yuv24(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source Y buffer too small"),
        "Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: nv21_to_yuv24 の dst の stride 不足で Err が返ること
#[test]
fn nv21_to_yuv24_dst_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let vu = vec![0u8; width * 2];
    let src = Nv21Image {
        y: &y,
        y_stride: width,
        uv: &vu,
        uv_stride: width,
    };
    let mut yuv24 = vec![0u8; width * height * 3];
    let mut dst = Yuv24ImageMut {
        data: &mut yuv24,
        stride: width * 3 - 1,
    };
    let result = nv21_to_yuv24(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: nv21_to_yuv24 の dst の stride の c_int 超過で Err が返ること
#[test]
fn nv21_to_yuv24_dst_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let vu = vec![0u8; width * 2];
    let src = Nv21Image {
        y: &y,
        y_stride: width,
        uv: &vu,
        uv_stride: width,
    };
    let mut yuv24 = vec![0u8; width * height * 3];
    let mut dst = Yuv24ImageMut {
        data: &mut yuv24,
        stride: c_int::MAX as usize + 1,
    };
    let result = nv21_to_yuv24(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("stride exceeds c_int range"),
        "dst 側の c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// i444_to_nv12 (4:4:4 → 4:2:0 UV パック)
// ============================================================

// 正常系: i444_to_nv12 が既知 I444 から期待 NV12 を出力すること
#[test]
fn i444_to_nv12_known_data() {
    let width = 4;
    let height = 2;
    let y: Vec<u8> = (0..(width * height)).map(|i| i as u8).collect();
    let u: Vec<u8> = (0..(width * height)).map(|i| (i * 2) as u8).collect();
    let v: Vec<u8> = (0..(width * height)).map(|i| (i * 3) as u8).collect();
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width,
        v: &v,
        v_stride: width,
    };
    let mut y2 = vec![0u8; width * height];
    let mut uv2 = vec![0u8; width];
    let mut dst = Nv12ImageMut {
        y: &mut y2,
        y_stride: width,
        uv: &mut uv2,
        uv_stride: width,
    };
    let size = ImageSize::new(width, height);

    i444_to_nv12(&src, &mut dst, size).expect("I444 から NV12 への変換が成功すること");

    // Y はコピー
    assert_eq!(y2, y, "Y プレーンがコピーされること");
    // UV は 2x2 ブロックの 4 点平均 (HalfMergeUVRow_C)
    for i in 0..(width / 2) {
        let u_block = [
            u[i * 2],
            u[i * 2 + 1],
            u[width + i * 2],
            u[width + i * 2 + 1],
        ];
        let v_block = [
            v[i * 2],
            v[i * 2 + 1],
            v[width + i * 2],
            v[width + i * 2 + 1],
        ];
        let expected = helpers::i444_uv_merge(u_block, v_block);
        let actual = [uv2[i * 2], uv2[i * 2 + 1]];
        assert_eq!(
            actual, expected,
            "UV[{}] が期待値と一致すること: 実測 {:?} 期待 {:?}",
            i, actual, expected
        );
    }
}

// 異常系: i444_to_nv12 の dst の UV バッファ不足で Err が返ること
#[test]
fn i444_to_nv12_dst_uv_buffer_too_small() {
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
    let mut y2 = vec![0u8; width * height];
    let mut uv2 = vec![0u8; width * 2 - 1];
    let mut dst = Nv12ImageMut {
        y: &mut y2,
        y_stride: width,
        uv: &mut uv2,
        uv_stride: width,
    };
    let result = i444_to_nv12(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst の UV バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination UV buffer too small"),
        "dst の UV 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i444_to_nv12 の src の U stride 不足で Err が返ること
#[test]
fn i444_to_nv12_src_u_stride_too_small() {
    let width = 8;
    let height = 4;
    let y = vec![0u8; width * height];
    let u = vec![0u8; width * height];
    let v = vec![0u8; width * height];
    let src = I444Image {
        y: &y,
        y_stride: width,
        u: &u,
        u_stride: width - 1,
        v: &v,
        v_stride: width,
    };
    let mut y2 = vec![0u8; width * height];
    let mut uv2 = vec![0u8; width * 2];
    let mut dst = Nv12ImageMut {
        y: &mut y2,
        y_stride: width,
        uv: &mut uv2,
        uv_stride: width,
    };
    let result = i444_to_nv12(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("U stride 不足では Err が返るべき");
    assert!(
        err.to_string()
            .contains("U stride smaller than chroma width"),
        "U 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: i444_to_nv12 の src の V stride の c_int 超過で Err が返ること
#[test]
fn i444_to_nv12_src_v_stride_exceeds_c_int() {
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
        v_stride: c_int::MAX as usize + 1,
    };
    let mut y2 = vec![0u8; width * height];
    let mut uv2 = vec![0u8; width * 2];
    let mut dst = Nv12ImageMut {
        y: &mut y2,
        y_stride: width,
        uv: &mut uv2,
        uv_stride: width,
    };
    let result = i444_to_nv12(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("V stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
