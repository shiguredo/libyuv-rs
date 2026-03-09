//! [libyuv] 画像変換・処理ライブラリの Rust バインディング
//!
//! [libyuv]: https://chromium.googlesource.com/libyuv/libyuv/
#![warn(missing_docs)]
#![allow(clippy::too_many_arguments)]

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

fn validate_yuv_src_inner(
    y: &[u8],
    y_stride: usize,
    u: &[u8],
    u_stride: usize,
    v: &[u8],
    v_stride: usize,
    size: ImageSize,
    uv_height_divisor: usize,
    function: &'static str,
) -> Result<(), Error> {
    if y.len() < y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            function,
            "source Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    if u.len() < u_stride * uv_height {
        return Err(Error::with_reason(
            -1,
            function,
            "source U buffer too small",
        ));
    }
    if v.len() < v_stride * uv_height {
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
    function: &'static str,
) -> Result<(), Error> {
    if y.len() < y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            function,
            "destination Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    if u.len() < u_stride * uv_height {
        return Err(Error::with_reason(
            -1,
            function,
            "destination U buffer too small",
        ));
    }
    if v.len() < v_stride * uv_height {
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
    function: &'static str,
) -> Result<(), Error> {
    if y.len() < y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            function,
            "source Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    if uv.len() < uv_stride * uv_height {
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
    function: &'static str,
) -> Result<(), Error> {
    if y.len() < y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            function,
            "destination Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    if uv.len() < uv_stride * uv_height {
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
    function: &'static str,
) -> Result<(), Error> {
    if y.len() < y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            function,
            "source Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    if u.len() < u_stride * uv_height {
        return Err(Error::with_reason(
            -1,
            function,
            "source U buffer too small",
        ));
    }
    if v.len() < v_stride * uv_height {
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
    function: &'static str,
) -> Result<(), Error> {
    if y.len() < y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            function,
            "destination Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    if u.len() < u_stride * uv_height {
        return Err(Error::with_reason(
            -1,
            function,
            "destination U buffer too small",
        ));
    }
    if v.len() < v_stride * uv_height {
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
    function: &'static str,
) -> Result<(), Error> {
    if y.len() < y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            function,
            "source Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    if uv.len() < uv_stride * uv_height {
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
    function: &'static str,
) -> Result<(), Error> {
    if y.len() < y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            function,
            "destination Y buffer too small",
        ));
    }
    let uv_height = size.height.div_ceil(uv_height_divisor);
    if uv.len() < uv_stride * uv_height {
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
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $uv_div:expr) => {
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
            /// ソースバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_yuv_src_inner(
                    self.y, self.y_stride,
                    self.u, self.u_stride,
                    self.v, self.v_stride,
                    size, $uv_div, function,
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

            /// デスティネーションバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_yuv_dst_inner(
                    self.y, self.y_stride,
                    self.u, self.u_stride,
                    self.v, self.v_stride,
                    size, $uv_div, function,
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
            /// ソースバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                if self.y.len() < self.y_stride * size.height {
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

            /// デスティネーションバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                if self.y.len() < self.y_stride * size.height {
                    return Err(Error::with_reason(-1, function, "destination Y buffer too small"));
                }
                Ok(())
            }
        }
    };
}

/// 2 プレーン NV 画像型を定義するマクロ
macro_rules! define_nv_image {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $uv_div:expr) => {
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
            /// ソースバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_nv_src_inner(
                    self.y, self.y_stride,
                    self.uv, self.uv_stride,
                    size, $uv_div, function,
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

            /// デスティネーションバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_nv_dst_inner(
                    self.y, self.y_stride,
                    self.uv, self.uv_stride,
                    size, $uv_div, function,
                )
            }
        }
    };
}

/// パック形式画像型を定義するマクロ
macro_rules! define_packed_image {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident) => {
        $(#[doc = $doc])*
        #[derive(Debug)]
        pub struct $name<'a> {
            /// ピクセルデータ
            pub data: &'a [u8],
            /// ストライド（行あたりのバイト数）
            pub stride: usize,
        }

        impl $name<'_> {
            /// ソースバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                if self.data.len() < self.stride * size.height {
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

            /// デスティネーションバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                if self.data.len() < self.stride * size.height {
                    return Err(Error::with_reason(-1, function, "destination buffer too small"));
                }
                Ok(())
            }
        }
    };
}

