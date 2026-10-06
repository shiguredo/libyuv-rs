//! プレーン操作 API の単体テスト
//!
//! エラーパス・境界値と、既知解による正常系（stride の行配置）を検証する。

use std::ffi::c_int;

use shiguredo_libyuv::{
    ArgbImage, ArgbImageMut, Error, ImageSize, argb_blur, convert_to_lsb_plane_16,
    convert_to_msb_plane_16, copy_plane, half_float_plane, merge_ar64_plane,
    merge_argb16_to_8_plane, merge_uv_plane_16, merge_xr30_plane, split_uv_plane,
    split_uv_plane_16,
};

// scale = 1.0 で 2 の冪 2^k（k は 0..=15）の入力を変換したときの f16 出力のビットパターンを返す。
// 2 の冪は仮数が 0 のため、truncation（C / NEON / AVX2）でも RNE 丸め（F16C / SVE2）でも
// 丸めが発生せず、f16 の指数バイアス 15 の規則で (k + 15) << 10 になる。
// k > 15 では指数が 30 を超え f16 の有限値の範囲から外れるため、このヘルパーは使わないこと。
// 0 の入力は全バックエンドで 0x0000 に変換される
fn f16_bits_of_pow2(k: u16) -> u16 {
    (k + 15) << 10
}

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

// 正常系: half_float_plane が stride == width で正しい出力を返すこと
#[test]
fn half_float_plane_stride_equal_width() {
    // scale = 1.0 では 2 の冪と 0 の入力は全バックエンドで変換後も厳密に一致する。
    // 2^16 は u16 に収まらないため、16 番目の周期は境界値の 0 を配置して 17 周期にする
    let width = 8;
    let height = 4;
    let src: Vec<u16> = (0..(width * height))
        .map(|i| match i % 17 {
            16 => 0,
            k => 1u16 << k,
        })
        .collect();
    let expected: Vec<u16> = (0..(width * height))
        .map(|i| match i % 17 {
            16 => 0,
            k => f16_bits_of_pow2(k as u16),
        })
        .collect();
    let mut dst = vec![0u16; width * height];
    let size = ImageSize::new(width, height);

    half_float_plane(&src, width, &mut dst, width, 1.0, size)
        .expect("stride == width の変換が成功すること");

    assert_eq!(
        dst, expected,
        "2 の冪の入力は f16 ビットパターンに厳密に一致するべき"
    );
}

// 正常系: half_float_plane が stride > width（パディング付き）で行配置を正しく行うこと
#[test]
fn half_float_plane_padded_stride_row_placement() {
    // stride > width では C 側の行合体（coalesce）が効かないため、行ごとの stride 送りが検証される
    let width = 8;
    let height = 4;
    let src_stride = 12; // パディング 4 要素
    let dst_stride = 12;
    let mut src = vec![0xAAAAu16; src_stride * height]; // 読み過ぎ検出用の別値
    for r in 0..height {
        for i in 0..width {
            src[r * src_stride + i] = 1u16 << r;
        }
    }
    let mut dst = vec![0xFFFFu16; dst_stride * height]; // 未書き込み領域の検出用
    let size = ImageSize::new(width, height);

    half_float_plane(&src, src_stride, &mut dst, dst_stride, 1.0, size)
        .expect("パディング付きの変換が成功すること");

    for r in 0..height {
        let expected = f16_bits_of_pow2(r as u16);
        for i in 0..width {
            assert_eq!(
                dst[r * dst_stride + i],
                expected,
                "行 {} の要素 {} が正しく配置されること",
                r,
                i
            );
        }
        // libyuv は 1 行あたり width 要素だけ書き込むため、パディング領域は書き換わらない
        for i in width..dst_stride {
            assert_eq!(
                dst[r * dst_stride + i],
                0xFFFF,
                "パディング領域は書き換わらないこと"
            );
        }
    }
}

