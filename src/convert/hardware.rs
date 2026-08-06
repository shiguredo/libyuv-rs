//! フォーマット変換関数 (hardware)

use std::ffi::c_int;

use crate::{
    AbgrImageMut, Android420Image, ArgbImageMut, AyuvImage, Error, I420ImageMut, ImageSize,
    Mm21Image, Mt2tImage, Nv12Image, Nv12ImageMut, Nv21ImageMut, P010ImageMut, Yuy2ImageMut,
    checked_buf_size, require_c_int, sys,
};

// ============================================================
// Android フォーマット
// ============================================================

/// Android420 から ARGB への変換
///
/// Android の NV12/NV21 系フォーマットから ARGB に変換する。
/// `pixel_stride_uv` は UV ピクセルストライド（1: planar、2: interleaved）。
///
/// `pixel_stride_uv == 2` のとき、`u` / `v` は同一バッファの連続領域（インターリーブ）で
/// なければならない（libyuv が `src_v - src_u` のポインタ減算を行うため。別スライスは
/// C 標準上は未定義動作になる。一般的な実装では整数減算に落ちるため実害はない）。
/// `u` / `v` のストライドは `2 * ceil(width / 2)` 以上である必要がある。検証はバッファ長に
/// `len() >= stride * ceil(height / 2)` を要求するため、共有バッファの末尾に 1 バイトの
/// パディングを確保すること（NV12 では v 側、NV21 では u 側のバッファ長が要求を満たす）。
pub fn android420_to_argb(
    src: &Android420Image<'_>,
    pixel_stride_uv: usize,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "Android420ToARGB")?;
    dst.validate(size, "Android420ToARGB")?;

    // pixel_stride_uv == 2 では libyuv は U/V をインターリーブデータとして 1 行
    // 2 * halfwidth バイト読み進める（高速パス NV12/NV21ToARGBMatrix とフォールバックの
    // WeavePixels の読み出し規則。convert_argb.cc の Android420ToARGBMatrix）。
    // そのためストライドをインターリーブ前提で検証する。バッファ長は src.validate が
    // u_stride * ceil(height / 2) を同一式で検査済みのため、ここでは検証しない
    match pixel_stride_uv {
        1 => {}
        2 => {
            let uv_stride = size.width.div_ceil(2).checked_mul(2).ok_or_else(|| {
                Error::with_reason(-1, "Android420ToARGB", "UV minimum stride overflow")
            })?;
            if src.u_stride < uv_stride {
                return Err(Error::with_reason(
                    -1,
                    "Android420ToARGB",
                    "U stride smaller than interleaved chroma width",
                ));
            }
            if src.v_stride < uv_stride {
                return Err(Error::with_reason(
                    -1,
                    "Android420ToARGB",
                    "V stride smaller than interleaved chroma width",
                ));
            }
        }
        _ => {
            return Err(Error::with_reason(
                -1,
                "Android420ToARGB",
                "pixel_stride_uv must be 1 or 2",
            ));
        }
    }

    // SAFETY: src / dst は .validate() と上記のインライン検証で検査可能な前提を検査済み。
    // 同一バッファ前提（pixel_stride_uv == 2）は docstring の契約に依存する。
    let result = unsafe {
        sys::Android420ToARGB(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            pixel_stride_uv as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "Android420ToARGB")
}

