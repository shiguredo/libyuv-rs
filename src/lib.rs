//! [libyuv] 画像変換・処理ライブラリの Rust バインディング
//!
//! [libyuv]: https://chromium.googlesource.com/libyuv/libyuv/
#![warn(missing_docs)]
#![expect(clippy::too_many_arguments)]

mod sys;

mod compare;
mod convert;
mod planar;
mod rotate;
mod scale;

pub use compare::*;
pub use convert::*;
pub use planar::*;
pub use rotate::*;
pub use scale::*;

/// ビルド時に参照したリポジトリ URL
pub const BUILD_REPOSITORY: &str = sys::BUILD_METADATA_REPOSITORY;

/// ビルド時に参照したリポジトリのバージョン（タグ）
pub const BUILD_VERSION: &str = sys::BUILD_METADATA_VERSION;

/// エラー
#[derive(Debug)]
pub struct Error {
    code: i32,
    function: &'static str,
    reason: Option<&'static str>,
}

impl Error {
    fn new(code: i32, function: &'static str, reason: Option<&'static str>) -> Self {
        Self {
            code,
            function,
            reason,
        }
    }

    fn with_reason(code: i32, function: &'static str, reason: &'static str) -> Self {
        Self::new(code, function, Some(reason))
    }

    fn check(code: i32, function: &'static str) -> Result<(), Self> {
        if code == 0 {
            Ok(())
        } else {
            Err(Self::new(code, function, None))
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(reason) = self.reason {
            write!(
                f,
                "{}() failed: code={}, reason={reason}",
                self.function, self.code
            )
        } else {
            write!(f, "{}() failed: code={}", self.function, self.code)
        }
    }
}

impl std::error::Error for Error {}

/// スケール品質フィルタ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    /// なし（最も高速だが品質は最低）
    None,
    /// 線形フィルタ（高速で適度な品質）
    Linear,
    /// バイリニア（中程度の速度と品質）
    Bilinear,
    /// ボックスフィルタ（中程度の速度、ダウンスケール時に有効）
    Box,
}

impl FilterMode {
    fn to_sys(self) -> sys::FilterMode {
        match self {
            FilterMode::None => sys::FilterMode_kFilterNone,
            FilterMode::Linear => sys::FilterMode_kFilterLinear,
            FilterMode::Bilinear => sys::FilterMode_kFilterBilinear,
            FilterMode::Box => sys::FilterMode_kFilterBox,
        }
    }
}

/// 回転モード
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationMode {
    /// 回転なし（0 度）
    None,
    /// 時計回りに 90 度回転
    Rotate90,
    /// 180 度回転
    Rotate180,
    /// 時計回りに 270 度回転（反時計回り 90 度）
    Rotate270,
}

impl RotationMode {
    fn to_sys(self) -> sys::RotationMode {
        match self {
            RotationMode::None => sys::RotationMode_kRotate0,
            RotationMode::Rotate90 => sys::RotationMode_kRotate90,
            RotationMode::Rotate180 => sys::RotationMode_kRotate180,
            RotationMode::Rotate270 => sys::RotationMode_kRotate270,
        }
    }

    /// 回転後の出力サイズを返す
    pub(crate) fn output_size(self, src: ImageSize) -> ImageSize {
        match self {
            RotationMode::None | RotationMode::Rotate180 => src,
            RotationMode::Rotate90 | RotationMode::Rotate270 => {
                ImageSize::new(src.height, src.width)
            }
        }
    }
}

/// 画像の幅と高さ
#[derive(Debug, Clone, Copy)]
pub struct ImageSize {
    /// 画像の幅
    pub width: usize,
    /// 画像の高さ
    pub height: usize,
}