// 異常系: half_float_plane の stride × 2（バイト単位変換後）が c_int の範囲を超えると Err が返ること
#[test]
fn half_float_plane_stride_bytes_exceeds_c_int() {
    // 要素数単位の stride は c_int の範囲内でも、バイト単位に変換すると範囲を超える。
    // 変換後の検証はバッファサイズ検証より前にあるため、巨大なバッファは必要ない。
    // 相手側の stride は width == 1 の最小有効値 1 にする（0 だと stride < width チェックが
    // 先に発火するため、対象の変換後 stride 検証に到達しない）
    let src = vec![0u16; 1];
    let mut dst = vec![0u16; 1];
    let size = ImageSize::new(1, 1);
    let max_stride = c_int::MAX as usize;

    let result = half_float_plane(&src, max_stride, &mut dst, 1, 1.0, size);
    let err = result.expect_err("src 側の stride × 2 超過では Err が返るべき");
    assert!(
        err.to_string()
            .contains("source stride (bytes) exceeds c_int range"),
        "src 側の変換後 stride 超過の reason が返るべき: {}",
        err
    );

    let result = half_float_plane(&src, 1, &mut dst, max_stride, 1.0, size);
    let err = result.expect_err("dst 側の stride × 2 超過では Err が返るべき");
    assert!(
        err.to_string()
            .contains("destination stride (bytes) exceeds c_int range"),
        "dst 側の変換後 stride 超過の reason が返るべき: {}",
        err
    );
}

// 異常系: half_float_plane の src_stride / dst_stride が width 未満で Err が返ること
#[test]
fn half_float_plane_stride_too_small() {
    let size = ImageSize::new(8, 1);

    let src = vec![0u16; 8];
    let mut dst = vec![0u16; 16];
    let result = half_float_plane(&src, 4, &mut dst, 8, 1.0, size);
    let err = result.expect_err("src_stride < width では Err が返るべき");
    assert!(
        err.to_string().contains("source stride smaller than width"),
        "src 側の stride 不足の reason が返るべき: {}",
        err
    );

    let src = vec![0u16; 16];
    let mut dst = vec![0u16; 8];
    let result = half_float_plane(&src, 8, &mut dst, 4, 1.0, size);
    let err = result.expect_err("dst_stride < width では Err が返るべき");
    assert!(
        err.to_string()
            .contains("destination stride smaller than width"),
        "dst 側の stride 不足の reason が返るべき: {}",
        err
    );
}

// 異常系: half_float_plane のバッファ不足で Err が返ること
#[test]
fn half_float_plane_buffer_too_small() {
    // stride は正しいが、1 行分の要素数が足りない場合にバッファサイズ検証で Err になること
    let src = vec![0u16; 7];
    let mut dst = vec![0u16; 8];
    let size = ImageSize::new(8, 1);

    let result = half_float_plane(&src, 8, &mut dst, 8, 1.0, size);
    let err = result.expect_err("src バッファ不足では Err が返るべき");
    assert!(
        err.to_string().contains("source buffer too small"),
        "src バッファ不足の reason が返るべき: {}",
        err
    );
}

// ============================================================
// argb_blur
// ============================================================

// argb_blur テスト用のヘルパー。src / dst は width * height * 4 バイト、cumsum は
// cumsum_rows 行分（stride32_cumsum = width * 4）を確保して radius で呼び出す。
// stride32_cumsum を width * 4 以外にするテストは直接 argb_blur を呼ぶこと
fn call_argb_blur(size: ImageSize, cumsum_rows: usize, radius: i32) -> Result<(), Error> {
    let src = vec![0u8; size.width * size.height * 4];
    let mut dst = vec![0u8; size.width * size.height * 4];
    let mut cumsum = vec![0i32; size.width * 4 * cumsum_rows];
    argb_blur(
        &ArgbImage {
            data: &src,
            stride: size.width * 4,
        },
        &mut ArgbImageMut {
            data: &mut dst,
            stride: size.width * 4,
        },
        size,
        &mut cumsum,
        size.width * 4,
        radius,
    )
}

// 正常系: argb_blur が C の循環バッファ契約どおり min(height, 有効 radius * 2 + 2) 行の
// cumsum バッファで成功すること（radius が小さいときは height 行より少ない行で足りる）
#[test]
fn argb_blur_cumsum_rows_less_than_height() {
    // width=10, height=10, radius=1: 有効 radius = min(1, 10, 10 / 2 - 1 = 4) = 1
    // 必要行数 = min(10, 1 * 2 + 2) = 4 行（height 行ではない）
    call_argb_blur(ImageSize::new(10, 10), 4, 1)
        .expect("必要行数ちょうどの cumsum では Ok が返るべき");
    // 従来の契約（height 行確保）のままでも成功すること（回帰防止）
    call_argb_blur(ImageSize::new(10, 10), 10, 1).expect("height 行の cumsum でも Ok が返るべき");
}

