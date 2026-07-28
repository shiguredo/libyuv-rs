//! フォーマット変換 API の単体テスト
//!
//! `detile_split_uv_plane` の正常系・異常系・境界値を検証する。

use shiguredo_libyuv::{ImageSize, detile_split_uv_plane};

// テスト用の基本パラメータ
// width=32, height=16, tile_height=16
// src_stride_uv = round_up(32, 16) = 32
// dst_stride_u = dst_stride_v = (32 + 1) / 2 = 16
// src_uv サイズ = 32 * ceil(16 / 16) * 16 = 512
// dst_u / dst_v サイズ = 16 * 16 = 256
const WIDTH: usize = 32;
const HEIGHT: usize = 16;
const TILE_HEIGHT: usize = 16;
const SRC_STRIDE_UV: usize = 32;
const DST_STRIDE_U: usize = 16;
const DST_STRIDE_V: usize = 16;
const SRC_UV_SIZE: usize = SRC_STRIDE_UV * HEIGHT.div_ceil(TILE_HEIGHT) * TILE_HEIGHT;
const DST_U_SIZE: usize = DST_STRIDE_U * HEIGHT;
const DST_V_SIZE: usize = DST_STRIDE_V * HEIGHT;

// 正常系: 適切なサイズのバッファで Ok(()) が返ること
#[test]
fn detile_split_uv_plane_ok() {
    let src_uv = vec![0u8; SRC_UV_SIZE];
    let mut dst_u = vec![0u8; DST_U_SIZE];
    let mut dst_v = vec![0u8; DST_V_SIZE];
    let size = ImageSize::new(WIDTH, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_ok());
    // 全ゼロ入力では出力も全ゼロであることを確認する
    assert!(dst_u.iter().all(|&b| b == 0));
    assert!(dst_v.iter().all(|&b| b == 0));
}

// 正常系: height が tile_height の非倍数（height=17, tile_height=16）でも Ok(()) が返ること
// タイル配置サイズは src_stride_uv * ceil(17/16) * 16 = 32 * 2 * 16 = 1024 となり、
// リニア相当サイズ (32 * 17 = 544) より大きくなる
#[test]
fn detile_split_uv_plane_height_not_multiple_of_tile_height() {
    let width: usize = 32;
    let height: usize = 17;
    let tile_height: usize = 16;
    let src_stride_uv: usize = 32; // round_up(32, 16) = 32
    let dst_stride_u: usize = 16; // (32 + 1) / 2 = 16
    let dst_stride_v: usize = 16;
    // タイル配置: 32 * ceil(17/16) * 16 = 32 * 2 * 16 = 1024
    let src_uv_size = src_stride_uv * height.div_ceil(tile_height) * tile_height;
    let dst_u_size = dst_stride_u * height;
    let dst_v_size = dst_stride_v * height;

    let src_uv = vec![0u8; src_uv_size];
    let mut dst_u = vec![0u8; dst_u_size];
    let mut dst_v = vec![0u8; dst_v_size];
    let size = ImageSize::new(width, height);

    let result = detile_split_uv_plane(
        &src_uv,
        src_stride_uv,
        &mut dst_u,
        dst_stride_u,
        &mut dst_v,
        dst_stride_v,
        size,
        tile_height,
    );
    assert!(result.is_ok());
}

// 正常系: 奇数 width（width=33）でも Ok(()) が返ること
// uv_plane_width = (33 + 1) / 2 = 17 となり、偶数 width とは異なる丸め経路を通る
#[test]
fn detile_split_uv_plane_odd_width() {
    let width: usize = 33;
    let height: usize = 16;
    let tile_height: usize = 16;
    let src_stride_uv: usize = 48; // round_up(33, 16) = 48
    let dst_stride_u: usize = 17; // (33 + 1) / 2 = 17
    let dst_stride_v: usize = 17;
    let src_uv_size = src_stride_uv * height.div_ceil(tile_height) * tile_height;
    let dst_u_size = dst_stride_u * height;
    let dst_v_size = dst_stride_v * height;

    let src_uv = vec![0u8; src_uv_size];
    let mut dst_u = vec![0u8; dst_u_size];
    let mut dst_v = vec![0u8; dst_v_size];
    let size = ImageSize::new(width, height);

    let result = detile_split_uv_plane(
        &src_uv,
        src_stride_uv,
        &mut dst_u,
        dst_stride_u,
        &mut dst_v,
        dst_stride_v,
        size,
        tile_height,
    );
    assert!(result.is_ok());
}

// 境界値: tile_height > height（height=8, tile_height=16）でも Ok(()) が返ること
// tile_groups = ceil(8/16) = 1 となり、src バッファは height より大きく要求される
#[test]
fn detile_split_uv_plane_tile_height_greater_than_height() {
    let width: usize = 32;
    let height: usize = 8;
    let tile_height: usize = 16;
    let src_stride_uv: usize = 32;
    let dst_stride_u: usize = 16;
    let dst_stride_v: usize = 16;
    // タイル配置: 32 * ceil(8/16) * 16 = 32 * 1 * 16 = 512
    let src_uv_size = src_stride_uv * height.div_ceil(tile_height) * tile_height;
    let dst_u_size = dst_stride_u * height;
    let dst_v_size = dst_stride_v * height;

    let src_uv = vec![0u8; src_uv_size];
    let mut dst_u = vec![0u8; dst_u_size];
    let mut dst_v = vec![0u8; dst_v_size];
    let size = ImageSize::new(width, height);

    let result = detile_split_uv_plane(
        &src_uv,
        src_stride_uv,
        &mut dst_u,
        dst_stride_u,
        &mut dst_v,
        dst_stride_v,
        size,
        tile_height,
    );
    assert!(result.is_ok());
}