/// Android420 から ABGR への変換
///
/// `pixel_stride_uv` は UV ピクセルストライド（1: planar、2: interleaved）。
///
/// `pixel_stride_uv == 2` のとき、`u` / `v` は同一バッファの連続領域（インターリーブ）で
/// なければならない（libyuv が `src_v - src_u` のポインタ減算を行うため。別スライスは
/// C 標準上は未定義動作になる。一般的な実装では整数減算に落ちるため実害はない）。
/// `u` / `v` のストライドは `2 * ceil(width / 2)` 以上である必要がある。検証はバッファ長に
/// `len() >= stride * ceil(height / 2)` を要求するため、共有バッファの末尾に 1 バイトの
/// パディングを確保すること（NV12 では v 側、NV21 では u 側のバッファ長が要求を満たす）。
pub fn android420_to_abgr(
    src: &Android420Image<'_>,
    pixel_stride_uv: usize,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "Android420ToABGR")?;
    dst.validate(size, "Android420ToABGR")?;

    // pixel_stride_uv == 2 では libyuv は U/V をインターリーブデータとして 1 行
    // 2 * halfwidth バイト読み進める（高速パス NV12/NV21ToARGBMatrix とフォールバックの
    // WeavePixels の読み出し規則。convert_argb.cc の Android420ToARGBMatrix）。
    // そのためストライドをインターリーブ前提で検証する。バッファ長は src.validate が
    // u_stride * ceil(height / 2) を同一式で検査済みのため、ここでは検証しない
    match pixel_stride_uv {
        1 => {}
        2 => {
            let uv_stride = size.width.div_ceil(2).checked_mul(2).ok_or_else(|| {
                Error::with_reason(-1, "Android420ToABGR", "UV minimum stride overflow")
            })?;
            if src.u_stride < uv_stride {
                return Err(Error::with_reason(
                    -1,
                    "Android420ToABGR",
                    "U stride smaller than interleaved chroma width",
                ));
            }
            if src.v_stride < uv_stride {
                return Err(Error::with_reason(
                    -1,
                    "Android420ToABGR",
                    "V stride smaller than interleaved chroma width",
                ));
            }
        }
        _ => {
            return Err(Error::with_reason(
                -1,
                "Android420ToABGR",
                "pixel_stride_uv must be 1 or 2",
            ));
        }
    }

    // SAFETY: src / dst は .validate() と上記のインライン検証で検査可能な前提を検査済み。
    // 同一バッファ前提（pixel_stride_uv == 2）は docstring の契約に依存する。
    let result = unsafe {
        sys::Android420ToABGR(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            pixel_stride_uv as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "Android420ToABGR")
}

/// Android420 から I420 への変換
///
/// `pixel_stride_uv` は UV ピクセルストライド（1: planar、2: interleaved）。
///
/// `pixel_stride_uv == 2` のとき、`u` / `v` は同一バッファの連続領域（インターリーブ）で
/// なければならない（libyuv が `src_v - src_u` のポインタ減算を行うため。別スライスは
/// C 標準上は未定義動作になる。一般的な実装では整数減算に落ちるため実害はない）。
/// `u` / `v` のストライドは `2 * ceil(width / 2)` 以上である必要がある。検証はバッファ長に
/// `len() >= stride * ceil(height / 2)` を要求するため、共有バッファの末尾に 1 バイトの
/// パディングを確保すること（NV12 では v 側、NV21 では u 側のバッファ長が要求を満たす）。
pub fn android420_to_i420(
    src: &Android420Image<'_>,
    pixel_stride_uv: usize,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "Android420ToI420")?;
    dst.validate(size, "Android420ToI420")?;

    // pixel_stride_uv == 2 では libyuv は U/V をインターリーブデータとして 1 行
    // 2 * halfwidth バイト読み進める（Android420ToI420 は rotate.cc の
    // Android420ToI420Rotate を kRotate0 で呼ぶ。高速パス SplitRotateUV と
    // フォールバックの SplitPixels の読み出し規則）。そのためストライドを
    // インターリーブ前提で検証する。バッファ長は src.validate が
    // u_stride * ceil(height / 2) を同一式で検査済みのため、ここでは検証しない
    match pixel_stride_uv {
        1 => {}
        2 => {
            let uv_stride = size.width.div_ceil(2).checked_mul(2).ok_or_else(|| {
                Error::with_reason(-1, "Android420ToI420", "UV minimum stride overflow")
            })?;
            if src.u_stride < uv_stride {
                return Err(Error::with_reason(
                    -1,
                    "Android420ToI420",
                    "U stride smaller than interleaved chroma width",
                ));
            }
            if src.v_stride < uv_stride {
                return Err(Error::with_reason(
                    -1,
                    "Android420ToI420",
                    "V stride smaller than interleaved chroma width",
                ));
            }
        }
        _ => {
            return Err(Error::with_reason(
                -1,
                "Android420ToI420",
                "pixel_stride_uv must be 1 or 2",
            ));
        }
    }

    // SAFETY: src / dst は .validate() と上記のインライン検証で検査可能な前提を検査済み。
    // 同一バッファ前提（pixel_stride_uv == 2）は docstring の契約に依存する。
    let result = unsafe {
        sys::Android420ToI420(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            pixel_stride_uv as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "Android420ToI420")
}

// ============================================================
// MM21 (MediaTek タイル形式)
// ============================================================

/// MM21 から I420 への変換
pub fn mm21_to_i420(
    src: &Mm21Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "MM21ToI420")?;
    dst.validate(size, "MM21ToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::MM21ToI420(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.uv.as_ptr(),
            src.uv_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "MM21ToI420")
}