// 異常系: argb_blur が cumsum の必要行数から 1 行不足で Err を返すこと
#[test]
fn argb_blur_cumsum_one_row_short() {
    // 必要行数 4 行に対し 3 行しか用意しない（1 行不足で Err になること）
    let err =
        call_argb_blur(ImageSize::new(10, 10), 3, 1).expect_err("1 行不足では Err が返るべき");
    assert!(
        err.to_string().contains("cumsum buffer too small"),
        "cumsum バッファ不足の reason が返るべき: {}",
        err
    );
}

// 正常系: argb_blur の cumsum 必要行数が width / 2 - 1 >= height では height 行のまま
// になること（従来と同じ要求）
#[test]
fn argb_blur_cumsum_rows_equals_height_when_radius_equals_height() {
    // width=20, height=8, radius=8: 有効 radius = min(8, 8, 20 / 2 - 1 = 9) = 8
    // 必要行数 = min(8, 8 * 2 + 2 = 18) = 8 行 = height 行
    call_argb_blur(ImageSize::new(20, 8), 8, 8)
        .expect("radius == height では height 行の cumsum で Ok が返るべき");
    let err = call_argb_blur(ImageSize::new(20, 8), 7, 8)
        .expect_err("height 行から 1 行不足では Err が返るべき");
    assert!(
        err.to_string().contains("cumsum buffer too small"),
        "cumsum バッファ不足の reason が返るべき: {}",
        err
    );
}

// 正常系: argb_blur の radius > height が clamp 後に radius == height と同じ必要行数
// になること。1 行不足で Err になること
// なお必要行数は半径が height を超えると常に height 行になるため、このテストは
// radius == height と同じ Ok / Err 境界を検証するものであり、clamp の有無は必要行数に
// 影響しない（C 側も同じ規則で clamp するため、C に渡る radius が clamp 前の値でも安全）
#[test]
fn argb_blur_radius_greater_than_height() {
    // radius=100 > height=8: 有効 radius = min(100, 8, 9) = 8 → 必要行数 8 行
    call_argb_blur(ImageSize::new(20, 8), 8, 100)
        .expect("radius > height では height 行の cumsum で Ok が返るべき");
    let err = call_argb_blur(ImageSize::new(20, 8), 7, 100)
        .expect_err("height 行から 1 行不足では Err が返るべき");
    assert!(
        err.to_string().contains("cumsum buffer too small"),
        "cumsum バッファ不足の reason が返るべき: {}",
        err
    );
}

// 正常系: argb_blur の radius が width / 2 - 1 で clamp される場合の必要行数で成功
// すること。1 行不足で Err になること
#[test]
fn argb_blur_radius_clamped_to_width() {
    // width=10, height=20, radius=10: 有効 radius = min(10, 20, 10 / 2 - 1 = 4) = 4
    // 必要行数 = min(20, 4 * 2 + 2 = 10) = 10 行（width clamp が効くケース。
    // clamp が機能しないと必要行数が min(20, 10 * 2 + 2 = 22) = 20 行になるため、
    // 10 行で Ok / 9 行で Err が clamp の正しさを検証する）
    call_argb_blur(ImageSize::new(10, 20), 10, 10)
        .expect("width clamp 後の必要行数の cumsum で Ok が返るべき");
    let err = call_argb_blur(ImageSize::new(10, 20), 9, 10)
        .expect_err("width clamp 後の必要行数から 1 行不足では Err が返るべき");
    assert!(
        err.to_string().contains("cumsum buffer too small"),
        "cumsum バッファ不足の reason が返るべき: {}",
        err
    );
}

// 正常系: argb_blur が height == 有効 radius * 2 + 2 の等号境界で必要行数どおりに
// 成功すること。1 行不足で Err になること
#[test]
fn argb_blur_cumsum_rows_equals_radius_times_2_plus_2() {
    // width=10, height=4, radius=1: 有効 radius = min(1, 4, 4) = 1
    // 必要行数 = min(4, 1 * 2 + 2 = 4) = 4 行 = height（等号境界）
    call_argb_blur(ImageSize::new(10, 4), 4, 1)
        .expect("height == 有効 radius * 2 + 2 では height 行の cumsum で Ok が返るべき");
    let err = call_argb_blur(ImageSize::new(10, 4), 3, 1)
        .expect_err("等号境界から 1 行不足では Err が返るべき");
    assert!(
        err.to_string().contains("cumsum buffer too small"),
        "cumsum バッファ不足の reason が返るべき: {}",
        err
    );
}

