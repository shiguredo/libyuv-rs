//! PBT 用の共通ヘルパー

use proptest::prelude::*;

/// 偶数の幅・高さを生成する Strategy（I420 は偶数必須）
pub fn even_size() -> impl Strategy<Value = (usize, usize)> {
    (1..=64usize, 1..=64usize).prop_map(|(w, h)| (w * 2, h * 2))
}

/// 幅が 32 の倍数、高さが偶数のサイズを生成する Strategy
pub fn aligned32_size() -> impl Strategy<Value = (usize, usize)> {
    (1..=4usize, 1..=64usize).prop_map(|(w, h)| (w * 32, h * 2))
}

/// ランダムな I420 バッファを生成する Strategy
pub fn arb_i420(width: usize, height: usize) -> impl Strategy<Value = (Vec<u8>, Vec<u8>, Vec<u8>)> {
    let y_size = width * height;
    let uv_size = (width / 2) * (height / 2);
    (
        proptest::collection::vec(0..=255u8, y_size),
        proptest::collection::vec(0..=255u8, uv_size),
        proptest::collection::vec(0..=255u8, uv_size),
    )
}

/// ランダムな Biplanar (NV12/NV21) バッファを生成する Strategy
pub fn arb_biplanar(width: usize, height: usize) -> impl Strategy<Value = (Vec<u8>, Vec<u8>)> {
    let y_size = width * height;
    let chroma_size = width * (height / 2);
    (
        proptest::collection::vec(0..=255u8, y_size),
        proptest::collection::vec(0..=255u8, chroma_size),
    )
}

/// ランダムなパック画像バッファを生成する Strategy
pub fn arb_packed(
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
) -> impl Strategy<Value = Vec<u8>> {
    proptest::collection::vec(0..=255u8, width * height * bytes_per_pixel)
}

/// ランダムな単一プレーンバッファを生成する Strategy
pub fn arb_plane(width: usize, height: usize) -> impl Strategy<Value = Vec<u8>> {
    proptest::collection::vec(0..=255u8, width * height)
}