impl ImageSize {
    /// 新しい画像サイズを作成
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

// ============================================================
// バッファサイズ検証用の内部ヘルパー関数
// ============================================================

use std::ffi::c_int;

/// usize の値が c_int の範囲内であることを検証する
fn require_c_int(value: usize, function: &'static str, reason: &'static str) -> Result<(), Error> {
    if c_int::try_from(value).is_err() {
        return Err(Error::with_reason(-1, function, reason));
    }
    Ok(())
}

/// stride * height をオーバーフロー安全に計算する
fn checked_buf_size(
    stride: usize,
    height: usize,
    function: &'static str,
    reason: &'static str,
) -> Result<usize, Error> {
    stride
        .checked_mul(height)
        .ok_or_else(|| Error::with_reason(-1, function, reason))
}

/// タイル配置の必要サイズ（最終タイル行を含む）をオーバーフロー安全に計算する。
/// タイル行間隔は stride * tile_height、最終タイル行の読み出しは tile_row_size バイト
/// （libyuv の行送り規則。planar_functions.cc の DetilePlane と convert.cc の
/// MT2TToP010 を参照）
fn checked_tiled_buf_size(
    tile_rows: usize,
    stride: usize,
    tile_height: usize,
    tile_row_size: usize,
    function: &'static str,
) -> Result<usize, Error> {
    (tile_rows - 1)
        .checked_mul(stride)
        .and_then(|v| v.checked_mul(tile_height))
        .and_then(|v| v.checked_add(tile_row_size))
        .ok_or_else(|| Error::with_reason(-1, function, "tiled buffer size overflow"))
}

/// タイル行の読み出しサイズをオーバーフロー安全に計算する。
/// 10bit パック（MT2T）は 10/8 倍になる。padded_width は 16 の倍数のため、
/// 10/8 倍しても切り捨ては発生しない
fn checked_tile_row_size(
    padded_width: usize,
    tile_height: usize,
    is_10bit: bool,
    function: &'static str,
) -> Result<usize, Error> {
    padded_width
        .checked_mul(tile_height)
        .and_then(|v| {
            if is_10bit {
                v.checked_mul(10).map(|w| w / 8)
            } else {
                Some(v)
            }
        })
        .ok_or_else(|| Error::with_reason(-1, function, "tile row size overflow"))
}

/// タイル形式（MM21 / MT2T）のソースバッファ検証。
///
/// タイル配置では 1 タイル行の読み出し幅が、幅を 16 の倍数に切り上げた値 × タイル高になり、
/// 線形サイズ（stride * height）より大きくなりうる。タイル行数は Y / UV とも
/// `height.div_ceil(32)`（`ceil(ceil(h / 2) / 16) == ceil(h / 32)` のため。UV の
/// タイル高は 16、Y は 32）。MM21 は 8bit、MT2T は 10bit パック（10/8 倍）で読み出す。
///
/// 検証は安全側（過大要求）である。MM21 は実際には最終タイル列の余り分と最終タイル行の
/// 端数行分を読まないため、式は最大で 1 タイル行分過大になる。MT2T は部分タイル行でも
/// フルサイズを読むため正確。
fn validate_tiled_nv_src_inner(
    y: &[u8],
    y_stride: usize,
    uv: &[u8],
    uv_stride: usize,
    size: ImageSize,
    function: &'static str,
    is_10bit: bool,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    require_c_int(y_stride, function, "Y stride exceeds c_int range")?;
    require_c_int(uv_stride, function, "UV stride exceeds c_int range")?;

    // ゼロサイズはタイル行数の計算（height.div_ceil(32) - 1）がアンダーフローするため
    // 先に Err にする。MM21 は height == 0 でも libyuv が no-op 成功を返すため、
    // この Err は検証側で一律に定める（0064 のゼロサイズ統一方針と整合）
    if size.width == 0 || size.height == 0 {
        return Err(Error::with_reason(
            -1,
            function,
            "width and height must be greater than 0",
        ));
    }

    // stride 下限チェック（線形検証と同じ。タイル行間隔が stride * tile_height のため、
    // stride が小さすぎるとタイル行の読み出しが次の行と重なる）
    if y_stride < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "Y stride smaller than width",
        ));
    }
    let min_uv_stride = size
        .width
        .div_ceil(2)
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, function, "UV minimum stride overflow"))?;
    if uv_stride < min_uv_stride {
        return Err(Error::with_reason(
            -1,
            function,
            "UV stride smaller than chroma width",
        ));
    }

    // タイル行の読み出し幅（16 の倍数に切り上げ）。MM21 の UV 幅 (width + 1) & ~1 も
    // 16 の倍数に切り上げると width の切り上げと一致するため、Y / UV とも同じ幅になる
    let padded_width = size
        .width
        .div_ceil(16)
        .checked_mul(16)
        .ok_or_else(|| Error::with_reason(-1, function, "padded width overflow"))?;

    // タイル行数（Y / UV で共通。UV のタイル高 16 と Y の 32 で ceil が一致する）
    let tile_rows = size.height.div_ceil(32);

    // 最終タイル行の読み出しサイズ（Y: タイル高 32、UV: タイル高 16）
    let y_tile_row_size = checked_tile_row_size(padded_width, 32, is_10bit, function)?;
    let uv_tile_row_size = checked_tile_row_size(padded_width, 16, is_10bit, function)?;

    // 必要サイズの検証
    let y_size = checked_tiled_buf_size(tile_rows, y_stride, 32, y_tile_row_size, function)?;
    if y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source Y buffer too small",
        ));
    }
    let uv_size = checked_tiled_buf_size(tile_rows, uv_stride, 16, uv_tile_row_size, function)?;
    if uv.len() < uv_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source UV buffer too small",
        ));
    }
    Ok(())
}

fn validate_yuv_src_inner(
    y: &[u8],
    y_stride: usize,
    u: &[u8],
    u_stride: usize,
    v: &[u8],
    v_stride: usize,
    size: ImageSize,
    uv_height_divisor: usize,
    uv_width_divisor: usize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    require_c_int(y_stride, function, "Y stride exceeds c_int range")?;
    require_c_int(u_stride, function, "U stride exceeds c_int range")?;
    require_c_int(v_stride, function, "V stride exceeds c_int range")?;
    // stride >= 最小幅チェック
    if y_stride < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "Y stride smaller than width",
        ));
    }
    let uv_width = size.width.div_ceil(uv_width_divisor);
    if u_stride < uv_width {
        return Err(Error::with_reason(
            -1,
            function,
            "U stride smaller than chroma width",
        ));
    }
    if v_stride < uv_width {
        return Err(Error::with_reason(
            -1,
            function,
            "V stride smaller than chroma width",
        ));
    }
    // バッファサイズチェック (オーバーフロー安全)
    let y_size = checked_buf_size(y_stride, size.height, function, "Y buffer size overflow")?;
    if y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    let u_size = checked_buf_size(u_stride, uv_height, function, "U buffer size overflow")?;
    if u.len() < u_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source U buffer too small",
        ));
    }
    let v_size = checked_buf_size(v_stride, uv_height, function, "V buffer size overflow")?;
    if v.len() < v_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source V buffer too small",
        ));
    }
    Ok(())
}

fn validate_yuv_dst_inner(
    y: &[u8],
    y_stride: usize,
    u: &[u8],
    u_stride: usize,
    v: &[u8],
    v_stride: usize,
    size: ImageSize,
    uv_height_divisor: usize,
    uv_width_divisor: usize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    require_c_int(y_stride, function, "Y stride exceeds c_int range")?;
    require_c_int(u_stride, function, "U stride exceeds c_int range")?;
    require_c_int(v_stride, function, "V stride exceeds c_int range")?;
    // stride >= 最小幅チェック
    if y_stride < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "Y stride smaller than width",
        ));
    }
    let uv_width = size.width.div_ceil(uv_width_divisor);
    if u_stride < uv_width {
        return Err(Error::with_reason(
            -1,
            function,
            "U stride smaller than chroma width",
        ));
    }
    if v_stride < uv_width {
        return Err(Error::with_reason(
            -1,
            function,
            "V stride smaller than chroma width",
        ));
    }
    // バッファサイズチェック (オーバーフロー安全)
    let y_size = checked_buf_size(y_stride, size.height, function, "Y buffer size overflow")?;
    if y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    let u_size = checked_buf_size(u_stride, uv_height, function, "U buffer size overflow")?;
    if u.len() < u_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination U buffer too small",
        ));
    }
    let v_size = checked_buf_size(v_stride, uv_height, function, "V buffer size overflow")?;
    if v.len() < v_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination V buffer too small",
        ));
    }
    Ok(())
}

fn validate_nv_src_inner(
    y: &[u8],
    y_stride: usize,
    uv: &[u8],
    uv_stride: usize,
    size: ImageSize,
    uv_height_divisor: usize,
    uv_width_divisor: usize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    require_c_int(y_stride, function, "Y stride exceeds c_int range")?;
    require_c_int(uv_stride, function, "UV stride exceeds c_int range")?;
    // stride >= 最小幅チェック
    if y_stride < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "Y stride smaller than width",
        ));
    }
    // UV はインターリーブなので最小幅 = ceil(width / uv_width_div) * 2
    let min_uv_stride = size
        .width
        .div_ceil(uv_width_divisor)
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, function, "UV minimum stride overflow"))?;
    if uv_stride < min_uv_stride {
        return Err(Error::with_reason(
            -1,
            function,
            "UV stride smaller than chroma width",
        ));
    }
    // バッファサイズチェック (オーバーフロー安全)
    let y_size = checked_buf_size(y_stride, size.height, function, "Y buffer size overflow")?;
    if y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    let uv_size = checked_buf_size(uv_stride, uv_height, function, "UV buffer size overflow")?;
    if uv.len() < uv_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source UV buffer too small",
        ));
    }
    Ok(())
}