// 正常系: argb_blur が最小の有効 height（height=2）で成功すること
#[test]
fn argb_blur_minimum_height() {
    // width=10, height=2, radius=1: 有効 radius = min(1, 2, 4) = 1
    // 必要行数 = min(2, 4) = 2 行（height <= 1 の Err との境界）
    call_argb_blur(ImageSize::new(10, 2), 2, 1)
        .expect("height=2 では 2 行の cumsum で Ok が返るべき");
}

// 異常系: argb_blur が空の cumsum バッファで Err を返すこと
#[test]
fn argb_blur_cumsum_empty_buffer() {
    let err =
        call_argb_blur(ImageSize::new(10, 10), 0, 1).expect_err("空の cumsum では Err が返るべき");
    assert!(
        err.to_string().contains("cumsum buffer too small"),
        "cumsum バッファ不足の reason が返るべき: {}",
        err
    );
}

// 異常系: argb_blur の radius <= 0 で Err が返ること
#[test]
fn argb_blur_radius_non_positive() {
    // radius=0 と負値は C の clamp 後に radius <= 0 の -1 返却条件と一致し Err になる
    for radius in [0, -1] {
        let err = call_argb_blur(ImageSize::new(8, 8), 8, radius)
            .expect_err("radius <= 0 では Err が返るべき");
        assert!(
            err.to_string().contains("radius must be greater than 0"),
            "radius 検証の reason が返るべき: {}",
            err
        );
    }
}

// 異常系: argb_blur の height <= 1 で Err が返ること
#[test]
fn argb_blur_height_too_small() {
    // height=1 と height=0 は C の -1 返却条件と一致し Err になる
    for height in [1, 0] {
        let err = call_argb_blur(ImageSize::new(8, height), 8, 1)
            .expect_err("height <= 1 では Err が返るべき");
        assert!(
            err.to_string().contains("height must be greater than 1"),
            "height 検証の reason が返るべき: {}",
            err
        );
    }
}

// 異常系: argb_blur の width <= 3 で Err が返ること
#[test]
fn argb_blur_width_too_small() {
    // width <= 3 では C の clamp 後も有効 radius が 0 以下になり Err になる。
    // width=0 も同様に Err になる
    for width in [3, 2, 1, 0] {
        let err = call_argb_blur(ImageSize::new(width, 8), 8, 1)
            .expect_err("width <= 3 では Err が返るべき");
        assert!(
            err.to_string().contains("width must be greater than 3"),
            "width 検証の reason が返るべき: {}",
            err
        );
    }
}

// 異常系: argb_blur の stride32_cumsum が width * 4 未満で Err が返ること
#[test]
fn argb_blur_cumsum_stride_too_small() {
    // stride32_cumsum を 1 要素不足させて、既存の stride 下限チェックが効くことを確認する
    let size = ImageSize::new(10, 10);
    let src = vec![0u8; 10 * 10 * 4];
    let mut dst = vec![0u8; 10 * 10 * 4];
    let mut cumsum = vec![0i32; 10 * 4 * 10];

    let result = argb_blur(
        &ArgbImage {
            data: &src,
            stride: 10 * 4,
        },
        &mut ArgbImageMut {
            data: &mut dst,
            stride: 10 * 4,
        },
        size,
        &mut cumsum,
        10 * 4 - 1, // stride32_cumsum が width * 4 より 1 要素不足
        1,
    );
    let err = result.expect_err("cumsum stride 不足では Err が返るべき");
    assert!(
        err.to_string()
            .contains("cumsum stride smaller than width * 4"),
        "cumsum stride 不足の reason が返るべき: {}",
        err
    );
}

// ============================================================
// 16bit 変換関数の depth 検証
// ============================================================

// merge_uv_plane_16 を正しいバッファ構成で呼び出すヘルパー。
// バッファサイズは size から導出する (u/v は width * height、uv は width * 2 * height)
fn call_merge_uv_plane_16(size: ImageSize, depth: i32) -> Result<(), Error> {
    let src_u = vec![0u16; size.width * size.height];
    let src_v = vec![0u16; size.width * size.height];
    let mut dst_uv = vec![0u16; size.width * size.height * 2];
    merge_uv_plane_16(
        &src_u,
        size.width,
        &src_v,
        size.width,
        &mut dst_uv,
        size.width * 2,
        size,
        depth,
    )
}

