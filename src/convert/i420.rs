//! フォーマット変換関数 (i420)

use std::ffi::c_int;

use crate::{
    Ab30ImageMut, AbgrImage, AbgrImageMut, Ar30ImageMut, ArgbImage, ArgbImageMut, Error,
    I010ImageMut, I400Image, I400ImageMut, I420Image, I420ImageMut, I422Image, I422ImageMut,
    I444Image, I444ImageMut, ImageSize, Nv12Image, Nv12ImageMut, Nv21Image, Nv21ImageMut,
    Rgb24Image, Rgb24ImageMut, require_c_int, sys,
};

// ============================================================
// I420 <-> ARGB
// ============================================================

/// I420 から ARGB への変換
pub fn i420_to_argb(
    src: &I420Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToARGB")?;
    dst.validate(size, "I420ToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToARGB(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I420ToARGB")
}

/// ARGB から I420 への変換
pub fn argb_to_i420(
    src: &ArgbImage<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToI420")?;
    dst.validate(size, "ARGBToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::ARGBToI420(
            src.data.as_ptr(),
            src.stride as c_int,
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

    Error::check(result, "ARGBToI420")
}

// ============================================================
// I420 <-> ABGR
// ============================================================

/// I420 から ABGR への変換
pub fn i420_to_abgr(
    src: &I420Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToABGR")?;
    dst.validate(size, "I420ToABGR")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToABGR(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I420ToABGR")
}

/// ABGR から I420 への変換
pub fn abgr_to_i420(
    src: &AbgrImage<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ABGRToI420")?;
    dst.validate(size, "ABGRToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::ABGRToI420(
            src.data.as_ptr(),
            src.stride as c_int,
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

    Error::check(result, "ABGRToI420")
}

// ============================================================
// I420 <-> RGB24
// ============================================================

/// RGB24 から I420 への変換
pub fn rgb24_to_i420(
    src: &Rgb24Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGB24ToI420")?;
    dst.validate(size, "RGB24ToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::RGB24ToI420(
            src.data.as_ptr(),
            src.stride as c_int,
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

    Error::check(result, "RGB24ToI420")
}

/// I420 から RGB24 への変換
pub fn i420_to_rgb24(
    src: &I420Image<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToRGB24")?;
    dst.validate(size, "I420ToRGB24")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToRGB24(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I420ToRGB24")
}

// ============================================================
// I420 <-> NV12
// ============================================================

/// NV12 から I420 への変換
pub fn nv12_to_i420(
    src: &Nv12Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12ToI420")?;
    dst.validate(size, "NV12ToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV12ToI420(
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

    Error::check(result, "NV12ToI420")
}

/// I420 から NV12 への変換
pub fn i420_to_nv12(
    src: &I420Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToNV12")?;
    dst.validate(size, "I420ToNV12")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToNV12(
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

    Error::check(result, "I420ToNV12")
}

// ============================================================
// I420 <-> NV21
// ============================================================

/// NV21 から I420 への変換
pub fn nv21_to_i420(
    src: &Nv21Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV21ToI420")?;
    dst.validate(size, "NV21ToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::NV21ToI420(
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

    Error::check(result, "NV21ToI420")
}

/// I420 から NV21 への変換
pub fn i420_to_nv21(
    src: &I420Image<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToNV21")?;
    dst.validate(size, "I420ToNV21")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToNV21(
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

    Error::check(result, "I420ToNV21")
}

// ============================================================
// I420 <-> I422
// ============================================================

/// I422 から I420 への変換
pub fn i422_to_i420(
    src: &I422Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToI420")?;
    dst.validate(size, "I422ToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I422ToI420(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
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

    Error::check(result, "I422ToI420")
}

/// I420 から I422 への変換
pub fn i420_to_i422(
    src: &I420Image<'_>,
    dst: &mut I422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToI422")?;
    dst.validate(size, "I420ToI422")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToI422(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
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

    Error::check(result, "I420ToI422")
}

// ============================================================
// I420 <-> I444
// ============================================================

/// I444 から I420 への変換
pub fn i444_to_i420(
    src: &I444Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I444ToI420")?;
    dst.validate(size, "I444ToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I444ToI420(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
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

    Error::check(result, "I444ToI420")
}

/// I420 から I444 への変換
pub fn i420_to_i444(
    src: &I420Image<'_>,
    dst: &mut I444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToI444")?;
    dst.validate(size, "I420ToI444")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToI444(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
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

    Error::check(result, "I420ToI444")
}

// ============================================================
// I420 コピー
// ============================================================

/// I420 画像のコピー
pub fn i420_copy(
    src: &I420Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420Copy")?;
    dst.validate(size, "I420Copy")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420Copy(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
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

    Error::check(result, "I420Copy")
}

// ============================================================
// I420 <-> ARGB (I422/I444 経由)
// ============================================================

/// I422 から ARGB への変換
pub fn i422_to_argb(
    src: &I422Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToARGB")?;
    dst.validate(size, "I422ToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I422ToARGB(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I422ToARGB")
}

/// I444 から ARGB への変換
pub fn i444_to_argb(
    src: &I444Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I444ToARGB")?;
    dst.validate(size, "I444ToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I444ToARGB(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I444ToARGB")
}

/// I422 から ABGR への変換
pub fn i422_to_abgr(
    src: &I422Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToABGR")?;
    dst.validate(size, "I422ToABGR")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I422ToABGR(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I422ToABGR")
}

/// I444 から ABGR への変換
pub fn i444_to_abgr(
    src: &I444Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I444ToABGR")?;
    dst.validate(size, "I444ToABGR")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I444ToABGR(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I444ToABGR")
}

// ============================================================
// I420 -> 10bit 変換
// ============================================================

/// I420 (8bit) から I010 (10bit I420) への変換
pub fn i420_to_i010(
    src: &I420Image<'_>,
    dst: &mut I010ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToI010")?;
    dst.validate(size, "I420ToI010")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToI010(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
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

    Error::check(result, "I420ToI010")
}

/// I420 (8bit) から AR30 への変換
pub fn i420_to_ar30(
    src: &I420Image<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToAR30")?;
    dst.validate(size, "I420ToAR30")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToAR30(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I420ToAR30")
}

/// I420 (8bit) から AB30 への変換
pub fn i420_to_ab30(
    src: &I420Image<'_>,
    dst: &mut Ab30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToAB30")?;
    dst.validate(size, "I420ToAB30")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToAB30(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I420ToAB30")
}

// ============================================================
// I400 (グレースケール) 変換
// ============================================================

/// I400 (グレースケール) から ARGB への変換
pub fn i400_to_argb(
    src: &I400Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I400ToARGB")?;
    dst.validate(size, "I400ToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I400ToARGB(
            src.y.as_ptr(),
            src.y_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I400ToARGB")
}

/// I400 (グレースケール) から I400 へのコピー
pub fn i400_to_i400(
    src: &I400Image<'_>,
    dst: &mut I400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I400ToI400")?;
    dst.validate(size, "I400ToI400")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I400ToI400(
            src.y.as_ptr(),
            src.y_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I400ToI400")
}

/// I400 (グレースケール) から I420 への変換
///
/// U/V プレーンは 128 で埋められる。
pub fn i400_to_i420(
    src: &I400Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I400ToI420")?;
    dst.validate(size, "I400ToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I400ToI420(
            src.y.as_ptr(),
            src.y_stride as c_int,
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

    Error::check(result, "I400ToI420")
}

/// I400 (グレースケール) から NV21 への変換
///
/// VU プレーンは 128 で埋められる。
pub fn i400_to_nv21(
    src: &I400Image<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I400ToNV21")?;
    dst.validate(size, "I400ToNV21")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I400ToNV21(
            src.y.as_ptr(),
            src.y_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I400ToNV21")
}

/// I420 から I400 (グレースケール) への変換
///
/// Y プレーンのみをコピーし、U/V プレーンは破棄される。
pub fn i420_to_i400(
    src: &I420Image<'_>,
    dst: &mut I400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToI400")?;
    dst.validate(size, "I420ToI400")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420ToI400(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I420ToI400")
}

/// ARGB から I400 (グレースケール) への変換
pub fn argb_to_i400(
    src: &ArgbImage<'_>,
    dst: &mut I400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToI400")?;
    dst.validate(size, "ARGBToI400")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::ARGBToI400(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToI400")
}

// ============================================================
// アルファチャンネル付き変換
// ============================================================

/// アルファプレーン（入力）のバッファサイズを検証する
///
/// stride の c_int 範囲と stride >= width も検証する。height == 0 のサイズ計算
/// （stride * (height - 1)）はデバッグビルドでアンダーフローパニック、リリース
/// ビルドでは wrap して検証をすり抜けうる（既知の制約）
fn validate_alpha_src(
    src_a: &[u8],
    src_stride_a: usize,
    size: ImageSize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック。巨大な stride が負値に切り詰められると C 側の行ポインタ
    // 加算（src_a += src_stride_a）がアルファプレーンのみ逆進し、スライス先頭より
    // 手前の領域外読み出しになる
    require_c_int(src_stride_a, function, "alpha stride exceeds c_int range")?;
    // stride >= width チェック（i420_blend と同じ仕様。安全性より検証パターンの統一が
    // 目的。C は stride < width でも行が重なるだけで OOB にはならない）
    if src_stride_a < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "alpha stride smaller than width",
        ));
    }
    let required = src_stride_a * (size.height - 1) + size.width;
    if src_a.len() < required {
        return Err(Error::with_reason(
            -1,
            function,
            "source alpha buffer too small",
        ));
    }
    Ok(())
}

/// アルファプレーン（出力）のバッファサイズを検証する
///
/// stride の c_int 範囲と stride >= width も検証する。height == 0 のサイズ計算
/// （stride * (height - 1)）はデバッグビルドでアンダーフローパニック、リリース
/// ビルドでは wrap して検証をすり抜けうる（既知の制約）
fn validate_alpha_dst(
    dst_a: &[u8],
    dst_stride_a: usize,
    size: ImageSize,
    function: &'static str,
) -> Result<(), Error> {
    // c_int 範囲チェック。巨大な stride が負値に切り詰められると C 側の行ポインタ
    // 加算（dst_a += dst_stride_a * 2）がアルファプレーンのみ逆進し、領域外書き込み
    // （メモリ破壊）になる
    require_c_int(dst_stride_a, function, "alpha stride exceeds c_int range")?;
    // stride >= width チェック（i420_blend と同じ仕様。安全性より検証パターンの統一が
    // 目的。C は stride < width でも行が重なるだけで OOB にはならない）
    if dst_stride_a < size.width {
        return Err(Error::with_reason(
            -1,
            function,
            "alpha stride smaller than width",
        ));
    }
    let required = dst_stride_a * (size.height - 1) + size.width;
    if dst_a.len() < required {
        return Err(Error::with_reason(
            -1,
            function,
            "destination alpha buffer too small",
        ));
    }
    Ok(())
}

/// I420 + アルファから ARGB への変換
pub fn i420_alpha_to_argb(
    src: &I420Image<'_>,
    src_a: &[u8],
    src_stride_a: usize,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    attenuate: bool,
) -> Result<(), Error> {
    src.validate(size, "I420AlphaToARGB")?;
    validate_alpha_src(src_a, src_stride_a, size, "I420AlphaToARGB")?;
    dst.validate(size, "I420AlphaToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420AlphaToARGB(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_a.as_ptr(),
            src_stride_a as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
            attenuate as c_int,
        )
    };

    Error::check(result, "I420AlphaToARGB")
}

/// I420 + アルファから ABGR への変換
pub fn i420_alpha_to_abgr(
    src: &I420Image<'_>,
    src_a: &[u8],
    src_stride_a: usize,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
    attenuate: bool,
) -> Result<(), Error> {
    src.validate(size, "I420AlphaToABGR")?;
    validate_alpha_src(src_a, src_stride_a, size, "I420AlphaToABGR")?;
    dst.validate(size, "I420AlphaToABGR")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I420AlphaToABGR(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_a.as_ptr(),
            src_stride_a as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
            attenuate as c_int,
        )
    };

    Error::check(result, "I420AlphaToABGR")
}

/// I422 + アルファから ARGB への変換
pub fn i422_alpha_to_argb(
    src: &I422Image<'_>,
    src_a: &[u8],
    src_stride_a: usize,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    attenuate: bool,
) -> Result<(), Error> {
    src.validate(size, "I422AlphaToARGB")?;
    validate_alpha_src(src_a, src_stride_a, size, "I422AlphaToARGB")?;
    dst.validate(size, "I422AlphaToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I422AlphaToARGB(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_a.as_ptr(),
            src_stride_a as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
            attenuate as c_int,
        )
    };

    Error::check(result, "I422AlphaToARGB")
}

/// I422 + アルファから ABGR への変換
pub fn i422_alpha_to_abgr(
    src: &I422Image<'_>,
    src_a: &[u8],
    src_stride_a: usize,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
    attenuate: bool,
) -> Result<(), Error> {
    src.validate(size, "I422AlphaToABGR")?;
    validate_alpha_src(src_a, src_stride_a, size, "I422AlphaToABGR")?;
    dst.validate(size, "I422AlphaToABGR")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I422AlphaToABGR(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_a.as_ptr(),
            src_stride_a as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
            attenuate as c_int,
        )
    };

    Error::check(result, "I422AlphaToABGR")
}

/// I444 + アルファから ARGB への変換
pub fn i444_alpha_to_argb(
    src: &I444Image<'_>,
    src_a: &[u8],
    src_stride_a: usize,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    attenuate: bool,
) -> Result<(), Error> {
    src.validate(size, "I444AlphaToARGB")?;
    validate_alpha_src(src_a, src_stride_a, size, "I444AlphaToARGB")?;
    dst.validate(size, "I444AlphaToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I444AlphaToARGB(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_a.as_ptr(),
            src_stride_a as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
            attenuate as c_int,
        )
    };

    Error::check(result, "I444AlphaToARGB")
}

/// I444 + アルファから ABGR への変換
pub fn i444_alpha_to_abgr(
    src: &I444Image<'_>,
    src_a: &[u8],
    src_stride_a: usize,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
    attenuate: bool,
) -> Result<(), Error> {
    src.validate(size, "I444AlphaToABGR")?;
    validate_alpha_src(src_a, src_stride_a, size, "I444AlphaToABGR")?;
    dst.validate(size, "I444AlphaToABGR")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::I444AlphaToABGR(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_a.as_ptr(),
            src_stride_a as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
            attenuate as c_int,
        )
    };

    Error::check(result, "I444AlphaToABGR")
}

/// ARGB から I420 + アルファへの変換
pub fn argb_to_i420_alpha(
    src: &ArgbImage<'_>,
    dst: &mut I420ImageMut<'_>,
    dst_a: &mut [u8],
    dst_stride_a: usize,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToI420Alpha")?;
    dst.validate(size, "ARGBToI420Alpha")?;
    validate_alpha_dst(dst_a, dst_stride_a, size, "ARGBToI420Alpha")?;

    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::ARGBToI420Alpha(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_a.as_mut_ptr(),
            dst_stride_a as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToI420Alpha")
}