/// MM21 から NV12 への変換
pub fn mm21_to_nv12(
    src: &Mm21Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "MM21ToNV12")?;
    dst.validate(size, "MM21ToNV12")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::MM21ToNV12(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.uv.as_ptr(),
            src.uv_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "MM21ToNV12")
}

/// MM21 から YUY2 への変換
pub fn mm21_to_yuy2(
    src: &Mm21Image<'_>,
    dst: &mut Yuy2ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "MM21ToYUY2")?;
    dst.validate(size, "MM21ToYUY2")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::MM21ToYUY2(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.uv.as_ptr(),
            src.uv_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "MM21ToYUY2")
}

/// MT2T から P010 への変換
///
/// MediaTek のタイル形式から P010 に変換する。
pub fn mt2t_to_p010(
    src: &Mt2tImage<'_>,
    dst: &mut P010ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "MT2TToP010")?;
    dst.validate(size, "MT2TToP010")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::MT2TToP010(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.uv.as_ptr(),
            src.uv_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "MT2TToP010")
}

// ============================================================
// AYUV
// ============================================================

/// AYUV から NV12 への変換
pub fn ayuv_to_nv12(
    src: &AyuvImage<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "AYUVToNV12")?;
    dst.validate(size, "AYUVToNV12")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::AYUVToNV12(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "AYUVToNV12")
}

/// AYUV から NV21 への変換
pub fn ayuv_to_nv21(
    src: &AyuvImage<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "AYUVToNV21")?;
    dst.validate(size, "AYUVToNV21")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::AYUVToNV21(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "AYUVToNV21")
}

// ============================================================
// Detile (タイル解除)
// ============================================================

/// タイル化されたプレーンをリニアに変換する
///
/// `tile_height` は 2 の累乗でなければならない（libyuv 内部でビットマスクを使用するため）。
/// `src` はタイル配置である必要があり、ソースストライドは幅を 16 の倍数に切り上げた値
/// （round_up(width, 16)）以上でなければならない。ソースの必要サイズはタイル配置のスパン
/// `src_stride * ceil(height / tile_height) * tile_height` で検証する
/// （libyuv の DetilePlane は 1 タイル行を 16 バイトチャンクの列として読み、タイル高
/// ごとに `src_stride * tile_height` で次のタイル行へジャンプする。planar_functions.cc を
/// 参照。全タイル段を一括で要求する安全側の過大要求である）
pub fn detile_plane(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_stride: usize,
    size: ImageSize,
    tile_height: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, "DetilePlane", "width exceeds c_int range")?;
    require_c_int(size.height, "DetilePlane", "height exceeds c_int range")?;
    require_c_int(
        src_stride,
        "DetilePlane",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "DetilePlane",
        "destination stride exceeds c_int range",
    )?;
    require_c_int(
        tile_height,
        "DetilePlane",
        "tile_height exceeds c_int range",
    )?;

    // width == 0 || height == 0 は no-op で Ok を返す
    // （libyuv の DetilePlane は -1 を返すが、このクレートではゼロサイズ入力を
    //  no-op に統一する方針）
    if size.width == 0 || size.height == 0 {
        return Ok(());
    }

    // tile_height は 2 の累乗でなければならない（libyuv 内部でビットマスクを使用するため）
    if !tile_height.is_power_of_two() {
        return Err(Error::with_reason(
            -1,
            "DetilePlane",
            "tile_height must be a power of two",
        ));
    }

    // stride >= 最小幅チェック
    // タイル配置は 16 バイト単位で処理するため、src_stride は 16 の倍数に丸めた幅以上必要
    let src_min_stride =
        size.width.div_ceil(16).checked_mul(16).ok_or_else(|| {
            Error::with_reason(-1, "DetilePlane", "source minimum stride overflow")
        })?;
    if src_stride < src_min_stride {
        return Err(Error::with_reason(
            -1,
            "DetilePlane",
            "source stride smaller than round_up(width, 16)",
        ));
    }
    if dst_stride < size.width {
        return Err(Error::with_reason(
            -1,
            "DetilePlane",
            "destination stride smaller than width",
        ));
    }

    // バッファサイズ検証（オーバーフロー安全）
    // src はタイル配置: src_stride * ceil(height / tile_height) * tile_height
    // （1 タイル行は 16 バイト間隔のチャンク列のため、線形サイズより大きくなりうる）
    let tile_groups = size.height.div_ceil(tile_height);
    let src_size = src_stride
        .checked_mul(tile_groups)
        .and_then(|v| v.checked_mul(tile_height))
        .ok_or_else(|| Error::with_reason(-1, "DetilePlane", "source buffer size overflow"))?;
    if src.len() < src_size {
        return Err(Error::with_reason(
            -1,
            "DetilePlane",
            "source buffer too small",
        ));
    }
    // dst はリニア出力
    let dst_size = checked_buf_size(
        dst_stride,
        size.height,
        "DetilePlane",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_size {
        return Err(Error::with_reason(
            -1,
            "DetilePlane",
            "destination buffer too small",
        ));
    }

    // SAFETY: 上記の検証が全前提条件を検査済み。
    let result = unsafe {
        sys::DetilePlane(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
            tile_height as c_int,
        )
    };

    Error::check(result, "DetilePlane")
}