// split_uv_plane_16 を正しいバッファ構成で呼び出すヘルパー
fn call_split_uv_plane_16(size: ImageSize, depth: i32) -> Result<(), Error> {
    let src_uv = vec![0u16; size.width * size.height * 2];
    let mut dst_u = vec![0u16; size.width * size.height];
    let mut dst_v = vec![0u16; size.width * size.height];
    split_uv_plane_16(
        &src_uv,
        size.width * 2,
        &mut dst_u,
        size.width,
        &mut dst_v,
        size.width,
        size,
        depth,
    )
}

// merge_ar64_plane を正しいバッファ構成で呼び出すヘルパー
fn call_merge_ar64_plane(size: ImageSize, depth: i32) -> Result<(), Error> {
    let src_r = vec![0u16; size.width * size.height];
    let src_g = vec![0u16; size.width * size.height];
    let src_b = vec![0u16; size.width * size.height];
    let src_a = vec![0u16; size.width * size.height];
    let mut dst_ar64 = vec![0u16; size.width * size.height * 4];
    merge_ar64_plane(
        &src_r,
        size.width,
        &src_g,
        size.width,
        &src_b,
        size.width,
        &src_a,
        size.width,
        &mut dst_ar64,
        size.width * 4,
        size,
        depth,
    )
}

// merge_xr30_plane を正しいバッファ構成で呼び出すヘルパー
fn call_merge_xr30_plane(size: ImageSize, depth: i32) -> Result<(), Error> {
    let src_r = vec![0u16; size.width * size.height];
    let src_g = vec![0u16; size.width * size.height];
    let src_b = vec![0u16; size.width * size.height];
    let mut dst_ar30 = vec![0u8; size.width * size.height * 4];
    merge_xr30_plane(
        &src_r,
        size.width,
        &src_g,
        size.width,
        &src_b,
        size.width,
        &mut dst_ar30,
        size.width * 4,
        size,
        depth,
    )
}

// merge_argb16_to_8_plane を正しいバッファ構成で呼び出すヘルパー
fn call_merge_argb16_to_8_plane(size: ImageSize, depth: i32) -> Result<(), Error> {
    let src_r = vec![0u16; size.width * size.height];
    let src_g = vec![0u16; size.width * size.height];
    let src_b = vec![0u16; size.width * size.height];
    let src_a = vec![0u16; size.width * size.height];
    let mut dst_argb = vec![0u8; size.width * size.height * 4];
    merge_argb16_to_8_plane(
        &src_r,
        size.width,
        &src_g,
        size.width,
        &src_b,
        size.width,
        &src_a,
        size.width,
        &mut dst_argb,
        size.width * 4,
        size,
        depth,
    )
}

// convert_to_lsb_plane_16 / convert_to_msb_plane_16 を正しいバッファ構成で呼び出す
// ヘルパー (変換関数を引数で切り替える)
fn call_convert_plane_16(lsb: bool, size: ImageSize, depth: i32) -> Result<(), Error> {
    let src = vec![0u16; size.width * size.height];
    let mut dst = vec![0u16; size.width * size.height];
    if lsb {
        convert_to_lsb_plane_16(&src, size.width, &mut dst, size.width, size, depth)
    } else {
        convert_to_msb_plane_16(&src, size.width, &mut dst, size.width, size, depth)
    }
}

// 正常系: merge_uv_plane_16 の depth が有効範囲 (8..=16) の境界値で成功すること
#[test]
fn merge_uv_plane_16_depth_valid_boundaries() {
    let size = ImageSize::new(4, 4);
    for depth in [8, 16] {
        call_merge_uv_plane_16(size, depth).expect("有効範囲の depth では Ok が返るべき");
    }
}