/// 3 プレーン 16bit YUV 画像型を定義するマクロ
macro_rules! define_yuv_image16 {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $uv_div:expr) => {
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
            /// ソースバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_yuv16_src_inner(
                    self.y, self.y_stride,
                    self.u, self.u_stride,
                    self.v, self.v_stride,
                    size, $uv_div, function,
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

            /// デスティネーションバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_yuv16_dst_inner(
                    self.y, self.y_stride,
                    self.u, self.u_stride,
                    self.v, self.v_stride,
                    size, $uv_div, function,
                )
            }
        }
    };
}

/// 2 プレーン 16bit NV 画像型を定義するマクロ
macro_rules! define_nv_image16 {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident, $uv_div:expr) => {
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
            /// ソースバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_nv16_src_inner(
                    self.y, self.y_stride,
                    self.uv, self.uv_stride,
                    size, $uv_div, function,
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

            /// デスティネーションバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                validate_nv16_dst_inner(
                    self.y, self.y_stride,
                    self.uv, self.uv_stride,
                    size, $uv_div, function,
                )
            }
        }
    };
}

/// パック形式 16bit 画像型を定義するマクロ
macro_rules! define_packed_image16 {
    ($(#[doc = $doc:expr])* $name:ident, $(#[doc = $doc_mut:expr])* $name_mut:ident) => {
        $(#[doc = $doc])*
        #[derive(Debug)]
        pub struct $name<'a> {
            /// ピクセルデータ
            pub data: &'a [u16],
            /// ストライド（行あたりの要素数）
            pub stride: usize,
        }

        impl $name<'_> {
            /// ソースバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                if self.data.len() < self.stride * size.height {
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

            /// デスティネーションバッファのバリデーション
            #[allow(dead_code)]
            pub(crate) fn validate(&self, size: ImageSize, function: &'static str) -> Result<(), Error> {
                if self.data.len() < self.stride * size.height {
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

// 4:2:0 (UV 高さ = height / 2)
define_yuv_image!(/// I420 画像 (YUV 4:2:0, BT.601 limited range)
    I420Image, /// I420 画像 (可変)
    I420ImageMut, 2);
define_yuv_image!(/// J420 画像 (YUV 4:2:0, BT.601 full range)
    J420Image, /// J420 画像 (可変)
    J420ImageMut, 2);
define_yuv_image!(/// H420 画像 (YUV 4:2:0, BT.709 limited range)
    H420Image, /// H420 画像 (可変)
    H420ImageMut, 2);
define_yuv_image!(/// U420 画像 (YUV 4:2:0, BT.2020 limited range)
    U420Image, /// U420 画像 (可変)
    U420ImageMut, 2);
define_yuv_image!(/// Android420 画像 (YUV 4:2:0, Android カメラ形式)
    Android420Image, /// Android420 画像 (可変)
    Android420ImageMut, 2);

// 4:2:2 (UV 高さ = height)
define_yuv_image!(/// I422 画像 (YUV 4:2:2, BT.601 limited range)
    I422Image, /// I422 画像 (可変)
    I422ImageMut, 1);
define_yuv_image!(/// J422 画像 (YUV 4:2:2, BT.601 full range)
    J422Image, /// J422 画像 (可変)
    J422ImageMut, 1);
define_yuv_image!(/// H422 画像 (YUV 4:2:2, BT.709 limited range)
    H422Image, /// H422 画像 (可変)
    H422ImageMut, 1);
define_yuv_image!(/// U422 画像 (YUV 4:2:2, BT.2020 limited range)
    U422Image, /// U422 画像 (可変)
    U422ImageMut, 1);

// 4:4:4 (UV 高さ = height)
define_yuv_image!(/// I444 画像 (YUV 4:4:4, BT.601 limited range)
    I444Image, /// I444 画像 (可変)
    I444ImageMut, 1);
define_yuv_image!(/// J444 画像 (YUV 4:4:4, BT.601 full range)
    J444Image, /// J444 画像 (可変)
    J444ImageMut, 1);
define_yuv_image!(/// H444 画像 (YUV 4:4:4, BT.709 limited range)
    H444Image, /// H444 画像 (可変)
    H444ImageMut, 1);
define_yuv_image!(/// U444 画像 (YUV 4:4:4, BT.2020 limited range)
    U444Image, /// U444 画像 (可変)
    U444ImageMut, 1);

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

// 4:2:0 (UV 高さ = height / 2)
define_nv_image!(/// NV12 画像 (Y + UV インターリーブ, 4:2:0)
    Nv12Image, /// NV12 画像 (可変)
    Nv12ImageMut, 2);
define_nv_image!(/// NV21 画像 (Y + VU インターリーブ, 4:2:0)
    Nv21Image, /// NV21 画像 (可変)
    Nv21ImageMut, 2);
define_nv_image!(/// MM21 画像 (タイル形式, 4:2:0)
    Mm21Image, /// MM21 画像 (可変)
    Mm21ImageMut, 2);
define_nv_image!(/// MT2T 画像 (10bit タイル形式, 4:2:0)
    Mt2tImage, /// MT2T 画像 (可変)
    Mt2tImageMut, 2);

// 4:2:2 (UV 高さ = height)
define_nv_image!(/// NV16 画像 (Y + UV インターリーブ, 4:2:2)
    Nv16Image, /// NV16 画像 (可変)
    Nv16ImageMut, 1);

// 4:4:4 (UV 高さ = height)
define_nv_image!(/// NV24 画像 (Y + UV インターリーブ, 4:4:4)
    Nv24Image, /// NV24 画像 (可変)
    Nv24ImageMut, 1);

// ============================================================
// パック形式 8bit 画像型
// ============================================================

// 4 bytes/pixel
define_packed_image!(/// ARGB 画像 (4 bytes/pixel)
    ArgbImage, /// ARGB 画像 (可変)
    ArgbImageMut);
define_packed_image!(/// ABGR 画像 (4 bytes/pixel)
    AbgrImage, /// ABGR 画像 (可変)
    AbgrImageMut);
define_packed_image!(/// RGBA 画像 (4 bytes/pixel)
    RgbaImage, /// RGBA 画像 (可変)
    RgbaImageMut);
define_packed_image!(/// BGRA 画像 (4 bytes/pixel)
    BgraImage, /// BGRA 画像 (可変)
    BgraImageMut);
define_packed_image!(/// AR30 画像 (10bit packed, 4 bytes/pixel)
    Ar30Image, /// AR30 画像 (可変)
    Ar30ImageMut);
define_packed_image!(/// AB30 画像 (10bit packed, 4 bytes/pixel)
    Ab30Image, /// AB30 画像 (可変)
    Ab30ImageMut);
define_packed_image!(/// AYUV 画像 (4 bytes/pixel)
    AyuvImage, /// AYUV 画像 (可変)
    AyuvImageMut);

// 3 bytes/pixel
define_packed_image!(/// RGB24 画像 (3 bytes/pixel)
    Rgb24Image, /// RGB24 画像 (可変)
    Rgb24ImageMut);
define_packed_image!(/// RAW 画像 (3 bytes/pixel, RGB 逆順)
    RawImage, /// RAW 画像 (可変)
    RawImageMut);
define_packed_image!(/// YUV24 画像 (3 bytes/pixel)
    Yuv24Image, /// YUV24 画像 (可変)
    Yuv24ImageMut);

// 2 bytes/pixel
define_packed_image!(/// RGB565 画像 (2 bytes/pixel)
    Rgb565Image, /// RGB565 画像 (可変)
    Rgb565ImageMut);
define_packed_image!(/// ARGB1555 画像 (2 bytes/pixel)
    Argb1555Image, /// ARGB1555 画像 (可変)
    Argb1555ImageMut);
define_packed_image!(/// ARGB4444 画像 (2 bytes/pixel)
    Argb4444Image, /// ARGB4444 画像 (可変)
    Argb4444ImageMut);
define_packed_image!(/// YUY2 画像 (パック YUV 4:2:2, 2 bytes/pixel)
    Yuy2Image, /// YUY2 画像 (可変)
    Yuy2ImageMut);
define_packed_image!(/// UYVY 画像 (パック YUV 4:2:2, 2 bytes/pixel)
    UyvyImage, /// UYVY 画像 (可変)
    UyvyImageMut);

// ============================================================
// 3 プレーン 16bit YUV 画像型
// ============================================================

// 4:2:0 (UV 高さ = height / 2)
define_yuv_image16!(/// I010 画像 (10bit YUV 4:2:0)
    I010Image, /// I010 画像 (可変)
    I010ImageMut, 2);
define_yuv_image16!(/// I012 画像 (12bit YUV 4:2:0)
    I012Image, /// I012 画像 (可変)
    I012ImageMut, 2);
define_yuv_image16!(/// H010 画像 (10bit YUV 4:2:0, BT.709)
    H010Image, /// H010 画像 (可変)
    H010ImageMut, 2);
define_yuv_image16!(/// U010 画像 (10bit YUV 4:2:0, BT.2020)
    U010Image, /// U010 画像 (可変)
    U010ImageMut, 2);

// 4:2:2 (UV 高さ = height)
define_yuv_image16!(/// I210 画像 (10bit YUV 4:2:2)
    I210Image, /// I210 画像 (可変)
    I210ImageMut, 1);
define_yuv_image16!(/// I212 画像 (12bit YUV 4:2:2)
    I212Image, /// I212 画像 (可変)
    I212ImageMut, 1);
define_yuv_image16!(/// H210 画像 (10bit YUV 4:2:2, BT.709)
    H210Image, /// H210 画像 (可変)
    H210ImageMut, 1);
define_yuv_image16!(/// U210 画像 (10bit YUV 4:2:2, BT.2020)
    U210Image, /// U210 画像 (可変)
    U210ImageMut, 1);

// 4:4:4 (UV 高さ = height)
define_yuv_image16!(/// I410 画像 (10bit YUV 4:4:4)
    I410Image, /// I410 画像 (可変)
    I410ImageMut, 1);
define_yuv_image16!(/// I412 画像 (12bit YUV 4:4:4)
    I412Image, /// I412 画像 (可変)
    I412ImageMut, 1);

// ============================================================
// 2 プレーン 16bit NV 画像型
// ============================================================

// 4:2:0 (UV 高さ = height / 2)
define_nv_image16!(/// P010 画像 (10bit Y + UV, 4:2:0)
    P010Image, /// P010 画像 (可変)
    P010ImageMut, 2);
define_nv_image16!(/// P012 画像 (12bit Y + UV, 4:2:0)
    P012Image, /// P012 画像 (可変)
    P012ImageMut, 2);

// 4:2:2 (UV 高さ = height)
define_nv_image16!(/// P210 画像 (10bit Y + UV, 4:2:2)
    P210Image, /// P210 画像 (可変)
    P210ImageMut, 1);
define_nv_image16!(/// P212 画像 (12bit Y + UV, 4:2:2)
    P212Image, /// P212 画像 (可変)
    P212ImageMut, 1);

// 4:4:4 (UV 高さ = height)
define_nv_image16!(/// P410 画像 (10bit Y + UV, 4:4:4)
    P410Image, /// P410 画像 (可変)
    P410ImageMut, 1);

// ============================================================
// パック形式 16bit 画像型
// ============================================================

define_packed_image16!(/// AR64 画像 (16bit ARGB, 4 要素/pixel)
    Ar64Image, /// AR64 画像 (可変)
    Ar64ImageMut);
define_packed_image16!(/// AB64 画像 (16bit ABGR, 4 要素/pixel)
    Ab64Image, /// AB64 画像 (可変)
    Ab64ImageMut);