// 異常系: src_uv バッファが不足するケースで Err が返ること
#[test]
fn detile_split_uv_plane_src_uv_too_small() {
    let src_uv = vec![0u8; SRC_UV_SIZE - 1];
    let mut dst_u = vec![0u8; DST_U_SIZE];
    let mut dst_v = vec![0u8; DST_V_SIZE];
    let size = ImageSize::new(WIDTH, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_err());
}

// 異常系: dst_u バッファが不足するケースで Err が返ること
#[test]
fn detile_split_uv_plane_dst_u_too_small() {
    let src_uv = vec![0u8; SRC_UV_SIZE];
    let mut dst_u = vec![0u8; DST_U_SIZE - 1];
    let mut dst_v = vec![0u8; DST_V_SIZE];
    let size = ImageSize::new(WIDTH, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_err());
}

// 異常系: dst_v バッファが不足するケースで Err が返ること
#[test]
fn detile_split_uv_plane_dst_v_too_small() {
    let src_uv = vec![0u8; SRC_UV_SIZE];
    let mut dst_u = vec![0u8; DST_U_SIZE];
    let mut dst_v = vec![0u8; DST_V_SIZE - 1];
    let size = ImageSize::new(WIDTH, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_err());
}

// 異常系: src_stride_uv が最小幅 (round_up(width, 16)) 未満のケースで Err が返ること
#[test]
fn detile_split_uv_plane_src_stride_too_small() {
    let src_uv = vec![0u8; SRC_UV_SIZE];
    let mut dst_u = vec![0u8; DST_U_SIZE];
    let mut dst_v = vec![0u8; DST_V_SIZE];
    let size = ImageSize::new(WIDTH, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV - 1,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_err());
}

// 異常系: dst_stride_u が最小幅 ((width + 1) / 2) 未満のケースで Err が返ること
#[test]
fn detile_split_uv_plane_dst_stride_u_too_small() {
    let src_uv = vec![0u8; SRC_UV_SIZE];
    let mut dst_u = vec![0u8; DST_U_SIZE];
    let mut dst_v = vec![0u8; DST_V_SIZE];
    let size = ImageSize::new(WIDTH, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U - 1,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_err());
}

// 異常系: dst_stride_v が最小幅 ((width + 1) / 2) 未満のケースで Err が返ること
#[test]
fn detile_split_uv_plane_dst_stride_v_too_small() {
    let src_uv = vec![0u8; SRC_UV_SIZE];
    let mut dst_u = vec![0u8; DST_U_SIZE];
    let mut dst_v = vec![0u8; DST_V_SIZE];
    let size = ImageSize::new(WIDTH, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V - 1,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_err());
}

// 異常系: tile_height が 0 のケースで Err が返ること
#[test]
fn detile_split_uv_plane_tile_height_zero() {
    let src_uv = vec![0u8; SRC_UV_SIZE];
    let mut dst_u = vec![0u8; DST_U_SIZE];
    let mut dst_v = vec![0u8; DST_V_SIZE];
    let size = ImageSize::new(WIDTH, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        0,
    );
    assert!(result.is_err());
}

// 異常系: tile_height が非 2 累乗 (3) のケースで Err が返ること
#[test]
fn detile_split_uv_plane_tile_height_not_power_of_two() {
    let src_uv = vec![0u8; SRC_UV_SIZE];
    let mut dst_u = vec![0u8; DST_U_SIZE];
    let mut dst_v = vec![0u8; DST_V_SIZE];
    let size = ImageSize::new(WIDTH, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        3,
    );
    assert!(result.is_err());
}

// 境界値: width=0 のケースで Ok(()) が返ること（libyuv は早期 return するため no-op）
#[test]
fn detile_split_uv_plane_width_zero() {
    let src_uv = vec![0u8; 16];
    let mut dst_u = vec![0u8; 16];
    let mut dst_v = vec![0u8; 16];
    let size = ImageSize::new(0, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_ok());
}

// 境界値: height=0 のケースで Ok(()) が返ること（libyuv は早期 return するため no-op）
#[test]
fn detile_split_uv_plane_height_zero() {
    let src_uv = vec![0u8; 16];
    let mut dst_u = vec![0u8; 16];
    let mut dst_v = vec![0u8; 16];
    let size = ImageSize::new(WIDTH, 0);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_ok());
}

// 異常系: width が c_int 範囲を超えるケースで Err が返ること
#[test]
fn detile_split_uv_plane_width_exceeds_c_int() {
    let src_uv = vec![0u8; 16];
    let mut dst_u = vec![0u8; 16];
    let mut dst_v = vec![0u8; 16];
    let size = ImageSize::new(i32::MAX as usize + 1, HEIGHT);

    let result = detile_split_uv_plane(
        &src_uv,
        SRC_STRIDE_UV,
        &mut dst_u,
        DST_STRIDE_U,
        &mut dst_v,
        DST_STRIDE_V,
        size,
        TILE_HEIGHT,
    );
    assert!(result.is_err());
}