// 異常系: merge_uv_plane_16 の depth が有効範囲 (8..=16) 外で Err が返ること
#[test]
fn merge_uv_plane_16_depth_out_of_range() {
    let size = ImageSize::new(4, 4);
    for depth in [7, 17, -1, i32::MIN, i32::MAX] {
        let err =
            call_merge_uv_plane_16(size, depth).expect_err("範囲外の depth では Err が返るべき");
        assert!(
            err.to_string().contains("depth must be between 8 and 16"),
            "depth 検証の reason が返るべき: {}",
            err
        );
    }
    // ゼロサイズ入力でも仕様として Err になること
    let err = call_merge_uv_plane_16(ImageSize::new(0, 0), 7)
        .expect_err("ゼロサイズ + 範囲外 depth では Err が返るべき");
    assert!(
        err.to_string().contains("depth must be between 8 and 16"),
        "depth 検証の reason が返るべき: {}",
        err
    );
}

// 正常系: split_uv_plane_16 の depth が有効範囲 (8..=16) の境界値で成功すること
#[test]
fn split_uv_plane_16_depth_valid_boundaries() {
    let size = ImageSize::new(4, 4);
    for depth in [8, 16] {
        call_split_uv_plane_16(size, depth).expect("有効範囲の depth では Ok が返るべき");
    }
}

// 異常系: split_uv_plane_16 の depth が有効範囲 (8..=16) 外で Err が返ること
#[test]
fn split_uv_plane_16_depth_out_of_range() {
    let size = ImageSize::new(4, 4);
    for depth in [7, 17, -1, i32::MIN, i32::MAX] {
        let err =
            call_split_uv_plane_16(size, depth).expect_err("範囲外の depth では Err が返るべき");
        assert!(
            err.to_string().contains("depth must be between 8 and 16"),
            "depth 検証の reason が返るべき: {}",
            err
        );
    }
    // ゼロサイズ入力でも仕様として Err になること
    let err = call_split_uv_plane_16(ImageSize::new(0, 0), 7)
        .expect_err("ゼロサイズ + 範囲外 depth では Err が返るべき");
    assert!(
        err.to_string().contains("depth must be between 8 and 16"),
        "depth 検証の reason が返るべき: {}",
        err
    );
}

// 正常系: merge_ar64_plane の depth が有効範囲 (1..=16) の境界値で成功すること
#[test]
fn merge_ar64_plane_depth_valid_boundaries() {
    let size = ImageSize::new(4, 4);
    for depth in [1, 16] {
        call_merge_ar64_plane(size, depth).expect("有効範囲の depth では Ok が返るべき");
    }
}

// 異常系: merge_ar64_plane の depth が有効範囲 (1..=16) 外で Err が返ること
#[test]
fn merge_ar64_plane_depth_out_of_range() {
    let size = ImageSize::new(4, 4);
    for depth in [0, 17, -1, i32::MIN, i32::MAX] {
        let err =
            call_merge_ar64_plane(size, depth).expect_err("範囲外の depth では Err が返るべき");
        assert!(
            err.to_string().contains("depth must be between 1 and 16"),
            "depth 検証の reason が返るべき: {}",
            err
        );
    }
    // ゼロサイズ入力でも仕様として Err になること
    let err = call_merge_ar64_plane(ImageSize::new(0, 0), 0)
        .expect_err("ゼロサイズ + 範囲外 depth では Err が返るべき");
    assert!(
        err.to_string().contains("depth must be between 1 and 16"),
        "depth 検証の reason が返るべき: {}",
        err
    );
}

// 正常系: merge_xr30_plane の depth が有効範囲 (10..=16) の境界値で成功すること
#[test]
fn merge_xr30_plane_depth_valid_boundaries() {
    let size = ImageSize::new(4, 4);
    for depth in [10, 16] {
        call_merge_xr30_plane(size, depth).expect("有効範囲の depth では Ok が返るべき");
    }
}

// 異常系: merge_xr30_plane の depth が有効範囲 (10..=16) 外で Err が返ること
#[test]
fn merge_xr30_plane_depth_out_of_range() {
    let size = ImageSize::new(4, 4);
    for depth in [9, 17, -1, i32::MIN, i32::MAX] {
        let err =
            call_merge_xr30_plane(size, depth).expect_err("範囲外の depth では Err が返るべき");
        assert!(
            err.to_string().contains("depth must be between 10 and 16"),
            "depth 検証の reason が返るべき: {}",
            err
        );
    }
    // ゼロサイズ入力でも仕様として Err になること
    let err = call_merge_xr30_plane(ImageSize::new(0, 0), 9)
        .expect_err("ゼロサイズ + 範囲外 depth では Err が返るべき");
    assert!(
        err.to_string().contains("depth must be between 10 and 16"),
        "depth 検証の reason が返るべき: {}",
        err
    );
}

