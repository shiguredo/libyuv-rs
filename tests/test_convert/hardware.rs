//! hardware サブモジュールの代表関数の単体テスト
//!
//! `src/convert/hardware.rs` に対応する。代表関数として
//! `ayuv_to_nv12` / `ayuv_to_nv21` (AYUV→NV 系) を選定し、
//! ground truth 方式の正常系とエラーパス (バッファ不足・stride 不足・c_int 超過) を検証する。
//! android420 / mm21 / mt2t / detile 系は 0045 / 0046 / 0047 のスコープのため選定しない。

use std::ffi::c_int;

use shiguredo_libyuv::{
    AyuvImage, ImageSize, Nv12ImageMut, Nv21ImageMut, ayuv_to_nv12, ayuv_to_nv21,
};

use super::helpers;

// AYUV はメモリ順 V,U,Y,A の 4 バイト/ピクセル。テスト用の AYUV データを生成する
fn build_ayuv(width: usize, height: usize) -> Vec<u8> {
    let mut ayuv = vec![0u8; width * height * 4];
    for i in 0..(width * height) {
        ayuv[i * 4] = (i * 7 % 256) as u8; // V
        ayuv[i * 4 + 1] = (i * 5 % 256) as u8; // U
        ayuv[i * 4 + 2] = (i * 3 % 256) as u8; // Y
        ayuv[i * 4 + 3] = 255; // A
    }
    ayuv
}

// ============================================================
// ayuv_to_nv12
// ============================================================