/// タイル化された 16bit プレーンをリニアに変換する
///
/// `tile_height` は 2 の累乗でなければならない（libyuv 内部でビットマスクを使用するため）。
/// `src` はタイル配置である必要があり、ソースストライドは幅を 16 の倍数に切り上げた値
/// （round_up(width, 16)）以上でなければならない。ソースの必要サイズはタイル配置のスパン
/// `src_stride * ceil(height / tile_height) * tile_height` で検証する
/// （libyuv の DetilePlane_16 は 1 タイル行を 16 要素チャンクの列として読み、タイル高
/// ごとに `src_stride * tile_height` で次のタイル行へジャンプする。planar_functions.cc を
/// 参照。全タイル段を一括で要求する安全側の過大要求である。要素数ベース）
pub fn detile_plane_16(
    src: &[u16],
    src_stride: usize,
    dst: &mut [u16],
    dst_stride: usize,
    size: ImageSize,
    tile_height: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(size.width, "DetilePlane_16", "width exceeds c_int range")?;
    require_c_int(size.height, "DetilePlane_16", "height exceeds c_int range")?;
    require_c_int(
        src_stride,
        "DetilePlane_16",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "DetilePlane_16",
        "destination stride exceeds c_int range",
    )?;
    require_c_int(
        tile_height,
        "DetilePlane_16",
        "tile_height exceeds c_int range",
    )?;

    // width == 0 || height == 0 は no-op で Ok を返す
    // （libyuv の DetilePlane_16 は -1 を返すが、このクレートではゼロサイズ入力を
    //  no-op に統一する方針）
    if size.width == 0 || size.height == 0 {
        return Ok(());
    }

    // tile_height は 2 の累乗でなければならない（libyuv 内部でビットマスクを使用するため）
    if !tile_height.is_power_of_two() {
        return Err(Error::with_reason(
            -1,
            "DetilePlane_16",
            "tile_height must be a power of two",
        ));
    }

    // stride >= 最小幅チェック
    // タイル配置は 16 要素単位で処理するため、src_stride は 16 の倍数に丸めた幅以上必要
    let src_min_stride = size.width.div_ceil(16).checked_mul(16).ok_or_else(|| {
        Error::with_reason(-1, "DetilePlane_16", "source minimum stride overflow")
    })?;
    if src_stride < src_min_stride {
        return Err(Error::with_reason(
            -1,
            "DetilePlane_16",
            "source stride smaller than round_up(width, 16)",
        ));
    }
    if dst_stride < size.width {
        return Err(Error::with_reason(
            -1,
            "DetilePlane_16",
            "destination stride smaller than width",
        ));
    }

    // バッファサイズ検証（オーバーフロー安全。要素数ベース）
    // src はタイル配置: src_stride * ceil(height / tile_height) * tile_height
    // （1 タイル行は 16 要素間隔のチャンク列のため、線形サイズより大きくなりうる）
    let tile_groups = size.height.div_ceil(tile_height);
    let src_size = src_stride
        .checked_mul(tile_groups)
        .and_then(|v| v.checked_mul(tile_height))
        .ok_or_else(|| Error::with_reason(-1, "DetilePlane_16", "source buffer size overflow"))?;
    if src.len() < src_size {
        return Err(Error::with_reason(
            -1,
            "DetilePlane_16",
            "source buffer too small",
        ));
    }
    // dst はリニア出力
    let dst_size = checked_buf_size(
        dst_stride,
        size.height,
        "DetilePlane_16",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_size {
        return Err(Error::with_reason(
            -1,
            "DetilePlane_16",
            "destination buffer too small",
        ));
    }

    // SAFETY: 上記の検証が全前提条件を検査済み。
    let result = unsafe {
        sys::DetilePlane_16(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
            tile_height as c_int,
        )
    };

    Error::check(result, "DetilePlane_16")
}

/// タイル化されたインターリーブ UV プレーンを分割してリニアに変換する
pub fn detile_split_uv_plane(
    src_uv: &[u8],
    src_stride_uv: usize,
    dst_u: &mut [u8],
    dst_stride_u: usize,
    dst_v: &mut [u8],
    dst_stride_v: usize,
    size: ImageSize,
    tile_height: usize,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        size.width,
        "DetileSplitUVPlane",
        "width exceeds c_int range",
    )?;
    require_c_int(
        size.height,
        "DetileSplitUVPlane",
        "height exceeds c_int range",
    )?;
    require_c_int(
        src_stride_uv,
        "DetileSplitUVPlane",
        "source UV stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_u,
        "DetileSplitUVPlane",
        "destination U stride exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_v,
        "DetileSplitUVPlane",
        "destination V stride exceeds c_int range",
    )?;
    require_c_int(
        tile_height,
        "DetileSplitUVPlane",
        "tile_height exceeds c_int range",
    )?;

    // width == 0 || height == 0 は C 実装の早期 return と同一セマンティクスで no-op
    if size.width == 0 || size.height == 0 {
        return Ok(());
    }

    // tile_height は 2 の累乗でなければならない（libyuv 内部でビットマスクを使用するため）
    if !tile_height.is_power_of_two() {
        return Err(Error::with_reason(
            -1,
            "DetileSplitUVPlane",
            "tile_height must be a power of two",
        ));
    }

    // stride >= 最小幅チェック
    // タイル配置は 16 バイト単位で処理するため、src_stride_uv は 16 の倍数に丸めた幅以上必要
    let src_min_stride = size.width.div_ceil(16) * 16;
    if src_stride_uv < src_min_stride {
        return Err(Error::with_reason(
            -1,
            "DetileSplitUVPlane",
            "source UV stride smaller than round_up(width, 16)",
        ));
    }
    // U/V プレーン行は画像幅の半分（(width + 1) / 2）
    let uv_plane_width = size.width.div_ceil(2);
    if dst_stride_u < uv_plane_width {
        return Err(Error::with_reason(
            -1,
            "DetileSplitUVPlane",
            "destination U stride smaller than (width + 1) / 2",
        ));
    }
    if dst_stride_v < uv_plane_width {
        return Err(Error::with_reason(
            -1,
            "DetileSplitUVPlane",
            "destination V stride smaller than (width + 1) / 2",
        ));
    }

    // バッファサイズ検証
    // src_uv はタイル配置: src_stride_uv * ceil(height / tile_height) * tile_height
    let tile_groups = size.height.div_ceil(tile_height);
    let src_size = src_stride_uv
        .checked_mul(tile_groups)
        .and_then(|v| v.checked_mul(tile_height))
        .ok_or_else(|| {
            Error::with_reason(-1, "DetileSplitUVPlane", "source UV buffer size overflow")
        })?;
    if src_uv.len() < src_size {
        return Err(Error::with_reason(
            -1,
            "DetileSplitUVPlane",
            "source UV buffer too small",
        ));
    }
    // dst_u / dst_v はリニア出力
    let dst_u_size = checked_buf_size(
        dst_stride_u,
        size.height,
        "DetileSplitUVPlane",
        "destination U buffer size overflow",
    )?;
    if dst_u.len() < dst_u_size {
        return Err(Error::with_reason(
            -1,
            "DetileSplitUVPlane",
            "destination U buffer too small",
        ));
    }
    let dst_v_size = checked_buf_size(
        dst_stride_v,
        size.height,
        "DetileSplitUVPlane",
        "destination V buffer size overflow",
    )?;
    if dst_v.len() < dst_v_size {
        return Err(Error::with_reason(
            -1,
            "DetileSplitUVPlane",
            "destination V buffer too small",
        ));
    }

    // SAFETY: .validate() が全前提条件を検査済み。
    unsafe {
        sys::DetileSplitUVPlane(
            src_uv.as_ptr(),
            src_stride_uv as c_int,
            dst_u.as_mut_ptr(),
            dst_stride_u as c_int,
            dst_v.as_mut_ptr(),
            dst_stride_v as c_int,
            size.width as c_int,
            size.height as c_int,
            tile_height as c_int,
        )
    };

    Ok(())
}