// 正常系: merge_argb16_to_8_plane の depth が有効範囲 (8..=16) の境界値で成功すること
#[test]
fn merge_argb16_to_8_plane_depth_valid_boundaries() {
    let size = ImageSize::new(4, 4);
    for depth in [8, 16] {
        call_merge_argb16_to_8_plane(size, depth).expect("有効範囲の depth では Ok が返るべき");
    }
}

// 異常系: merge_argb16_to_8_plane の depth が有効範囲 (8..=16) 外で Err が返ること
#[test]
fn merge_argb16_to_8_plane_depth_out_of_range() {
    let size = ImageSize::new(4, 4);
    for depth in [7, 17, -1, i32::MIN, i32::MAX] {
        let err = call_merge_argb16_to_8_plane(size, depth)
            .expect_err("範囲外の depth では Err が返るべき");
        assert!(
            err.to_string().contains("depth must be between 8 and 16"),
            "depth 検証の reason が返るべき: {}",
            err
        );
    }
    // ゼロサイズ入力でも仕様として Err になること
    let err = call_merge_argb16_to_8_plane(ImageSize::new(0, 0), 7)
        .expect_err("ゼロサイズ + 範囲外 depth では Err が返るべき");
    assert!(
        err.to_string().contains("depth must be between 8 and 16"),
        "depth 検証の reason が返るべき: {}",
        err
    );
}

// 正常系: convert_to_lsb_plane_16 / convert_to_msb_plane_16 の depth が有効範囲
// (8..=16) の境界値で成功すること
#[test]
fn convert_to_lsb_msb_plane_16_depth_valid_boundaries() {
    let size = ImageSize::new(4, 4);
    for lsb in [true, false] {
        for depth in [8, 16] {
            call_convert_plane_16(lsb, size, depth).expect("有効範囲の depth では Ok が返るべき");
        }
    }
}

// 正常系: convert_to_lsb_plane_16 の depth == 16 が恒等コピーになること。
// libyuv の ConvertToLSBPlane_16 は depth == 16 で SIMD 経路が全 0 を出力し、
// C 経路では符号付き乗算がオーバーフロー（未定義動作）になるため、本 crate では
// depth == 16 を sys::CopyPlane_16 による行コピーに置き換えている。
// stride > width とし C 側の行合体（coalesce）を無効化して、行ごとに
// 異なる画素値（32767 / 32768 / 65535）で行送りが正しい 16bit 幅で行われることも検証する
#[test]
fn convert_to_lsb_plane_16_depth_16_is_identity() {
    // stride > width のパディング付き配置にする（stride == width では CopyPlane の
    // 行合体が発動し、行送りコードが実行されないため）。src のパディングは
    // dst の初期値（0xFFFF）と異なる値にし、コピー実装が stride 単位で行全体を
    // 書き込んだ場合に検出できるようにする
    let width = 4;
    let height = 2;
    let stride = 6; // パディング 2 要素
    let src = [
        // 1 行目: 境界値 + パディング
        0u16, 32767, 32768, 65535, 0x5A5A, 0x5A5A,
        // 2 行目: 1 行目と異なる画素値（行送りの誤りを検出するため）+ パディング
        65535, 32768, 32767, 0, 0x5A5A, 0x5A5A,
    ];
    let size = ImageSize::new(width, height);
    let mut dst = [0xFFFFu16; 12];

    convert_to_lsb_plane_16(&src, stride, &mut dst, stride, size, 16)
        .expect("depth == 16 では Ok が返るべき");
    // この assert は主に SIMD 経路の全 0 出力の回帰を検出する。C 経路のみの環境では
    // 旧実装も 2 の補数ラップにより見かけ上恒等になるため検出不能（UB は出力の観測では
    // 検証できない）。CI の SIMD 対応ランナー（Linux / macOS）では修正前コードが全 0 を
    // 出すため回帰を検出できる
    let expected = [
        0u16, 32767, 32768, 65535, 0xFFFF, 0xFFFF, 65535, 32768, 32767, 0, 0xFFFF, 0xFFFF,
    ];
    assert_eq!(
        dst, expected,
        "depth == 16 は恒等コピーになり入力と一致するべき"
    );
}