fn validate_nv_dst_inner(
    y: &[u8],
    y_stride: usize,
    uv: &[u8],
    uv_stride: usize,
    size: ImageSize,
    uv_height_divisor: usize,
    uv_width_divisor: usize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    require_c_int(y_stride, function, "Y stride exceeds c_int range")?;
    require_c_int(uv_stride, function, "UV stride exceeds c_int range")?;
    // stride >= 最小幅チェック
    if y_stride < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "Y stride smaller than width",
        ));
    }
    let min_uv_stride = size
        .width
        .div_ceil(uv_width_divisor)
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, function, "UV minimum stride overflow"))?;
    if uv_stride < min_uv_stride {
        return Err(Error::with_reason(
            -1,
            function,
            "UV stride smaller than chroma width",
        ));
    }
    // バッファサイズチェック (オーバーフロー安全)
    let y_size = checked_buf_size(y_stride, size.height, function, "Y buffer size overflow")?;
    if y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    let uv_size = checked_buf_size(uv_stride, uv_height, function, "UV buffer size overflow")?;
    if uv.len() < uv_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination UV buffer too small",
        ));
    }
    Ok(())
}

fn validate_yuv16_src_inner(
    y: &[u16],
    y_stride: usize,
    u: &[u16],
    u_stride: usize,
    v: &[u16],
    v_stride: usize,
    size: ImageSize,
    uv_height_divisor: usize,
    uv_width_divisor: usize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    require_c_int(y_stride, function, "Y stride exceeds c_int range")?;
    require_c_int(u_stride, function, "U stride exceeds c_int range")?;
    require_c_int(v_stride, function, "V stride exceeds c_int range")?;
    // stride >= 最小幅チェック (16bit: ストライドは要素数、width はピクセル数)
    if y_stride < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "Y stride smaller than width",
        ));
    }
    let uv_width = size.width.div_ceil(uv_width_divisor);
    if u_stride < uv_width {
        return Err(Error::with_reason(
            -1,
            function,
            "U stride smaller than chroma width",
        ));
    }
    if v_stride < uv_width {
        return Err(Error::with_reason(
            -1,
            function,
            "V stride smaller than chroma width",
        ));
    }
    // バッファサイズチェック (オーバーフロー安全)
    let y_size = checked_buf_size(y_stride, size.height, function, "Y buffer size overflow")?;
    if y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    let u_size = checked_buf_size(u_stride, uv_height, function, "U buffer size overflow")?;
    if u.len() < u_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source U buffer too small",
        ));
    }
    let v_size = checked_buf_size(v_stride, uv_height, function, "V buffer size overflow")?;
    if v.len() < v_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source V buffer too small",
        ));
    }
    Ok(())
}

fn validate_yuv16_dst_inner(
    y: &[u16],
    y_stride: usize,
    u: &[u16],
    u_stride: usize,
    v: &[u16],
    v_stride: usize,
    size: ImageSize,
    uv_height_divisor: usize,
    uv_width_divisor: usize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    require_c_int(y_stride, function, "Y stride exceeds c_int range")?;
    require_c_int(u_stride, function, "U stride exceeds c_int range")?;
    require_c_int(v_stride, function, "V stride exceeds c_int range")?;
    // stride >= 最小幅チェック
    if y_stride < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "Y stride smaller than width",
        ));
    }
    let uv_width = size.width.div_ceil(uv_width_divisor);
    if u_stride < uv_width {
        return Err(Error::with_reason(
            -1,
            function,
            "U stride smaller than chroma width",
        ));
    }
    if v_stride < uv_width {
        return Err(Error::with_reason(
            -1,
            function,
            "V stride smaller than chroma width",
        ));
    }
    // バッファサイズチェック (オーバーフロー安全)
    let y_size = checked_buf_size(y_stride, size.height, function, "Y buffer size overflow")?;
    if y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    let u_size = checked_buf_size(u_stride, uv_height, function, "U buffer size overflow")?;
    if u.len() < u_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination U buffer too small",
        ));
    }
    let v_size = checked_buf_size(v_stride, uv_height, function, "V buffer size overflow")?;
    if v.len() < v_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination V buffer too small",
        ));
    }
    Ok(())
}

fn validate_nv16_src_inner(
    y: &[u16],
    y_stride: usize,
    uv: &[u16],
    uv_stride: usize,
    size: ImageSize,
    uv_height_divisor: usize,
    uv_width_divisor: usize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    require_c_int(y_stride, function, "Y stride exceeds c_int range")?;
    require_c_int(uv_stride, function, "UV stride exceeds c_int range")?;
    // stride >= 最小幅チェック
    if y_stride < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "Y stride smaller than width",
        ));
    }
    // 16bit NV: UV はインターリーブ、最小要素数 = ceil(width / uv_width_div) * 2
    let min_uv_stride = size
        .width
        .div_ceil(uv_width_divisor)
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, function, "UV minimum stride overflow"))?;
    if uv_stride < min_uv_stride {
        return Err(Error::with_reason(
            -1,
            function,
            "UV stride smaller than chroma width",
        ));
    }
    // バッファサイズチェック (オーバーフロー安全)
    let y_size = checked_buf_size(y_stride, size.height, function, "Y buffer size overflow")?;
    if y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    let uv_size = checked_buf_size(uv_stride, uv_height, function, "UV buffer size overflow")?;
    if uv.len() < uv_size {
        return Err(Error::with_reason(
            -1,
            function,
            "source UV buffer too small",
        ));
    }
    Ok(())
}