// 正常系: ayuv_to_nv12 が既知 AYUV から期待 NV12 を出力すること
#[test]
fn ayuv_to_nv12_known_data() {
    let width = 4;
    let height = 2;
    let ayuv = build_ayuv(width, height);
    let src = AyuvImage {
        data: &ayuv,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut uv = vec![0u8; width];
    let mut dst = Nv12ImageMut {
        y: &mut y,
        y_stride: width,
        uv: &mut uv,
        uv_stride: width,
    };
    let size = ImageSize::new(width, height);

    ayuv_to_nv12(&src, &mut dst, size).expect("AYUV から NV12 への変換が成功すること");

    // Y は AYUV の 3 バイト目 (V,U,Y,A の Y)
    for i in 0..(width * height) {
        assert_eq!(
            y[i],
            ayuv[i * 4 + 2],
            "Y[{}] が AYUV の Y 成分と一致すること",
            i
        );
    }
    // UV は 2x2 ブロックの平均 (U はバイト 1 / 5 / stride+1 / stride+5、V は 0 / 4 / stride+0 / stride+4)
    for i in 0..(width / 2) {
        let u_block = [
            ayuv[(i * 2) * 4 + 1],
            ayuv[(i * 2 + 1) * 4 + 1],
            ayuv[(width + i * 2) * 4 + 1],
            ayuv[(width + i * 2 + 1) * 4 + 1],
        ];
        let v_block = [
            ayuv[(i * 2) * 4],
            ayuv[(i * 2 + 1) * 4],
            ayuv[(width + i * 2) * 4],
            ayuv[(width + i * 2 + 1) * 4],
        ];
        let expected = helpers::ayuv_uv_merge(u_block, v_block);
        let actual = [uv[i * 2], uv[i * 2 + 1]];
        assert_eq!(
            actual, expected,
            "UV[{}] が期待値と一致すること: 実測 {:?} 期待 {:?}",
            i, actual, expected
        );
    }
}

// 異常系: ayuv_to_nv12 の src のバッファ不足で Err が返ること
#[test]
fn ayuv_to_nv12_src_buffer_too_small() {
    let width = 8;
    let height = 4;
    let ayuv = vec![0u8; width * height * 4 - 1];
    let src = AyuvImage {
        data: &ayuv,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut uv = vec![0u8; width * 2];
    let mut dst = Nv12ImageMut {
        y: &mut y,
        y_stride: width,
        uv: &mut uv,
        uv_stride: width,
    };
    let result = ayuv_to_nv12(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("src バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source buffer too small"),
        "src 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: ayuv_to_nv12 の dst の Y バッファ不足で Err が返ること
#[test]
fn ayuv_to_nv12_dst_y_buffer_too_small() {
    let width = 8;
    let height = 4;
    let ayuv = vec![0u8; width * height * 4];
    let src = AyuvImage {
        data: &ayuv,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height - 1];
    let mut uv = vec![0u8; width * 2];
    let mut dst = Nv12ImageMut {
        y: &mut y,
        y_stride: width,
        uv: &mut uv,
        uv_stride: width,
    };
    let result = ayuv_to_nv12(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst の Y バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination Y buffer too small"),
        "dst の Y 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: ayuv_to_nv12 の dst の UV stride の c_int 超過で Err が返ること
#[test]
fn ayuv_to_nv12_dst_uv_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let ayuv = vec![0u8; width * height * 4];
    let src = AyuvImage {
        data: &ayuv,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut uv = vec![0u8; width * 2];
    let mut dst = Nv12ImageMut {
        y: &mut y,
        y_stride: width,
        uv: &mut uv,
        uv_stride: c_int::MAX as usize + 1,
    };
    let result = ayuv_to_nv12(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("UV stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}

// ============================================================
// ayuv_to_nv21
// ============================================================

// 正常系: ayuv_to_nv21 が既知 AYUV から期待 NV21 を出力すること
#[test]
fn ayuv_to_nv21_known_data() {
    let width = 4;
    let height = 2;
    let ayuv = build_ayuv(width, height);
    let src = AyuvImage {
        data: &ayuv,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut vu = vec![0u8; width];
    let mut dst = Nv21ImageMut {
        y: &mut y,
        y_stride: width,
        uv: &mut vu,
        uv_stride: width,
    };
    let size = ImageSize::new(width, height);

    ayuv_to_nv21(&src, &mut dst, size).expect("AYUV から NV21 への変換が成功すること");

    // Y は AYUV の 3 バイト目
    for i in 0..(width * height) {
        assert_eq!(
            y[i],
            ayuv[i * 4 + 2],
            "Y[{}] が AYUV の Y 成分と一致すること",
            i
        );
    }
    // NV21 は VU 順 (AYUVToVURow_C)。V はバイト 0 / 4 / stride+0 / stride+4、U は 1 / 5 / stride+1 / stride+5
    for i in 0..(width / 2) {
        let v_block = [
            ayuv[(i * 2) * 4],
            ayuv[(i * 2 + 1) * 4],
            ayuv[(width + i * 2) * 4],
            ayuv[(width + i * 2 + 1) * 4],
        ];
        let u_block = [
            ayuv[(i * 2) * 4 + 1],
            ayuv[(i * 2 + 1) * 4 + 1],
            ayuv[(width + i * 2) * 4 + 1],
            ayuv[(width + i * 2 + 1) * 4 + 1],
        ];
        let v_avg =
            ((v_block[0] as u32 + v_block[1] as u32 + v_block[2] as u32 + v_block[3] as u32 + 2)
                >> 2) as u8;
        let u_avg =
            ((u_block[0] as u32 + u_block[1] as u32 + u_block[2] as u32 + u_block[3] as u32 + 2)
                >> 2) as u8;
        let actual = [vu[i * 2], vu[i * 2 + 1]];
        assert_eq!(
            actual,
            [v_avg, u_avg],
            "VU[{}] が期待値と一致すること: 実測 {:?} 期待 {:?}",
            i,
            actual,
            [v_avg, u_avg]
        );
    }
}

// 異常系: ayuv_to_nv21 の src の stride 不足で Err が返ること
#[test]
fn ayuv_to_nv21_src_stride_too_small() {
    let width = 8;
    let height = 4;
    let ayuv = vec![0u8; width * height * 4];
    let src = AyuvImage {
        data: &ayuv,
        stride: width * 4 - 1,
    };
    let mut y = vec![0u8; width * height];
    let mut vu = vec![0u8; width * 2];
    let mut dst = Nv21ImageMut {
        y: &mut y,
        y_stride: width,
        uv: &mut vu,
        uv_stride: width,
    };
    let result = ayuv_to_nv21(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("src stride 不足では Err が返るべき");
    assert!(
        err.to_string().contains("stride smaller than width * bpp"),
        "src 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: ayuv_to_nv21 の dst の UV バッファ不足で Err が返ること
#[test]
fn ayuv_to_nv21_dst_uv_buffer_too_small() {
    let width = 8;
    let height = 4;
    let ayuv = vec![0u8; width * height * 4];
    let src = AyuvImage {
        data: &ayuv,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut vu = vec![0u8; width * 2 - 1];
    let mut dst = Nv21ImageMut {
        y: &mut y,
        y_stride: width,
        uv: &mut vu,
        uv_stride: width,
    };
    let result = ayuv_to_nv21(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("dst の UV バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("destination UV buffer too small"),
        "dst の UV 側のバッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: ayuv_to_nv21 の dst の Y stride の c_int 超過で Err が返ること
#[test]
fn ayuv_to_nv21_dst_y_stride_exceeds_c_int() {
    let width = 8;
    let height = 4;
    let ayuv = vec![0u8; width * height * 4];
    let src = AyuvImage {
        data: &ayuv,
        stride: width * 4,
    };
    let mut y = vec![0u8; width * height];
    let mut vu = vec![0u8; width * 2];
    let mut dst = Nv21ImageMut {
        y: &mut y,
        y_stride: c_int::MAX as usize + 1,
        uv: &mut vu,
        uv_stride: width,
    };
    let result = ayuv_to_nv21(&src, &mut dst, ImageSize::new(width, height));
    let err = result.expect_err("c_int 範囲を超える stride では Err が返るべき");
    assert!(
        err.to_string().contains("Y stride exceeds c_int range"),
        "c_int 範囲超過の reason が返るべき: {}",
        err
    );
}
