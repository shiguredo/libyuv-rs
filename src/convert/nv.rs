//! フォーマット変換関数 (nv)

use std::ffi::c_int;

use crate::{
    AbgrImageMut, ArgbImage, ArgbImageMut, Error, I422Image, I444Image, ImageSize, Nv12Image,
    Nv12ImageMut, Nv16Image, Nv21Image, Nv21ImageMut, Nv24ImageMut, RawImageMut, Rgb24ImageMut,
    Rgb565ImageMut, Yuv24ImageMut, sys,
};

// ============================================================
// NV12/NV21 <-> ARGB
// ============================================================

/// NV12 から ARGB への変換
pub fn nv12_to_argb(
    src: &Nv12Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12ToARGB")?;
    dst.validate(size, "NV12ToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV12ToARGB(
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

    Error::check(result, "NV12ToARGB")
}

/// NV21 から ARGB への変換
pub fn nv21_to_argb(
    src: &Nv21Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV21ToARGB")?;
    dst.validate(size, "NV21ToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV21ToARGB(
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

    Error::check(result, "NV21ToARGB")
}

/// NV12 から ABGR への変換
pub fn nv12_to_abgr(
    src: &Nv12Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12ToABGR")?;
    dst.validate(size, "NV12ToABGR")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV12ToABGR(
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

    Error::check(result, "NV12ToABGR")
}

/// NV21 から ABGR への変換
pub fn nv21_to_abgr(
    src: &Nv21Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV21ToABGR")?;
    dst.validate(size, "NV21ToABGR")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV21ToABGR(
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

    Error::check(result, "NV21ToABGR")
}

// ============================================================
// ARGB <-> NV12/NV21
// ============================================================

/// ARGB から NV12 への変換
pub fn argb_to_nv12(
    src: &ArgbImage<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToNV12")?;
    dst.validate(size, "ARGBToNV12")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::ARGBToNV12(
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

    Error::check(result, "ARGBToNV12")
}

/// ARGB から NV21 への変換
pub fn argb_to_nv21(
    src: &ArgbImage<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToNV21")?;
    dst.validate(size, "ARGBToNV21")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::ARGBToNV21(
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

    Error::check(result, "ARGBToNV21")
}

// ============================================================
// NV12 <-> RGB24
// ============================================================

/// NV12 から RGB24 への変換
pub fn nv12_to_rgb24(
    src: &Nv12Image<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12ToRGB24")?;
    dst.validate(size, "NV12ToRGB24")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV12ToRGB24(
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

    Error::check(result, "NV12ToRGB24")
}

/// NV21 から RGB24 への変換
pub fn nv21_to_rgb24(
    src: &Nv21Image<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV21ToRGB24")?;
    dst.validate(size, "NV21ToRGB24")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV21ToRGB24(
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

    Error::check(result, "NV21ToRGB24")
}

// ============================================================
// I444 <-> NV12
// ============================================================

/// I444 から NV12 への変換
pub fn i444_to_nv12(
    src: &I444Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I444ToNV12")?;
    dst.validate(size, "I444ToNV12")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I444ToNV12(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I444ToNV12")
}

/// I444 から NV21 への変換
pub fn i444_to_nv21(
    src: &I444Image<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I444ToNV21")?;
    dst.validate(size, "I444ToNV21")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I444ToNV21(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I444ToNV21")
}

// ============================================================
// I422 <-> NV21
// ============================================================

/// I422 から NV21 への変換
pub fn i422_to_nv21(
    src: &I422Image<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToNV21")?;
    dst.validate(size, "I422ToNV21")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I422ToNV21(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I422ToNV21")
}

// ============================================================
// NV12 コピー
// ============================================================

/// NV12 画像のコピー
pub fn nv12_copy(
    src: &Nv12Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12Copy")?;
    dst.validate(size, "NV12Copy")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV12Copy(
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

    Error::check(result, "NV12Copy")
}

/// NV21 から NV12 への変換
pub fn nv21_to_nv12(
    src: &Nv21Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV21ToNV12")?;
    dst.validate(size, "NV21ToNV12")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV21ToNV12(
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

    Error::check(result, "NV21ToNV12")
}

// ============================================================
// 追加コピー
// ============================================================

/// NV21 画像のコピー
pub fn nv21_copy(
    src: &Nv21Image<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV21Copy")?;
    dst.validate(size, "NV21Copy")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV21Copy(
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

    Error::check(result, "NV21Copy")
}

// ============================================================
// 追加 NV12/NV21 変換
// ============================================================

/// NV12 から RAW への変換
pub fn nv12_to_raw(
    src: &Nv12Image<'_>,
    dst: &mut RawImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12ToRAW")?;
    dst.validate(size, "NV12ToRAW")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV12ToRAW(
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

    Error::check(result, "NV12ToRAW")
}

/// NV12 から RGB565 への変換
pub fn nv12_to_rgb565(
    src: &Nv12Image<'_>,
    dst: &mut Rgb565ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12ToRGB565")?;
    dst.validate(size, "NV12ToRGB565")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV12ToRGB565(
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

    Error::check(result, "NV12ToRGB565")
}

/// NV12 から NV24 への変換
///
/// NV24 は 4:4:4 サブサンプリングのため、chroma の高さが luma と同じになる。
/// `Nv24ImageMut::validate` は chroma を height で検証する。
pub fn nv12_to_nv24(
    src: &Nv12Image<'_>,
    dst: &mut Nv24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12ToNV24")?;
    dst.validate(size, "NV12ToNV24")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV12ToNV24(
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

    Error::check(result, "NV12ToNV24")
}

/// NV16 から NV24 への変換
///
/// NV16 は 4:2:2 サブサンプリングのため、chroma の高さが luma と同じになる。
/// NV24 は 4:4:4 サブサンプリングのため、chroma の高さが luma と同じになる。
/// `Nv16Image::validate` は chroma を height で検証し、`Nv24ImageMut::validate` も chroma を height で検証する。
pub fn nv16_to_nv24(
    src: &Nv16Image<'_>,
    dst: &mut Nv24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV16ToNV24")?;
    dst.validate(size, "NV16ToNV24")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV16ToNV24(
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

    Error::check(result, "NV16ToNV24")
}

/// NV21 から RAW への変換
pub fn nv21_to_raw(
    src: &Nv21Image<'_>,
    dst: &mut RawImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV21ToRAW")?;
    dst.validate(size, "NV21ToRAW")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV21ToRAW(
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

    Error::check(result, "NV21ToRAW")
}

/// NV21 から YUV24 への変換
pub fn nv21_to_yuv24(
    src: &Nv21Image<'_>,
    dst: &mut Yuv24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV21ToYUV24")?;
    dst.validate(size, "NV21ToYUV24")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV21ToYUV24(
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

    Error::check(result, "NV21ToYUV24")
}