fn validate_nv16_dst_inner(
    y: &[u16],
    y_stride: usize,
    uv: &[u16],
    uv_stride: usize,
    size: ImageSize,
    uv_height_divisor: usize,
    uv_width_divisor: usize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    require_c_int(y_stride, function, "Y stride exceeds c_int range")?;
    require_c_int(uv_stride, function, "UV stride exceeds c_int range")?;
    // stride >= 最小幅チェック
    if y_stride < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "Y stride smaller than width",
        ));
    }
    let min_uv_stride = size
        .width
        .div_ceil(uv_width_divisor)
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, function, "UV minimum stride overflow"))?;
    if uv_stride < min_uv_stride {
        return Err(Error::with_reason(
            -1,
            function,
            "UV stride smaller than chroma width",
        ));
    }
    // バッファサイズチェック (オーバーフロー安全)
    let y_size = checked_buf_size(y_stride, size.height, function, "Y buffer size overflow")?;
    if y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    let uv_size = checked_buf_size(uv_stride, uv_height, function, "UV buffer size overflow")?;
    if uv.len() < uv_size {
        return Err(Error::with_reason(
            -1,
            function,
            "destination UV buffer too small",
        ));
    }
    Ok(())
}

// ============================================================
// 型定義マクロ
// ============================================================

/// 3 プレーン YUV 画像型を定義するマクロ
macro_rules! define_yuv_image {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $uv_height_div:expr, $uv_width_div:expr) => {
        $(#[doc = $doc])*
        #[derive(Debug)]
        pub struct $name<'a> {
            /// Y プレーンデータ
            pub y: &'a [u8],
            /// Y プレーンのストライド（行あたりのバイト数）
            pub y_stride: usize,
            /// U プレーンデータ
            pub u: &'a [u8],
            /// U プレーンのストライド
            pub u_stride: usize,
            /// V プレーンデータ
            pub v: &'a [u8],
            /// V プレーンのストライド
            pub v_stride: usize,
        }

        impl $name<'_> {
            #[allow(dead_code)]
            /// ソースバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_yuv_src_inner(
                    self.y, self.y_stride,
                    self.u, self.u_stride,
                    self.v, self.v_stride,
                    size, $uv_height_div, $uv_width_div, function,
                )
            }
        }

        $(#[doc = $doc_mut])*
        #[derive(Debug)]
        pub struct $name_mut<'a> {
            /// Y プレーンデータ
            pub y: &'a mut [u8],
            /// Y プレーンのストライド（行あたりのバイト数）
            pub y_stride: usize,
            /// U プレーンデータ
            pub u: &'a mut [u8],
            /// U プレーンのストライド
            pub u_stride: usize,
            /// V プレーンデータ
            pub v: &'a mut [u8],
            /// V プレーンのストライド
            pub v_stride: usize,
        }

        impl $name_mut<'_> {
            /// 不変参照への変換
            pub fn as_ref(&self) -> $name<'_> {
                $name {
                    y: self.y,
                    y_stride: self.y_stride,
                    u: self.u,
                    u_stride: self.u_stride,
                    v: self.v,
                    v_stride: self.v_stride,
                }
            }

            #[allow(dead_code)]
            /// デスティネーションバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_yuv_dst_inner(
                    self.y, self.y_stride,
                    self.u, self.u_stride,
                    self.v, self.v_stride,
                    size, $uv_height_div, $uv_width_div, function,
                )
            }
        }
    };
}

/// Y プレーンのみの画像型を定義するマクロ
macro_rules! define_y_image {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident) => {
        $(#[doc = $doc])*
        #[derive(Debug)]
        pub struct $name<'a> {
            /// Y プレーンデータ
            pub y: &'a [u8],
            /// Y プレーンのストライド（行あたりのバイト数）
            pub y_stride: usize,
        }

        impl $name<'_> {
            #[allow(dead_code)]
            /// ソースバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                require_c_int(size.width, function, "width exceeds c_int range")?;
                require_c_int(size.height, function, "height exceeds c_int range")?;
                require_c_int(self.y_stride, function, "Y stride exceeds c_int range")?;
                if self.y_stride < size.width {
                    return Err(Error::with_reason(-1, function, "Y stride smaller than width"));
                }
                let buf_size = checked_buf_size(self.y_stride, size.height, function, "buffer size overflow")?;
                if self.y.len() < buf_size {
                    return Err(Error::with_reason(-1, function, "source Y buffer too small"));
                }
                Ok(())
            }
        }

        $(#[doc = $doc_mut])*
        #[derive(Debug)]
        pub struct $name_mut<'a> {
            /// Y プレーンデータ
            pub y: &'a mut [u8],
            /// Y プレーンのストライド（行あたりのバイト数）
            pub y_stride: usize,
        }

        impl $name_mut<'_> {
            /// 不変参照への変換
            pub fn as_ref(&self) -> $name<'_> {
                $name {
                    y: self.y,
                    y_stride: self.y_stride,
                }
            }

            #[allow(dead_code)]
            /// デスティネーションバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                require_c_int(size.width, function, "width exceeds c_int range")?;
                require_c_int(size.height, function, "height exceeds c_int range")?;
                require_c_int(self.y_stride, function, "Y stride exceeds c_int range")?;
                if self.y_stride < size.width {
                    return Err(Error::with_reason(-1, function, "Y stride smaller than width"));
                }
                let buf_size = checked_buf_size(self.y_stride, size.height, function, "buffer size overflow")?;
                if self.y.len() < buf_size {
                    return Err(Error::with_reason(-1, function, "destination Y buffer too small"));
                }
                Ok(())
            }
        }
    };
}

/// 2 プレーン NV 画像型を定義するマクロ
macro_rules! define_nv_image {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $uv_height_div:expr, $uv_width_div:expr) => {
        $(#[doc = $doc])*
        #[derive(Debug)]
        pub struct $name<'a> {
            /// Y プレーンデータ
            pub y: &'a [u8],
            /// Y プレーンのストライド（行あたりのバイト数）
            pub y_stride: usize,
            /// UV プレーンデータ（インターリーブ）
            pub uv: &'a [u8],
            /// UV プレーンのストライド
            pub uv_stride: usize,
        }

        impl $name<'_> {
            #[allow(dead_code)]
            /// ソースバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_nv_src_inner(
                    self.y, self.y_stride,
                    self.uv, self.uv_stride,
                    size, $uv_height_div, $uv_width_div, function,
                )
            }
        }

        $(#[doc = $doc_mut])*
        #[derive(Debug)]
        pub struct $name_mut<'a> {
            /// Y プレーンデータ
            pub y: &'a mut [u8],
            /// Y プレーンのストライド（行あたりのバイト数）
            pub y_stride: usize,
            /// UV プレーンデータ（インターリーブ）
            pub uv: &'a mut [u8],
            /// UV プレーンのストライド
            pub uv_stride: usize,
        }

        impl $name_mut<'_> {
            /// 不変参照への変換
            pub fn as_ref(&self) -> $name<'_> {
                $name {
                    y: self.y,
                    y_stride: self.y_stride,
                    uv: self.uv,
                    uv_stride: self.uv_stride,
                }
            }

            #[allow(dead_code)]
            /// デスティネーションバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_nv_dst_inner(
                    self.y, self.y_stride,
                    self.uv, self.uv_stride,
                    size, $uv_height_div, $uv_width_div, function,
                )
            }
        }
    };
}