// 異常系: convert_to_lsb_plane_16 の depth == 16 で、要素単位では c_int 範囲内でも
// バイト単位（stride * 2 / width * 2）にすると c_int の範囲を超える場合に Err が返ること
#[test]
fn convert_to_lsb_plane_16_depth_16_stride_bytes_exceeds_c_int() {
    let src = vec![0u16; 1];
    let mut dst = vec![0u16; 1];
    let size = ImageSize::new(1, 1);
    // 要素単位では c_int 範囲内だが、* 2 すると範囲を超える最小の stride を使う
    // （half_float_plane 側は内部で * 2 してから判定するため c_int::MAX でも検証が
    // 発火したが、こちらは要素単位の require_c_int を通る必要があるため、* 2 超過の
    // 最小境界 (c_int::MAX / 2 + 1) を選ぶ）
    let max_stride = (c_int::MAX as usize) / 2 + 1;

    let result = convert_to_lsb_plane_16(&src, max_stride, &mut dst, 1, size, 16);
    let err = result.expect_err("src 側のバイト単位 stride 超過では Err が返るべき");
    assert!(
        err.to_string()
            .contains("source stride (bytes) exceeds c_int range"),
        "src 側のバイト単位 stride 超過の reason が返るべき: {}",
        err
    );

    let result = convert_to_lsb_plane_16(&src, 1, &mut dst, max_stride, size, 16);
    let err = result.expect_err("dst 側のバイト単位 stride 超過では Err が返るべき");
    assert!(
        err.to_string()
            .contains("destination stride (bytes) exceeds c_int range"),
        "dst 側のバイト単位 stride 超過の reason が返るべき: {}",
        err
    );
}

// 正常系: convert_to_lsb_plane_16 の depth == 16 はゼロサイズ入力で no-op（Ok）になること。
// ゼロサイズは恒等コピーの意味論としても no-op であり、libyuv の C 実装
// （ConvertToLSBPlane_16 / CopyPlane_16）も早期 return するため、Ok を返す契約を固定する
#[test]
fn convert_to_lsb_plane_16_depth_16_zero_size_is_noop() {
    let src = [0u16; 0];
    let mut dst = [0u16; 0];

    let result = convert_to_lsb_plane_16(&src, 0, &mut dst, 0, ImageSize::new(0, 0), 16);
    assert!(
        result.is_ok(),
        "ゼロサイズ + depth == 16 では Ok（no-op）が返るべき"
    );
}

// 異常系: convert_to_lsb_plane_16 / convert_to_msb_plane_16 の depth が有効範囲
// (8..=16) 外で Err が返ること
#[test]
fn convert_to_lsb_msb_plane_16_depth_out_of_range() {
    let size = ImageSize::new(4, 4);
    // 境界値 (下限 7 / 上限 17) と負値・極端な値を LSB / MSB 両方で検証する
    for lsb in [true, false] {
        for depth in [7, 17, -1, i32::MIN, i32::MAX] {
            let err = call_convert_plane_16(lsb, size, depth).expect_err(&format!(
                "範囲外の depth では Err が返るべき (lsb={lsb}, depth={depth})"
            ));
            assert!(
                err.to_string().contains("depth must be between 8 and 16"),
                "depth 検証の reason が返るべき: {}",
                err
            );
        }
    }
    // C 側のシフト式が特に危険な代表値 (LSB は 31: 1 << 31 が int 表現不能、
    // MSB は -15: 1 << (16 - (-15)) = 1 << 31 が int 表現不能) でも、C に渡る前に
    // Rust 側の検証で Err になること
    for (lsb, depth) in [(true, 31), (false, -15)] {
        let err = call_convert_plane_16(lsb, size, depth).expect_err(&format!(
            "範囲外の depth では Err が返るべき (lsb={lsb}, depth={depth})"
        ));
        assert!(
            err.to_string().contains("depth must be between 8 and 16"),
            "depth 検証の reason が返るべき: {}",
            err
        );
    }
    // ゼロサイズ入力でも仕様として Err になること (LSB / MSB 両方)
    for lsb in [true, false] {
        let err = call_convert_plane_16(lsb, ImageSize::new(0, 0), 7)
            .expect_err("ゼロサイズ + 範囲外 depth では Err が返るべき");
        assert!(
            err.to_string().contains("depth must be between 8 and 16"),
            "depth 検証の reason が返るべき: {}",
            err
        );
    }
}