/// タイル化された Y と UV プレーンから YUY2 に変換する
///
/// `tile_height` は 2 以上かつ 2 の累乗でなければならない（libyuv 内部でビットマスクを
/// 使用するため。1 は 2 の累乗だが、UV のタイル高 `tile_height / 2` が 0 になり検証を
/// すり抜けるため不許可）。`src` の Y / UV はタイル配置である必要があり、各ストライドは
/// 幅を 16 の倍数に切り上げた値（round_up(width, 16)）以上でなければならない。
/// ソースの必要サイズはタイル配置のスパンで検証する（Y は
/// `y_stride * ceil(height / tile_height) * tile_height`、UV はタイル高が半分の
/// `uv_stride * ceil(height / tile_height) * (tile_height / 2)`。libyuv の DetileToYUY2 は
/// 1 タイル行を 16 バイトチャンクの列として読み、タイル高ごとに Y は
/// `y_stride * tile_height`、UV は `uv_stride * (tile_height / 2)` で次のタイル行へ
/// ジャンプする。planar_functions.cc を参照。全タイル段を一括で要求する安全側の
/// 過大要求である）
pub fn detile_to_yuy2(
    src: &Nv12Image<'_>,
    dst: &mut Yuy2ImageMut<'_>,
    size: ImageSize,
    tile_height: usize,
) -> Result<(), Error> {
    // width == 0 || height == 0 は C 実装（DetileToYUY2 は width <= 0 で return する）
    // と同一セマンティクスで no-op。ゼロサイズ入力は tile_height の検証を含め
    // すべての検証を省略して Ok を返す
    if size.width == 0 || size.height == 0 {
        return Ok(());
    }

    // c_int 範囲チェック
    require_c_int(
        tile_height,
        "DetileToYUY2",
        "tile_height exceeds c_int range",
    )?;

    // tile_height は 2 以上かつ 2 の累乗でなければならない
    if !tile_height.is_power_of_two() {
        return Err(Error::with_reason(
            -1,
            "DetileToYUY2",
            "tile_height must be a power of two",
        ));
    }
    if tile_height < 2 {
        return Err(Error::with_reason(
            -1,
            "DetileToYUY2",
            "tile_height must be at least 2",
        ));
    }

    src.validate(size, "DetileToYUY2")?;
    dst.validate(size, "DetileToYUY2")?;

    // タイル配置は 16 バイト単位で処理するため、各ストライドは 16 の倍数に丸めた幅以上必要
    let src_min_stride =
        size.width.div_ceil(16).checked_mul(16).ok_or_else(|| {
            Error::with_reason(-1, "DetileToYUY2", "source minimum stride overflow")
        })?;
    if src.y_stride < src_min_stride {
        return Err(Error::with_reason(
            -1,
            "DetileToYUY2",
            "Y stride smaller than round_up(width, 16)",
        ));
    }
    if src.uv_stride < src_min_stride {
        return Err(Error::with_reason(
            -1,
            "DetileToYUY2",
            "UV stride smaller than round_up(width, 16)",
        ));
    }

    // バッファサイズ検証（オーバーフロー安全）
    // Y はタイル配置: y_stride * ceil(height / tile_height) * tile_height
    let tile_groups = size.height.div_ceil(tile_height);
    let y_size = src
        .y_stride
        .checked_mul(tile_groups)
        .and_then(|v| v.checked_mul(tile_height))
        .ok_or_else(|| Error::with_reason(-1, "DetileToYUY2", "source Y buffer size overflow"))?;
    if src.y.len() < y_size {
        return Err(Error::with_reason(
            -1,
            "DetileToYUY2",
            "source Y buffer too small",
        ));
    }
    // UV はタイル配置（タイル高は tile_height / 2）:
    // uv_stride * ceil(height / tile_height) * (tile_height / 2)
    let uv_size = src
        .uv_stride
        .checked_mul(tile_groups)
        .and_then(|v| v.checked_mul(tile_height / 2))
        .ok_or_else(|| Error::with_reason(-1, "DetileToYUY2", "source UV buffer size overflow"))?;
    if src.uv.len() < uv_size {
        return Err(Error::with_reason(
            -1,
            "DetileToYUY2",
            "source UV buffer too small",
        ));
    }

    // SAFETY: .validate() と上記の検証が全前提条件を検査済み。
    unsafe {
        sys::DetileToYUY2(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.uv.as_ptr(),
            src.uv_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
            tile_height as c_int,
        )
    };

    Ok(())
}