/// パック形式画像型を定義するマクロ
macro_rules! define_packed_image {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $bpp:expr) => {
        $(#[doc = $doc])*
        #[derive(Debug)]
        pub struct $name<'a> {
            /// ピクセルデータ
            pub data: &'a [u8],
            /// ストライド（行あたりのバイト数）
            pub stride: usize,
        }

        impl $name<'_> {
            #[allow(dead_code)]
            /// ソースバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                require_c_int(size.width, function, "width exceeds c_int range")?;
                require_c_int(size.height, function, "height exceeds c_int range")?;
                require_c_int(self.stride, function, "stride exceeds c_int range")?;
                let min_stride = size.width.checked_mul($bpp)
                    .ok_or_else(|| Error::with_reason(-1, function, "minimum stride overflow"))?;
                if self.stride < min_stride {
                    return Err(Error::with_reason(-1, function, "stride smaller than width * bpp"));
                }
                let buf_size = checked_buf_size(self.stride, size.height, function, "buffer size overflow")?;
                if self.data.len() < buf_size {
                    return Err(Error::with_reason(-1, function, "source buffer too small"));
                }
                Ok(())
            }
        }

        $(#[doc = $doc_mut])*
        #[derive(Debug)]
        pub struct $name_mut<'a> {
            /// ピクセルデータ
            pub data: &'a mut [u8],
            /// ストライド（行あたりのバイト数）
            pub stride: usize,
        }

        impl $name_mut<'_> {
            /// 不変参照への変換
            pub fn as_ref(&self) -> $name<'_> {
                $name {
                    data: self.data,
                    stride: self.stride,
                }
            }

            #[allow(dead_code)]
            /// デスティネーションバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                require_c_int(size.width, function, "width exceeds c_int range")?;
                require_c_int(size.height, function, "height exceeds c_int range")?;
                require_c_int(self.stride, function, "stride exceeds c_int range")?;
                let min_stride = size.width.checked_mul($bpp)
                    .ok_or_else(|| Error::with_reason(-1, function, "minimum stride overflow"))?;
                if self.stride < min_stride {
                    return Err(Error::with_reason(-1, function, "stride smaller than width * bpp"));
                }
                let buf_size = checked_buf_size(self.stride, size.height, function, "buffer size overflow")?;
                if self.data.len() < buf_size {
                    return Err(Error::with_reason(-1, function, "destination buffer too small"));
                }
                Ok(())
            }
        }
    };
}

/// 3 プレーン 16bit YUV 画像型を定義するマクロ
macro_rules! define_yuv_image16 {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $uv_height_div:expr, $uv_width_div:expr) => {
        $(#[doc = $doc])*
        #[derive(Debug)]
        pub struct $name<'a> {
            /// Y プレーンデータ
            pub y: &'a [u16],
            /// Y プレーンのストライド（行あたりの要素数）
            pub y_stride: usize,
            /// U プレーンデータ
            pub u: &'a [u16],
            /// U プレーンのストライド
            pub u_stride: usize,
            /// V プレーンデータ
            pub v: &'a [u16],
            /// V プレーンのストライド
            pub v_stride: usize,
        }

        impl $name<'_> {
            #[allow(dead_code)]
            /// ソースバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_yuv16_src_inner(
                    self.y, self.y_stride,
                    self.u, self.u_stride,
                    self.v, self.v_stride,
                    size, $uv_height_div, $uv_width_div, function,
                )
            }
        }

        $(#[doc = $doc_mut])*
        #[derive(Debug)]
        pub struct $name_mut<'a> {
            /// Y プレーンデータ
            pub y: &'a mut [u16],
            /// Y プレーンのストライド（行あたりの要素数）
            pub y_stride: usize,
            /// U プレーンデータ
            pub u: &'a mut [u16],
            /// U プレーンのストライド
            pub u_stride: usize,
            /// V プレーンデータ
            pub v: &'a mut [u16],
            /// V プレーンのストライド
            pub v_stride: usize,
        }

        impl $name_mut<'_> {
            /// 不変参照への変換
            pub fn as_ref(&self) -> $name<'_> {
                $name {
                    y: self.y,
                    y_stride: self.y_stride,
                    u: self.u,
                    u_stride: self.u_stride,
                    v: self.v,
                    v_stride: self.v_stride,
                }
            }

            #[allow(dead_code)]
            /// デスティネーションバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_yuv16_dst_inner(
                    self.y, self.y_stride,
                    self.u, self.u_stride,
                    self.v, self.v_stride,
                    size, $uv_height_div, $uv_width_div, function,
                )
            }
        }
    };
}

/// 2 プレーン 16bit NV 画像型を定義するマクロ
macro_rules! define_nv_image16 {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $uv_height_div:expr, $uv_width_div:expr) => {
        $(#[doc = $doc])*
        #[derive(Debug)]
        pub struct $name<'a> {
            /// Y プレーンデータ
            pub y: &'a [u16],
            /// Y プレーンのストライド（行あたりの要素数）
            pub y_stride: usize,
            /// UV プレーンデータ（インターリーブ）
            pub uv: &'a [u16],
            /// UV プレーンのストライド
            pub uv_stride: usize,
        }

        impl $name<'_> {
            #[allow(dead_code)]
            /// ソースバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_nv16_src_inner(
                    self.y, self.y_stride,
                    self.uv, self.uv_stride,
                    size, $uv_height_div, $uv_width_div, function,
                )
            }
        }

        $(#[doc = $doc_mut])*
        #[derive(Debug)]
        pub struct $name_mut<'a> {
            /// Y プレーンデータ
            pub y: &'a mut [u16],
            /// Y プレーンのストライド（行あたりの要素数）
            pub y_stride: usize,
            /// UV プレーンデータ（インターリーブ）
            pub uv: &'a mut [u16],
            /// UV プレーンのストライド
            pub uv_stride: usize,
        }

        impl $name_mut<'_> {
            /// 不変参照への変換
            pub fn as_ref(&self) -> $name<'_> {
                $name {
                    y: self.y,
                    y_stride: self.y_stride,
                    uv: self.uv,
                    uv_stride: self.uv_stride,
                }
            }

            #[allow(dead_code)]
            /// デスティネーションバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_nv16_dst_inner(
                    self.y, self.y_stride,
                    self.uv, self.uv_stride,
                    size, $uv_height_div, $uv_width_div, function,
                )
            }
        }
    };
}

/// パック形式 16bit 画像型を定義するマクロ
macro_rules! define_packed_image16 {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $epp:expr) => {
        $(#[doc = $doc])*
        #[derive(Debug)]
        pub struct $name<'a> {
            /// ピクセルデータ
            pub data: &'a [u16],
            /// ストライド（行あたりの要素数）
            pub stride: usize,
        }

        impl $name<'_> {
            #[allow(dead_code)]
            /// ソースバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                require_c_int(size.width, function, "width exceeds c_int range")?;
                require_c_int(size.height, function, "height exceeds c_int range")?;
                require_c_int(self.stride, function, "stride exceeds c_int range")?;
                let min_stride = size.width.checked_mul($epp)
                    .ok_or_else(|| Error::with_reason(-1, function, "minimum stride overflow"))?;
                if self.stride < min_stride {
                    return Err(Error::with_reason(-1, function, "stride smaller than width * epp"));
                }
                let buf_size = checked_buf_size(self.stride, size.height, function, "buffer size overflow")?;
                if self.data.len() < buf_size {
                    return Err(Error::with_reason(-1, function, "source buffer too small"));
                }
                Ok(())
            }
        }

        $(#[doc = $doc_mut])*
        #[derive(Debug)]
        pub struct $name_mut<'a> {
            /// ピクセルデータ
            pub data: &'a mut [u16],
            /// ストライド（行あたりの要素数）
            pub stride: usize,
        }

        impl $name_mut<'_> {
            /// 不変参照への変換
            pub fn as_ref(&self) -> $name<'_> {
                $name {
                    data: self.data,
                    stride: self.stride,
                }
            }

            #[allow(dead_code)]
            /// デスティネーションバッファのバリデーション
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                require_c_int(size.width, function, "width exceeds c_int range")?;
                require_c_int(size.height, function, "height exceeds c_int range")?;
                require_c_int(self.stride, function, "stride exceeds c_int range")?;
                let min_stride = size.width.checked_mul($epp)
                    .ok_or_else(|| Error::with_reason(-1, function, "minimum stride overflow"))?;
                if self.stride < min_stride {
                    return Err(Error::with_reason(-1, function, "stride smaller than width * epp"));
                }
                let buf_size = checked_buf_size(self.stride, size.height, function, "buffer size overflow")?;
                if self.data.len() < buf_size {
                    return Err(Error::with_reason(-1, function, "destination buffer too small"));
                }
                Ok(())
            }
        }
    };
}

// ============================================================
// 3 プレーン 8bit YUV 画像型
// ============================================================

// 4:2:0 (UV 高さ = height / 2, UV 幅 = width / 2)
define_yuv_image!(/// I420 画像 (YUV 4:2:0, BT.601 limited range)
    I420Image, /// I420 画像 (可変)
    I420ImageMut, 2, 2);
define_yuv_image!(/// J420 画像 (YUV 4:2:0, BT.601 full range)
    J420Image, /// J420 画像 (可変)
    J420ImageMut, 2, 2);
define_yuv_image!(/// H420 画像 (YUV 4:2:0, BT.709 limited range)
    H420Image, /// H420 画像 (可変)
    H420ImageMut, 2, 2);
define_yuv_image!(/// U420 画像 (YUV 4:2:0, BT.2020 limited range)
    U420Image, /// U420 画像 (可変)
    U420ImageMut, 2, 2);
define_yuv_image!(/// Android420 画像 (YUV 4:2:0, Android カメラ形式)
    Android420Image, /// Android420 画像 (可変)
    Android420ImageMut, 2, 2);

// 4:2:2 (UV 高さ = height, UV 幅 = width / 2)
define_yuv_image!(/// I422 画像 (YUV 4:2:2, BT.601 limited range)
    I422Image, /// I422 画像 (可変)
    I422ImageMut, 1, 2);
define_yuv_image!(/// J422 画像 (YUV 4:2:2, BT.601 full range)
    J422Image, /// J422 画像 (可変)
    J422ImageMut, 1, 2);
define_yuv_image!(/// H422 画像 (YUV 4:2:2, BT.709 limited range)
    H422Image, /// H422 画像 (可変)
    H422ImageMut, 1, 2);
define_yuv_image!(/// U422 画像 (YUV 4:2:2, BT.2020 limited range)
    U422Image, /// U422 画像 (可変)
    U422ImageMut, 1, 2);

// 4:4:4 (UV 高さ = height, UV 幅 = width)
define_yuv_image!(/// I444 画像 (YUV 4:4:4, BT.601 limited range)
    I444Image, /// I444 画像 (可変)
    I444ImageMut, 1, 1);
define_yuv_image!(/// J444 画像 (YUV 4:4:4, BT.601 full range)
    J444Image, /// J444 画像 (可変)
    J444ImageMut, 1, 1);
define_yuv_image!(/// H444 画像 (YUV 4:4:4, BT.709 limited range)
    H444Image, /// H444 画像 (可変)
    H444ImageMut, 1, 1);
define_yuv_image!(/// U444 画像 (YUV 4:4:4, BT.2020 limited range)
    U444Image, /// U444 画像 (可変)
    U444ImageMut, 1, 1);

// ============================================================
// Y プレーンのみの 8bit 画像型
// ============================================================

define_y_image!(/// I400 画像 (グレースケール, BT.601 limited range)
    I400Image, /// I400 画像 (可変)
    I400ImageMut);
define_y_image!(/// J400 画像 (グレースケール, BT.601 full range)
    J400Image, /// J400 画像 (可変)
    J400ImageMut);

// ============================================================
// 2 プレーン 8bit NV 画像型
// ============================================================

// 4:2:0 (UV 高さ = height / 2, UV 幅 = width / 2)
define_nv_image!(/// NV12 画像 (Y + UV インターリーブ, 4:2:0)
    Nv12Image, /// NV12 画像 (可変)
    Nv12ImageMut, 2, 2);
define_nv_image!(/// NV21 画像 (Y + VU インターリーブ, 4:2:0)
    Nv21Image, /// NV21 画像 (可変)
    Nv21ImageMut, 2, 2);

/// MM21 画像 (MediaTek タイル形式, 4:2:0)
///
/// libyuv の `MM21To*` はタイル配置で入力バッファを読み出すため、バッファ検証は
/// 線形サイズではなくタイル配置の必要サイズ（`validate_tiled_nv_src_inner` 参照）で行う。
#[derive(Debug)]
pub struct Mm21Image<'a> {
    /// Y プレーンデータ
    pub y: &'a [u8],
    /// Y プレーンのストライド（行あたりのバイト数）
    pub y_stride: usize,
    /// UV プレーンデータ（インターリーブ）
    pub uv: &'a [u8],
    /// UV プレーンのストライド
    pub uv_stride: usize,
}

impl Mm21Image<'_> {
    /// ソースバッファのバリデーション
    pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
        validate_tiled_nv_src_inner(
            self.y,
            self.y_stride,
            self.uv,
            self.uv_stride,
            size,
            function,
            false,
        )
    }
}

/// MT2T 画像 (MediaTek 10bit タイル形式, 4:2:0)
///
/// `y_stride` / `uv_stride` は**バイト単位**で指定する。MT2T は 10bit パックのため、
/// 1 行のバイト数は幅の 10/8 倍になり、libyuv の `MT2TToP010` は stride をバイト単位で
/// 受け取る。バッファ検証はタイル配置と 10bit パックを反映した必要サイズ
/// （`validate_tiled_nv_src_inner` 参照）で行う。
#[derive(Debug)]
pub struct Mt2tImage<'a> {
    /// Y プレーンデータ
    pub y: &'a [u8],
    /// Y プレーンのストライド（行あたりのバイト数）
    pub y_stride: usize,
    /// UV プレーンデータ（インターリーブ）
    pub uv: &'a [u8],
    /// UV プレーンのストライド
    pub uv_stride: usize,
}

impl Mt2tImage<'_> {
    /// ソースバッファのバリデーション
    pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
        validate_tiled_nv_src_inner(
            self.y,
            self.y_stride,
            self.uv,
            self.uv_stride,
            size,
            function,
            true,
        )
    }
}

// 4:2:2 (UV 高さ = height, UV 幅 = width / 2)
define_nv_image!(/// NV16 画像 (Y + UV インターリーブ, 4:2:2)
    Nv16Image, /// NV16 画像 (可変)
    Nv16ImageMut, 1, 2);

// 4:4:4 (UV 高さ = height, UV 幅 = width)
define_nv_image!(/// NV24 画像 (Y + UV インターリーブ, 4:4:4)
    Nv24Image, /// NV24 画像 (可変)
    Nv24ImageMut, 1, 1);

// ============================================================
// パック形式 8bit 画像型
// ============================================================

// 4 bytes/pixel
define_packed_image!(/// ARGB 画像 (4 bytes/pixel)
    ArgbImage, /// ARGB 画像 (可変)
    ArgbImageMut, 4);
define_packed_image!(/// ABGR 画像 (4 bytes/pixel)
    AbgrImage, /// ABGR 画像 (可変)
    AbgrImageMut, 4);
define_packed_image!(/// RGBA 画像 (4 bytes/pixel)
    RgbaImage, /// RGBA 画像 (可変)
    RgbaImageMut, 4);
define_packed_image!(/// BGRA 画像 (4 bytes/pixel)
    BgraImage, /// BGRA 画像 (可変)
    BgraImageMut, 4);
define_packed_image!(/// AR30 画像 (10bit packed, 4 bytes/pixel)
    Ar30Image, /// AR30 画像 (可変)
    Ar30ImageMut, 4);
define_packed_image!(/// AB30 画像 (10bit packed, 4 bytes/pixel)
    Ab30Image, /// AB30 画像 (可変)
    Ab30ImageMut, 4);
define_packed_image!(/// AYUV 画像 (4 bytes/pixel)
    AyuvImage, /// AYUV 画像 (可変)
    AyuvImageMut, 4);

// 3 bytes/pixel
define_packed_image!(/// RGB24 画像 (3 bytes/pixel)
    Rgb24Image, /// RGB24 画像 (可変)
    Rgb24ImageMut, 3);
define_packed_image!(/// RAW 画像 (3 bytes/pixel, RGB 逆順)
    RawImage, /// RAW 画像 (可変)
    RawImageMut, 3);
define_packed_image!(/// YUV24 画像 (3 bytes/pixel)
    Yuv24Image, /// YUV24 画像 (可変)
    Yuv24ImageMut, 3);

// 2 bytes/pixel
define_packed_image!(/// RGB565 画像 (2 bytes/pixel)
    Rgb565Image, /// RGB565 画像 (可変)
    Rgb565ImageMut, 2);
define_packed_image!(/// ARGB1555 画像 (2 bytes/pixel)
    Argb1555Image, /// ARGB1555 画像 (可変)
    Argb1555ImageMut, 2);
define_packed_image!(/// ARGB4444 画像 (2 bytes/pixel)
    Argb4444Image, /// ARGB4444 画像 (可変)
    Argb4444ImageMut, 2);
define_packed_image!(/// YUY2 画像 (パック YUV 4:2:2, 2 bytes/pixel)
    Yuy2Image, /// YUY2 画像 (可変)
    Yuy2ImageMut, 2);
define_packed_image!(/// UYVY 画像 (パック YUV 4:2:2, 2 bytes/pixel)
    UyvyImage, /// UYVY 画像 (可変)
    UyvyImageMut, 2);

// ============================================================
// 3 プレーン 16bit YUV 画像型
// ============================================================

// 4:2:0 (UV 高さ = height / 2, UV 幅 = width / 2)
define_yuv_image16!(/// I010 画像 (10bit YUV 4:2:0)
    I010Image, /// I010 画像 (可変)
    I010ImageMut, 2, 2);
define_yuv_image16!(/// I012 画像 (12bit YUV 4:2:0)
    I012Image, /// I012 画像 (可変)
    I012ImageMut, 2, 2);
define_yuv_image16!(/// H010 画像 (10bit YUV 4:2:0, BT.709)
    H010Image, /// H010 画像 (可変)
    H010ImageMut, 2, 2);
define_yuv_image16!(/// U010 画像 (10bit YUV 4:2:0, BT.2020)
    U010Image, /// U010 画像 (可変)
    U010ImageMut, 2, 2);

// 4:2:2 (UV 高さ = height, UV 幅 = width / 2)
define_yuv_image16!(/// I210 画像 (10bit YUV 4:2:2)
    I210Image, /// I210 画像 (可変)
    I210ImageMut, 1, 2);
define_yuv_image16!(/// I212 画像 (12bit YUV 4:2:2)
    I212Image, /// I212 画像 (可変)
    I212ImageMut, 1, 2);
define_yuv_image16!(/// H210 画像 (10bit YUV 4:2:2, BT.709)
    H210Image, /// H210 画像 (可変)
    H210ImageMut, 1, 2);
define_yuv_image16!(/// U210 画像 (10bit YUV 4:2:2, BT.2020)
    U210Image, /// U210 画像 (可変)
    U210ImageMut, 1, 2);

// 4:4:4 (UV 高さ = height, UV 幅 = width)
define_yuv_image16!(/// I410 画像 (10bit YUV 4:4:4)
    I410Image, /// I410 画像 (可変)
    I410ImageMut, 1, 1);
define_yuv_image16!(/// I412 画像 (12bit YUV 4:4:4)
    I412Image, /// I412 画像 (可変)
    I412ImageMut, 1, 1);

// ============================================================
// 2 プレーン 16bit NV 画像型
// ============================================================

// 4:2:0 (UV 高さ = height / 2, UV 幅 = width / 2)
define_nv_image16!(/// P010 画像 (10bit Y + UV, 4:2:0)
    P010Image, /// P010 画像 (可変)
    P010ImageMut, 2, 2);
define_nv_image16!(/// P012 画像 (12bit Y + UV, 4:2:0)
    P012Image, /// P012 画像 (可変)
    P012ImageMut, 2, 2);

// 4:2:2 (UV 高さ = height, UV 幅 = width / 2)
define_nv_image16!(/// P210 画像 (10bit Y + UV, 4:2:2)
    P210Image, /// P210 画像 (可変)
    P210ImageMut, 1, 2);
define_nv_image16!(/// P212 画像 (12bit Y + UV, 4:2:2)
    P212Image, /// P212 画像 (可変)
    P212ImageMut, 1, 2);

// 4:4:4 (UV 高さ = height, UV 幅 = width)
define_nv_image16!(/// P410 画像 (10bit Y + UV, 4:4:4)
    P410Image, /// P410 画像 (可変)
    P410ImageMut, 1, 1);

// ============================================================
// パック形式 16bit 画像型
// ============================================================

define_packed_image16!(/// AR64 画像 (16bit ARGB, 4 要素/pixel)
    Ar64Image, /// AR64 画像 (可変)
    Ar64ImageMut, 4);
define_packed_image16!(/// AB64 画像 (16bit ABGR, 4 要素/pixel)
    Ab64Image, /// AB64 画像 (可変)
    Ab64ImageMut, 4);

#[cfg(test)]
mod tests {
    use super::*;

    // require_c_int: c_int::MAX は Ok を返すこと
    #[test]
    fn require_c_int_max_ok() {
        let result = require_c_int(c_int::MAX as usize, "test", "test reason");
        assert!(result.is_ok(), "c_int::MAX は Ok を返すべき");
    }

    // require_c_int: c_int::MAX + 1 は Err を返すこと
    #[test]
    fn require_c_int_overflow_err() {
        let result = require_c_int(c_int::MAX as usize + 1, "test", "test reason");
        assert!(result.is_err(), "c_int::MAX + 1 は Err を返すべき");
    }

    // checked_buf_size: 通常の乗算は Ok を返すこと
    #[test]
    fn checked_buf_size_normal_ok() {
        let result = checked_buf_size(8, 8, "test", "test reason");
        assert!(result.is_ok(), "8 * 8 は Ok を返すべき");
        assert_eq!(result.expect("Ok が返るはず"), 64);
    }

    // checked_buf_size: オーバーフローは Err を返すこと
    #[test]
    fn checked_buf_size_overflow_err() {
        let result = checked_buf_size(usize::MAX, 2, "test", "test reason");
        assert!(result.is_err(), "usize::MAX * 2 は Err を返すべき");
    }

    // validate_yuv_src_inner: 正常系で Ok を返すこと
    #[test]
    fn validate_yuv_src_inner_ok() {
        let y = vec![0u8; 64];
        let u = vec![0u8; 16];
        let v = vec![0u8; 16];
        let size = ImageSize::new(8, 8);
        let result = validate_yuv_src_inner(&y, 8, &u, 4, &v, 4, size, 2, 2, "test");
        assert!(result.is_ok(), "正常なバッファでは Ok を返すべき");
    }

    // validate_yuv_src_inner: バッファ不足で Err を返すこと
    #[test]
    fn validate_yuv_src_inner_buffer_too_small() {
        let y = vec![0u8; 10]; // 64 バイト必要だが 10 しか用意しない
        let u = vec![0u8; 16];
        let v = vec![0u8; 16];
        let size = ImageSize::new(8, 8);
        let result = validate_yuv_src_inner(&y, 8, &u, 4, &v, 4, size, 2, 2, "test");
        assert!(result.is_err(), "バッファ不足では Err を返すべき");
    }

    // validate_yuv_src_inner: stride 不足で Err を返すこと
    #[test]
    fn validate_yuv_src_inner_stride_too_small() {
        let y = vec![0u8; 64];
        let u = vec![0u8; 16];
        let v = vec![0u8; 16];
        let size = ImageSize::new(8, 8);
        let result = validate_yuv_src_inner(&y, 4, &u, 4, &v, 4, size, 2, 2, "test");
        assert!(result.is_err(), "stride 不足では Err を返すべき");
    }

    // validate_nv_src_inner: 正常系で Ok を返すこと
    #[test]
    fn validate_nv_src_inner_ok() {
        let y = vec![0u8; 64];
        let uv = vec![0u8; 32];
        let size = ImageSize::new(8, 8);
        let result = validate_nv_src_inner(&y, 8, &uv, 8, size, 2, 2, "test");
        assert!(result.is_ok(), "正常なバッファでは Ok を返すべき");
    }

    // validate_nv_src_inner: バッファ不足で Err を返すこと
    #[test]
    fn validate_nv_src_inner_buffer_too_small() {
        let y = vec![0u8; 10];
        let uv = vec![0u8; 32];
        let size = ImageSize::new(8, 8);
        let result = validate_nv_src_inner(&y, 8, &uv, 8, size, 2, 2, "test");
        assert!(result.is_err(), "バッファ不足では Err を返すべき");
    }
}
