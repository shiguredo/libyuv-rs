//! フォーマット変換関数
#![allow(clippy::too_many_arguments)]

use std::ffi::c_int;

use crate::{
    Ab30ImageMut, Ab64Image, Ab64ImageMut, AbgrImage, AbgrImageMut, Android420Image, Ar30Image,
    Ar30ImageMut, Ar64Image, Ar64ImageMut, Argb1555Image, Argb1555ImageMut, Argb4444Image,
    Argb4444ImageMut, ArgbImage, ArgbImageMut, AyuvImage, BgraImage, BgraImageMut, Error,
    H010Image, H210Image, H420Image, H422Image, H444Image, I010Image, I010ImageMut, I012Image,
    I012ImageMut, I210Image, I210ImageMut, I212Image, I400Image, I400ImageMut, I410Image,
    I410ImageMut, I412Image, I420Image, I420ImageMut, I422Image, I422ImageMut, I444Image,
    I444ImageMut, ImageSize, J400Image, J400ImageMut, J420Image, J420ImageMut, J422Image,
    J422ImageMut, J444Image, J444ImageMut, Mm21Image, Mt2tImage, Nv12Image, Nv12ImageMut,
    Nv16Image, Nv21Image, Nv21ImageMut, Nv24ImageMut, P010Image, P010ImageMut, P012Image,
    P012ImageMut, P210Image, P210ImageMut, P212ImageMut, P410ImageMut, RawImage, RawImageMut,
    Rgb24Image, Rgb24ImageMut, Rgb565Image, Rgb565ImageMut, RgbaImage, RgbaImageMut, U010Image,
    U210Image, U420Image, U422Image, U444Image, UyvyImage, UyvyImageMut, Yuv24ImageMut, Yuy2Image,
    Yuy2ImageMut, sys,
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
// I422 <-> I444
// ============================================================

/// I422 から I444 への変換
pub fn i422_to_i444(
    src: &I422Image<'_>,
    dst: &mut I444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToI444")?;
    dst.validate(size, "I422ToI444")?;

    let result = unsafe {
        sys::I422ToI444(
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

    Error::check(result, "I422ToI444")
}

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
// ARGB コピー・チャンネル変換
// ============================================================

/// ARGB 画像のコピー
pub fn argb_copy(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBCopy")?;
    dst.validate(size, "ARGBCopy")?;

    let result = unsafe {
        sys::ARGBCopy(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBCopy")
}

/// ARGB から ABGR への変換
pub fn argb_to_abgr(
    src: &ArgbImage<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToABGR")?;
    dst.validate(size, "ARGBToABGR")?;

    let result = unsafe {
        sys::ARGBToABGR(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToABGR")
}

/// ABGR から ARGB への変換
pub fn abgr_to_argb(
    src: &AbgrImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ABGRToARGB")?;
    dst.validate(size, "ABGRToARGB")?;

    let result = unsafe {
        sys::ABGRToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ABGRToARGB")
}

/// ARGB から RGB24 への変換
pub fn argb_to_rgb24(
    src: &ArgbImage<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToRGB24")?;
    dst.validate(size, "ARGBToRGB24")?;

    let result = unsafe {
        sys::ARGBToRGB24(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToRGB24")
}

/// RGB24 から ARGB への変換
pub fn rgb24_to_argb(
    src: &Rgb24Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGB24ToARGB")?;
    dst.validate(size, "RGB24ToARGB")?;

    let result = unsafe {
        sys::RGB24ToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RGB24ToARGB")
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
// ARGB から YUV への変換
// ============================================================

/// ARGB から I422 への変換
pub fn argb_to_i422(
    src: &ArgbImage<'_>,
    dst: &mut I422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToI422")?;
    dst.validate(size, "ARGBToI422")?;

    let result = unsafe {
        sys::ARGBToI422(
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

    Error::check(result, "ARGBToI422")
}

/// ARGB から I444 への変換
pub fn argb_to_i444(
    src: &ArgbImage<'_>,
    dst: &mut I444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToI444")?;
    dst.validate(size, "ARGBToI444")?;

    let result = unsafe {
        sys::ARGBToI444(
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

    Error::check(result, "ARGBToI444")
}

// ============================================================
// I444 <-> RGB24
// ============================================================

/// I444 から RGB24 への変換
pub fn i444_to_rgb24(
    src: &I444Image<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I444ToRGB24")?;
    dst.validate(size, "I444ToRGB24")?;

    let result = unsafe {
        sys::I444ToRGB24(
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

    Error::check(result, "I444ToRGB24")
}

// ============================================================
// I422 <-> RGB24
// ============================================================

/// I422 から RGB24 への変換
pub fn i422_to_rgb24(
    src: &I422Image<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToRGB24")?;
    dst.validate(size, "I422ToRGB24")?;

    let result = unsafe {
        sys::I422ToRGB24(
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

    Error::check(result, "I422ToRGB24")
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
// AR30 (10bit packed RGB)
// ============================================================

/// AR30 から ARGB への変換
pub fn ar30_to_argb(
    src: &Ar30Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "AR30ToARGB")?;
    dst.validate(size, "AR30ToARGB")?;

    let result = unsafe {
        sys::AR30ToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "AR30ToARGB")
}

/// AR30 から ABGR への変換
pub fn ar30_to_abgr(
    src: &Ar30Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "AR30ToABGR")?;
    dst.validate(size, "AR30ToABGR")?;

    let result = unsafe {
        sys::AR30ToABGR(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "AR30ToABGR")
}

/// AR30 から AB30 への変換
pub fn ar30_to_ab30(
    src: &Ar30Image<'_>,
    dst: &mut Ab30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "AR30ToAB30")?;
    dst.validate(size, "AR30ToAB30")?;

    let result = unsafe {
        sys::AR30ToAB30(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "AR30ToAB30")
}

/// ARGB から AR30 への変換
pub fn argb_to_ar30(
    src: &ArgbImage<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToAR30")?;
    dst.validate(size, "ARGBToAR30")?;

    let result = unsafe {
        sys::ARGBToAR30(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToAR30")
}

/// ABGR から AR30 への変換
pub fn abgr_to_ar30(
    src: &AbgrImage<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ABGRToAR30")?;
    dst.validate(size, "ABGRToAR30")?;

    let result = unsafe {
        sys::ABGRToAR30(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ABGRToAR30")
}

// ============================================================
// AR64 / AB64 (16bit per channel)
// ============================================================

/// AR64 から ARGB への変換
pub fn ar64_to_argb(
    src: &Ar64Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "AR64ToARGB")?;
    dst.validate(size, "AR64ToARGB")?;

    let result = unsafe {
        sys::AR64ToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "AR64ToARGB")
}

/// AR64 から AB64 への変換
pub fn ar64_to_ab64(
    src: &Ar64Image<'_>,
    dst: &mut Ab64ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "AR64ToAB64")?;
    dst.validate(size, "AR64ToAB64")?;

    let result = unsafe {
        sys::AR64ToAB64(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "AR64ToAB64")
}

/// ARGB から AR64 への変換
pub fn argb_to_ar64(
    src: &ArgbImage<'_>,
    dst: &mut Ar64ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToAR64")?;
    dst.validate(size, "ARGBToAR64")?;

    let result = unsafe {
        sys::ARGBToAR64(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToAR64")
}

/// AB64 から ARGB への変換
pub fn ab64_to_argb(
    src: &Ab64Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "AB64ToARGB")?;
    dst.validate(size, "AB64ToARGB")?;

    let result = unsafe {
        sys::AB64ToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "AB64ToARGB")
}

/// ARGB から AB64 への変換
pub fn argb_to_ab64(
    src: &ArgbImage<'_>,
    dst: &mut Ab64ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToAB64")?;
    dst.validate(size, "ARGBToAB64")?;

    let result = unsafe {
        sys::ARGBToAB64(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToAB64")
}

/// AR64 チャンネルシャッフル
///
/// `shuffler` はチャンネルの並び替え順序を指定する 4 バイト配列
pub fn ar64_shuffle(
    src: &Ar64Image<'_>,
    dst: &mut Ar64ImageMut<'_>,
    size: ImageSize,
    shuffler: &[u8; 4],
) -> Result<(), Error> {
    src.validate(size, "AR64Shuffle")?;
    dst.validate(size, "AR64Shuffle")?;

    let result = unsafe {
        sys::AR64Shuffle(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            shuffler.as_ptr(),
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "AR64Shuffle")
}

// ============================================================
// 12bit 高ビット深度変換
// ============================================================

/// I012 (12bit I420) から I420 への変換
pub fn i012_to_i420(
    src: &I012Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I012ToI420")?;
    dst.validate(size, "I012ToI420")?;

    let result = unsafe {
        sys::I012ToI420(
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

    Error::check(result, "I012ToI420")
}

/// I012 (12bit I420) から P012 への変換
pub fn i012_to_p012(
    src: &I012Image<'_>,
    dst: &mut P012ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I012ToP012")?;
    dst.validate(size, "I012ToP012")?;

    let result = unsafe {
        sys::I012ToP012(
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

    Error::check(result, "I012ToP012")
}

/// I420 から I012 (12bit I420) への変換
pub fn i420_to_i012(
    src: &I420Image<'_>,
    dst: &mut I012ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToI012")?;
    dst.validate(size, "I420ToI012")?;

    let result = unsafe {
        sys::I420ToI012(
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

    Error::check(result, "I420ToI012")
}

/// I212 (12bit I422) から I420 への変換
pub fn i212_to_i420(
    src: &I212Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I212ToI420")?;
    dst.validate(size, "I212ToI420")?;

    let result = unsafe {
        sys::I212ToI420(
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

    Error::check(result, "I212ToI420")
}

/// I212 (12bit I422) から I422 への変換
pub fn i212_to_i422(
    src: &I212Image<'_>,
    dst: &mut I422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I212ToI422")?;
    dst.validate(size, "I212ToI422")?;

    let result = unsafe {
        sys::I212ToI422(
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

    Error::check(result, "I212ToI422")
}

/// I212 (12bit I422) から P212 への変換
pub fn i212_to_p212(
    src: &I212Image<'_>,
    dst: &mut P212ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I212ToP212")?;
    dst.validate(size, "I212ToP212")?;

    let result = unsafe {
        sys::I212ToP212(
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

    Error::check(result, "I212ToP212")
}

/// I412 (12bit I444) から I420 への変換
pub fn i412_to_i420(
    src: &I412Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I412ToI420")?;
    dst.validate(size, "I412ToI420")?;

    let result = unsafe {
        sys::I412ToI420(
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

    Error::check(result, "I412ToI420")
}

/// I412 (12bit I444) から I444 への変換
pub fn i412_to_i444(
    src: &I412Image<'_>,
    dst: &mut I444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I412ToI444")?;
    dst.validate(size, "I412ToI444")?;

    let result = unsafe {
        sys::I412ToI444(
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

    Error::check(result, "I412ToI444")
}

/// P012 から I012 (12bit I420) への変換
pub fn p012_to_i012(
    src: &P012Image<'_>,
    dst: &mut I012ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "P012ToI012")?;
    dst.validate(size, "P012ToI012")?;

    let result = unsafe {
        sys::P012ToI012(
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

    Error::check(result, "P012ToI012")
}
// ============================================================
// I010 (10bit I420) 変換
// ============================================================

/// I010 (10bit I420) から ARGB への変換
pub fn i010_to_argb(
    src: &I010Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I010ToARGB")?;
    dst.validate(size, "I010ToARGB")?;

    let result = unsafe {
        sys::I010ToARGB(
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

    Error::check(result, "I010ToARGB")
}

/// I010 (10bit I420) から ABGR への変換
pub fn i010_to_abgr(
    src: &I010Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I010ToABGR")?;
    dst.validate(size, "I010ToABGR")?;

    let result = unsafe {
        sys::I010ToABGR(
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

    Error::check(result, "I010ToABGR")
}

/// I010 (10bit I420) から AR30 への変換
pub fn i010_to_ar30(
    src: &I010Image<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I010ToAR30")?;
    dst.validate(size, "I010ToAR30")?;

    let result = unsafe {
        sys::I010ToAR30(
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

    Error::check(result, "I010ToAR30")
}

/// I010 (10bit I420) から AB30 への変換
pub fn i010_to_ab30(
    src: &I010Image<'_>,
    dst: &mut Ab30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I010ToAB30")?;
    dst.validate(size, "I010ToAB30")?;

    let result = unsafe {
        sys::I010ToAB30(
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

    Error::check(result, "I010ToAB30")
}

/// I010 (10bit I420) から I420 (8bit) への変換
pub fn i010_to_i420(
    src: &I010Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I010ToI420")?;
    dst.validate(size, "I010ToI420")?;

    let result = unsafe {
        sys::I010ToI420(
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

    Error::check(result, "I010ToI420")
}

/// I010 (10bit I420) から I410 (10bit I444) への変換
pub fn i010_to_i410(
    src: &I010Image<'_>,
    dst: &mut I410ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I010ToI410")?;
    dst.validate(size, "I010ToI410")?;

    let result = unsafe {
        sys::I010ToI410(
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

    Error::check(result, "I010ToI410")
}

/// I010 (10bit I420) から NV12 (8bit) への変換
pub fn i010_to_nv12(
    src: &I010Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I010ToNV12")?;
    dst.validate(size, "I010ToNV12")?;

    let result = unsafe {
        sys::I010ToNV12(
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

    Error::check(result, "I010ToNV12")
}

/// I010 (10bit I420) から P010 への変換
pub fn i010_to_p010(
    src: &I010Image<'_>,
    dst: &mut P010ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I010ToP010")?;
    dst.validate(size, "I010ToP010")?;

    let result = unsafe {
        sys::I010ToP010(
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

    Error::check(result, "I010ToP010")
}

/// I010 (10bit I420) 画像のコピー
pub fn i010_copy(
    src: &I010Image<'_>,
    dst: &mut I010ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I010Copy")?;
    dst.validate(size, "I010Copy")?;

    let result = unsafe {
        sys::I010Copy(
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

    Error::check(result, "I010Copy")
}

// ============================================================
// I210 (10bit I422) 変換
// ============================================================

/// I210 (10bit I422) から ARGB への変換
pub fn i210_to_argb(
    src: &I210Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210ToARGB")?;
    dst.validate(size, "I210ToARGB")?;

    let result = unsafe {
        sys::I210ToARGB(
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

    Error::check(result, "I210ToARGB")
}

/// I210 (10bit I422) から ABGR への変換
pub fn i210_to_abgr(
    src: &I210Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210ToABGR")?;
    dst.validate(size, "I210ToABGR")?;

    let result = unsafe {
        sys::I210ToABGR(
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

    Error::check(result, "I210ToABGR")
}

/// I210 (10bit I422) から AR30 への変換
pub fn i210_to_ar30(
    src: &I210Image<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210ToAR30")?;
    dst.validate(size, "I210ToAR30")?;

    let result = unsafe {
        sys::I210ToAR30(
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

    Error::check(result, "I210ToAR30")
}

/// I210 (10bit I422) から AB30 への変換
pub fn i210_to_ab30(
    src: &I210Image<'_>,
    dst: &mut Ab30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210ToAB30")?;
    dst.validate(size, "I210ToAB30")?;

    let result = unsafe {
        sys::I210ToAB30(
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

    Error::check(result, "I210ToAB30")
}

/// I210 (10bit I422) から I010 (10bit I420) への変換
pub fn i210_to_i010(
    src: &I210Image<'_>,
    dst: &mut I010ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210ToI010")?;
    dst.validate(size, "I210ToI010")?;

    let result = unsafe {
        sys::I210ToI010(
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

    Error::check(result, "I210ToI010")
}

/// I210 (10bit I422) から I410 (10bit I444) への変換
pub fn i210_to_i410(
    src: &I210Image<'_>,
    dst: &mut I410ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210ToI410")?;
    dst.validate(size, "I210ToI410")?;

    let result = unsafe {
        sys::I210ToI410(
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

    Error::check(result, "I210ToI410")
}

/// I210 (10bit I422) から I420 (8bit) への変換
pub fn i210_to_i420(
    src: &I210Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210ToI420")?;
    dst.validate(size, "I210ToI420")?;

    let result = unsafe {
        sys::I210ToI420(
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

    Error::check(result, "I210ToI420")
}

/// I210 (10bit I422) から I422 (8bit) への変換
pub fn i210_to_i422(
    src: &I210Image<'_>,
    dst: &mut I422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210ToI422")?;
    dst.validate(size, "I210ToI422")?;

    let result = unsafe {
        sys::I210ToI422(
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

    Error::check(result, "I210ToI422")
}

/// I422 (8bit) から I210 (10bit I422) への変換
pub fn i422_to_i210(
    src: &I422Image<'_>,
    dst: &mut I210ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToI210")?;
    dst.validate(size, "I422ToI210")?;

    let result = unsafe {
        sys::I422ToI210(
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

    Error::check(result, "I422ToI210")
}

/// I210 (10bit I422) から P210 への変換
pub fn i210_to_p210(
    src: &I210Image<'_>,
    dst: &mut P210ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210ToP210")?;
    dst.validate(size, "I210ToP210")?;

    let result = unsafe {
        sys::I210ToP210(
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

    Error::check(result, "I210ToP210")
}

/// I210 (10bit I422) 画像のコピー
pub fn i210_copy(
    src: &I210Image<'_>,
    dst: &mut I210ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I210Copy")?;
    dst.validate(size, "I210Copy")?;

    let result = unsafe {
        sys::I210Copy(
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

    Error::check(result, "I210Copy")
}

// ============================================================
// I410 (10bit I444) 変換
// ============================================================

/// I410 (10bit I444) から I010 (10bit I420) への変換
pub fn i410_to_i010(
    src: &I410Image<'_>,
    dst: &mut I010ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I410ToI010")?;
    dst.validate(size, "I410ToI010")?;

    let result = unsafe {
        sys::I410ToI010(
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

    Error::check(result, "I410ToI010")
}

/// I410 (10bit I444) から I420 (8bit) への変換
pub fn i410_to_i420(
    src: &I410Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I410ToI420")?;
    dst.validate(size, "I410ToI420")?;

    let result = unsafe {
        sys::I410ToI420(
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

    Error::check(result, "I410ToI420")
}

/// I410 (10bit I444) から I444 (8bit) への変換
pub fn i410_to_i444(
    src: &I410Image<'_>,
    dst: &mut I444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I410ToI444")?;
    dst.validate(size, "I410ToI444")?;

    let result = unsafe {
        sys::I410ToI444(
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

    Error::check(result, "I410ToI444")
}

/// I410 (10bit I444) 画像のコピー
pub fn i410_copy(
    src: &I410Image<'_>,
    dst: &mut I410ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I410Copy")?;
    dst.validate(size, "I410Copy")?;

    let result = unsafe {
        sys::I410Copy(
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

    Error::check(result, "I410Copy")
}

// ============================================================
// H 系 (BT.709) 10bit 変換
// ============================================================

/// H010 (BT.709 10bit I420) から ARGB への変換
pub fn h010_to_argb(
    src: &H010Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H010ToARGB")?;
    dst.validate(size, "H010ToARGB")?;

    let result = unsafe {
        sys::H010ToARGB(
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

    Error::check(result, "H010ToARGB")
}

/// H010 (BT.709 10bit I420) から ABGR への変換
pub fn h010_to_abgr(
    src: &H010Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H010ToABGR")?;
    dst.validate(size, "H010ToABGR")?;

    let result = unsafe {
        sys::H010ToABGR(
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

    Error::check(result, "H010ToABGR")
}

/// H010 (BT.709 10bit I420) から AR30 への変換
pub fn h010_to_ar30(
    src: &H010Image<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H010ToAR30")?;
    dst.validate(size, "H010ToAR30")?;

    let result = unsafe {
        sys::H010ToAR30(
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

    Error::check(result, "H010ToAR30")
}

/// H010 (BT.709 10bit I420) から AB30 への変換
pub fn h010_to_ab30(
    src: &H010Image<'_>,
    dst: &mut Ab30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H010ToAB30")?;
    dst.validate(size, "H010ToAB30")?;

    let result = unsafe {
        sys::H010ToAB30(
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

    Error::check(result, "H010ToAB30")
}

/// H210 (BT.709 10bit I422) から ARGB への変換
pub fn h210_to_argb(
    src: &H210Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H210ToARGB")?;
    dst.validate(size, "H210ToARGB")?;

    let result = unsafe {
        sys::H210ToARGB(
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

    Error::check(result, "H210ToARGB")
}

/// H210 (BT.709 10bit I422) から ABGR への変換
pub fn h210_to_abgr(
    src: &H210Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H210ToABGR")?;
    dst.validate(size, "H210ToABGR")?;

    let result = unsafe {
        sys::H210ToABGR(
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

    Error::check(result, "H210ToABGR")
}

/// H210 (BT.709 10bit I422) から AR30 への変換
pub fn h210_to_ar30(
    src: &H210Image<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H210ToAR30")?;
    dst.validate(size, "H210ToAR30")?;

    let result = unsafe {
        sys::H210ToAR30(
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

    Error::check(result, "H210ToAR30")
}

/// H210 (BT.709 10bit I422) から AB30 への変換
pub fn h210_to_ab30(
    src: &H210Image<'_>,
    dst: &mut Ab30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H210ToAB30")?;
    dst.validate(size, "H210ToAB30")?;

    let result = unsafe {
        sys::H210ToAB30(
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

    Error::check(result, "H210ToAB30")
}

// ============================================================
// H 系 (BT.709) 8bit 変換
// ============================================================

/// H420 (BT.709 I420) から ARGB への変換
pub fn h420_to_argb(
    src: &H420Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H420ToARGB")?;
    dst.validate(size, "H420ToARGB")?;

    let result = unsafe {
        sys::H420ToARGB(
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

    Error::check(result, "H420ToARGB")
}

/// H420 (BT.709 I420) から ABGR への変換
pub fn h420_to_abgr(
    src: &H420Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H420ToABGR")?;
    dst.validate(size, "H420ToABGR")?;

    let result = unsafe {
        sys::H420ToABGR(
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

    Error::check(result, "H420ToABGR")
}

/// H420 (BT.709 I420) から AR30 への変換
pub fn h420_to_ar30(
    src: &H420Image<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H420ToAR30")?;
    dst.validate(size, "H420ToAR30")?;

    let result = unsafe {
        sys::H420ToAR30(
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

    Error::check(result, "H420ToAR30")
}

/// H420 (BT.709 I420) から AB30 への変換
pub fn h420_to_ab30(
    src: &H420Image<'_>,
    dst: &mut Ab30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H420ToAB30")?;
    dst.validate(size, "H420ToAB30")?;

    let result = unsafe {
        sys::H420ToAB30(
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

    Error::check(result, "H420ToAB30")
}

/// H420 (BT.709 I420) から RAW への変換
pub fn h420_to_raw(
    src: &H420Image<'_>,
    dst: &mut RawImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H420ToRAW")?;
    dst.validate(size, "H420ToRAW")?;

    let result = unsafe {
        sys::H420ToRAW(
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

    Error::check(result, "H420ToRAW")
}

/// H420 (BT.709 I420) から RGB24 への変換
pub fn h420_to_rgb24(
    src: &H420Image<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H420ToRGB24")?;
    dst.validate(size, "H420ToRGB24")?;

    let result = unsafe {
        sys::H420ToRGB24(
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

    Error::check(result, "H420ToRGB24")
}

/// H420 (BT.709 I420) から RGB565 への変換
pub fn h420_to_rgb565(
    src: &H420Image<'_>,
    dst: &mut Rgb565ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H420ToRGB565")?;
    dst.validate(size, "H420ToRGB565")?;

    let result = unsafe {
        sys::H420ToRGB565(
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

    Error::check(result, "H420ToRGB565")
}

/// H422 (BT.709 I422) から ARGB への変換
pub fn h422_to_argb(
    src: &H422Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H422ToARGB")?;
    dst.validate(size, "H422ToARGB")?;

    let result = unsafe {
        sys::H422ToARGB(
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

    Error::check(result, "H422ToARGB")
}

/// H422 (BT.709 I422) から ABGR への変換
pub fn h422_to_abgr(
    src: &H422Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H422ToABGR")?;
    dst.validate(size, "H422ToABGR")?;

    let result = unsafe {
        sys::H422ToABGR(
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

    Error::check(result, "H422ToABGR")
}

/// H444 (BT.709 I444) から ARGB への変換
pub fn h444_to_argb(
    src: &H444Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H444ToARGB")?;
    dst.validate(size, "H444ToARGB")?;

    let result = unsafe {
        sys::H444ToARGB(
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

    Error::check(result, "H444ToARGB")
}

/// H444 (BT.709 I444) から ABGR への変換
pub fn h444_to_abgr(
    src: &H444Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "H444ToABGR")?;
    dst.validate(size, "H444ToABGR")?;

    let result = unsafe {
        sys::H444ToABGR(
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

    Error::check(result, "H444ToABGR")
}

// ============================================================
// U 系 (BT.2020) 10bit 変換
// ============================================================

/// U010 (BT.2020 10bit I420) から ARGB への変換
pub fn u010_to_argb(
    src: &U010Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U010ToARGB")?;
    dst.validate(size, "U010ToARGB")?;

    let result = unsafe {
        sys::U010ToARGB(
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

    Error::check(result, "U010ToARGB")
}

/// U010 (BT.2020 10bit I420) から ABGR への変換
pub fn u010_to_abgr(
    src: &U010Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U010ToABGR")?;
    dst.validate(size, "U010ToABGR")?;

    let result = unsafe {
        sys::U010ToABGR(
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

    Error::check(result, "U010ToABGR")
}

/// U010 (BT.2020 10bit I420) から AR30 への変換
pub fn u010_to_ar30(
    src: &U010Image<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U010ToAR30")?;
    dst.validate(size, "U010ToAR30")?;

    let result = unsafe {
        sys::U010ToAR30(
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

    Error::check(result, "U010ToAR30")
}

/// U010 (BT.2020 10bit I420) から AB30 への変換
pub fn u010_to_ab30(
    src: &U010Image<'_>,
    dst: &mut Ab30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U010ToAB30")?;
    dst.validate(size, "U010ToAB30")?;

    let result = unsafe {
        sys::U010ToAB30(
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

    Error::check(result, "U010ToAB30")
}

/// U210 (BT.2020 10bit I422) から ARGB への変換
pub fn u210_to_argb(
    src: &U210Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U210ToARGB")?;
    dst.validate(size, "U210ToARGB")?;

    let result = unsafe {
        sys::U210ToARGB(
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

    Error::check(result, "U210ToARGB")
}

/// U210 (BT.2020 10bit I422) から ABGR への変換
pub fn u210_to_abgr(
    src: &U210Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U210ToABGR")?;
    dst.validate(size, "U210ToABGR")?;

    let result = unsafe {
        sys::U210ToABGR(
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

    Error::check(result, "U210ToABGR")
}

/// U210 (BT.2020 10bit I422) から AR30 への変換
pub fn u210_to_ar30(
    src: &U210Image<'_>,
    dst: &mut Ar30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U210ToAR30")?;
    dst.validate(size, "U210ToAR30")?;

    let result = unsafe {
        sys::U210ToAR30(
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

    Error::check(result, "U210ToAR30")
}

/// U210 (BT.2020 10bit I422) から AB30 への変換
pub fn u210_to_ab30(
    src: &U210Image<'_>,
    dst: &mut Ab30ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U210ToAB30")?;
    dst.validate(size, "U210ToAB30")?;

    let result = unsafe {
        sys::U210ToAB30(
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

    Error::check(result, "U210ToAB30")
}

// ============================================================
// U 系 (BT.2020) 8bit 変換
// ============================================================

/// U420 (BT.2020 I420) から ARGB への変換
pub fn u420_to_argb(
    src: &U420Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U420ToARGB")?;
    dst.validate(size, "U420ToARGB")?;

    let result = unsafe {
        sys::U420ToARGB(
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

    Error::check(result, "U420ToARGB")
}

/// U420 (BT.2020 I420) から ABGR への変換
pub fn u420_to_abgr(
    src: &U420Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U420ToABGR")?;
    dst.validate(size, "U420ToABGR")?;

    let result = unsafe {
        sys::U420ToABGR(
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

    Error::check(result, "U420ToABGR")
}

/// U422 (BT.2020 I422) から ARGB への変換
pub fn u422_to_argb(
    src: &U422Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U422ToARGB")?;
    dst.validate(size, "U422ToARGB")?;

    let result = unsafe {
        sys::U422ToARGB(
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

    Error::check(result, "U422ToARGB")
}

/// U422 (BT.2020 I422) から ABGR への変換
pub fn u422_to_abgr(
    src: &U422Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U422ToABGR")?;
    dst.validate(size, "U422ToABGR")?;

    let result = unsafe {
        sys::U422ToABGR(
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

    Error::check(result, "U422ToABGR")
}

/// U444 (BT.2020 I444) から ARGB への変換
pub fn u444_to_argb(
    src: &U444Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U444ToARGB")?;
    dst.validate(size, "U444ToARGB")?;

    let result = unsafe {
        sys::U444ToARGB(
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

    Error::check(result, "U444ToARGB")
}

/// U444 (BT.2020 I444) から ABGR への変換
pub fn u444_to_abgr(
    src: &U444Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "U444ToABGR")?;
    dst.validate(size, "U444ToABGR")?;

    let result = unsafe {
        sys::U444ToABGR(
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

    Error::check(result, "U444ToABGR")
}

// ============================================================
// P 系 (packed 10bit) 変換
// ============================================================

/// P010 から I010 (10bit I420) への変換
pub fn p010_to_i010(
    src: &P010Image<'_>,
    dst: &mut I010ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "P010ToI010")?;
    dst.validate(size, "P010ToI010")?;

    let result = unsafe {
        sys::P010ToI010(
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

    Error::check(result, "P010ToI010")
}

/// P010 から NV12 (8bit) への変換
pub fn p010_to_nv12(
    src: &P010Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "P010ToNV12")?;
    dst.validate(size, "P010ToNV12")?;

    let result = unsafe {
        sys::P010ToNV12(
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

    Error::check(result, "P010ToNV12")
}

/// P010 から P410 (4:4:4) への変換
pub fn p010_to_p410(
    src: &P010Image<'_>,
    dst: &mut P410ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "P010ToP410")?;
    dst.validate(size, "P010ToP410")?;

    let result = unsafe {
        sys::P010ToP410(
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

    Error::check(result, "P010ToP410")
}

/// P210 から P410 (4:4:4) への変換
pub fn p210_to_p410(
    src: &P210Image<'_>,
    dst: &mut P410ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    // P210 はクロマの高さ = 輝度の高さ（4:2:2）
    // ソースのバリデーションもクロマ高さ = height を使う
    if src.y.len() < src.y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "P210ToP410",
            "source Y buffer too small",
        ));
    }
    if src.uv.len() < src.uv_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "P210ToP410",
            "source UV buffer too small",
        ));
    }
    dst.validate(size, "P210ToP410")?;

    let result = unsafe {
        sys::P210ToP410(
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

    Error::check(result, "P210ToP410")
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
// 追加コピー
// ============================================================

/// I422 画像のコピー
pub fn i422_copy(
    src: &I422Image<'_>,
    dst: &mut I422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422Copy")?;
    dst.validate(size, "I422Copy")?;

    let result = unsafe {
        sys::I422Copy(
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

    Error::check(result, "I422Copy")
}

/// I444 画像のコピー
pub fn i444_copy(
    src: &I444Image<'_>,
    dst: &mut I444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I444Copy")?;
    dst.validate(size, "I444Copy")?;

    let result = unsafe {
        sys::I444Copy(
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

    Error::check(result, "I444Copy")
}

/// NV21 画像のコピー
pub fn nv21_copy(
    src: &Nv21Image<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV21Copy")?;
    dst.validate(size, "NV21Copy")?;

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
// Android フォーマット
// ============================================================

/// Android420 から ARGB への変換
///
/// Android の NV12/NV21 系フォーマットから ARGB に変換する。
/// `pixel_stride_uv` は UV ピクセルストライド（1: planar、2: interleaved）。
pub fn android420_to_argb(
    src: &Android420Image<'_>,
    pixel_stride_uv: usize,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "Android420ToARGB")?;
    dst.validate(size, "Android420ToARGB")?;

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
pub fn android420_to_abgr(
    src: &Android420Image<'_>,
    pixel_stride_uv: usize,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "Android420ToABGR")?;
    dst.validate(size, "Android420ToABGR")?;

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
pub fn android420_to_i420(
    src: &Android420Image<'_>,
    pixel_stride_uv: usize,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "Android420ToI420")?;
    dst.validate(size, "Android420ToI420")?;

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
pub fn detile_plane(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_stride: usize,
    size: ImageSize,
    tile_height: usize,
) -> Result<(), Error> {
    if src.len() < src_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "DetilePlane",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "DetilePlane",
            "destination buffer too small",
        ));
    }

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
pub fn detile_plane_16(
    src: &[u16],
    src_stride: usize,
    dst: &mut [u16],
    dst_stride: usize,
    size: ImageSize,
    tile_height: usize,
) -> Result<(), Error> {
    if src.len() < src_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "DetilePlane_16",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "DetilePlane_16",
            "destination buffer too small",
        ));
    }

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
) {
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
}

/// タイル化された Y と UV プレーンから YUY2 に変換する
pub fn detile_to_yuy2(
    src: &Nv12Image<'_>,
    dst: &mut Yuy2ImageMut<'_>,
    size: ImageSize,
    tile_height: usize,
) -> Result<(), Error> {
    src.validate(size, "DetileToYUY2")?;
    dst.validate(size, "DetileToYUY2")?;

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
/// validate_biplanar_dst は chroma を height/2 で検証するため、
/// dst の chroma バッファサイズを別途検証する。
pub fn nv12_to_nv24(
    src: &Nv12Image<'_>,
    dst: &mut Nv24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12ToNV24")?;
    // NV24 の dst は chroma の高さが luma と同じなので個別に検証する
    if dst.y.len() < dst.y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "NV12ToNV24",
            "destination Y buffer too small",
        ));
    }
    if dst.uv.len() < dst.uv_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "NV12ToNV24",
            "destination UV buffer too small",
        ));
    }

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
/// validate_biplanar_src/dst は chroma を height/2 で検証するため、
/// src/dst の chroma バッファサイズを別途検証する。
pub fn nv16_to_nv24(
    src: &Nv16Image<'_>,
    dst: &mut Nv24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    // NV16 の src は chroma の高さが luma と同じなので個別に検証する
    if src.y.len() < src.y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "NV16ToNV24",
            "source Y buffer too small",
        ));
    }
    if src.uv.len() < src.uv_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "NV16ToNV24",
            "source UV buffer too small",
        ));
    }
    // NV24 の dst は chroma の高さが luma と同じなので個別に検証する
    if dst.y.len() < dst.y_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "NV16ToNV24",
            "destination Y buffer too small",
        ));
    }
    if dst.uv.len() < dst.uv_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "NV16ToNV24",
            "destination UV buffer too small",
        ));
    }

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

/// ABGR から NV12 への変換
pub fn abgr_to_nv12(
    src: &AbgrImage<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ABGRToNV12")?;
    dst.validate(size, "ABGRToNV12")?;

    let result = unsafe {
        sys::ABGRToNV12(
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

    Error::check(result, "ABGRToNV12")
}

/// ABGR から NV21 への変換
pub fn abgr_to_nv21(
    src: &AbgrImage<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ABGRToNV21")?;
    dst.validate(size, "ABGRToNV21")?;

    let result = unsafe {
        sys::ABGRToNV21(
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

    Error::check(result, "ABGRToNV21")
}

// ============================================================
// YUY2 -> 他
// ============================================================

/// YUY2 から ARGB への変換
pub fn yuy2_to_argb(
    src: &Yuy2Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToARGB")?;
    dst.validate(size, "YUY2ToARGB")?;

    let result = unsafe {
        sys::YUY2ToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "YUY2ToARGB")
}

/// YUY2 から I420 への変換
pub fn yuy2_to_i420(
    src: &Yuy2Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToI420")?;
    dst.validate(size, "YUY2ToI420")?;

    let result = unsafe {
        sys::YUY2ToI420(
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

    Error::check(result, "YUY2ToI420")
}

/// YUY2 から I422 への変換
pub fn yuy2_to_i422(
    src: &Yuy2Image<'_>,
    dst: &mut I422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToI422")?;
    dst.validate(size, "YUY2ToI422")?;

    let result = unsafe {
        sys::YUY2ToI422(
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

    Error::check(result, "YUY2ToI422")
}

/// YUY2 から NV12 への変換
pub fn yuy2_to_nv12(
    src: &Yuy2Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToNV12")?;
    dst.validate(size, "YUY2ToNV12")?;

    let result = unsafe {
        sys::YUY2ToNV12(
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

    Error::check(result, "YUY2ToNV12")
}

/// YUY2 から Y プレーンへの変換
pub fn yuy2_to_y(
    src: &Yuy2Image<'_>,
    dst_y: &mut [u8],
    dst_stride_y: usize,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToY")?;
    if dst_y.len() < dst_stride_y * size.height {
        return Err(Error::with_reason(
            -1,
            "YUY2ToY",
            "destination Y buffer too small",
        ));
    }

    let result = unsafe {
        sys::YUY2ToY(
            src.data.as_ptr(),
            src.stride as c_int,
            dst_y.as_mut_ptr(),
            dst_stride_y as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "YUY2ToY")
}

// ============================================================
// 他 -> YUY2
// ============================================================

/// ARGB から YUY2 への変換
pub fn argb_to_yuy2(
    src: &ArgbImage<'_>,
    dst: &mut Yuy2ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToYUY2")?;
    dst.validate(size, "ARGBToYUY2")?;

    let result = unsafe {
        sys::ARGBToYUY2(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToYUY2")
}

/// I420 から YUY2 への変換
pub fn i420_to_yuy2(
    src: &I420Image<'_>,
    dst: &mut Yuy2ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToYUY2")?;
    dst.validate(size, "I420ToYUY2")?;

    let result = unsafe {
        sys::I420ToYUY2(
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

    Error::check(result, "I420ToYUY2")
}

/// I422 から YUY2 への変換
pub fn i422_to_yuy2(
    src: &I422Image<'_>,
    dst: &mut Yuy2ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToYUY2")?;
    dst.validate(size, "I422ToYUY2")?;

    let result = unsafe {
        sys::I422ToYUY2(
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

    Error::check(result, "I422ToYUY2")
}

// ============================================================
// UYVY -> 他
// ============================================================

/// UYVY から ARGB への変換
pub fn uyvy_to_argb(
    src: &UyvyImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToARGB")?;
    dst.validate(size, "UYVYToARGB")?;

    let result = unsafe {
        sys::UYVYToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "UYVYToARGB")
}

/// UYVY から I420 への変換
pub fn uyvy_to_i420(
    src: &UyvyImage<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToI420")?;
    dst.validate(size, "UYVYToI420")?;

    let result = unsafe {
        sys::UYVYToI420(
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

    Error::check(result, "UYVYToI420")
}

/// UYVY から I422 への変換
pub fn uyvy_to_i422(
    src: &UyvyImage<'_>,
    dst: &mut I422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToI422")?;
    dst.validate(size, "UYVYToI422")?;

    let result = unsafe {
        sys::UYVYToI422(
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

    Error::check(result, "UYVYToI422")
}

/// UYVY から NV12 への変換
pub fn uyvy_to_nv12(
    src: &UyvyImage<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToNV12")?;
    dst.validate(size, "UYVYToNV12")?;

    let result = unsafe {
        sys::UYVYToNV12(
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

    Error::check(result, "UYVYToNV12")
}

/// UYVY から Y プレーンへの変換
pub fn uyvy_to_y(
    src: &UyvyImage<'_>,
    dst_y: &mut [u8],
    dst_stride_y: usize,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToY")?;
    if dst_y.len() < dst_stride_y * size.height {
        return Err(Error::with_reason(
            -1,
            "UYVYToY",
            "destination Y buffer too small",
        ));
    }

    let result = unsafe {
        sys::UYVYToY(
            src.data.as_ptr(),
            src.stride as c_int,
            dst_y.as_mut_ptr(),
            dst_stride_y as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "UYVYToY")
}

// ============================================================
// 他 -> UYVY
// ============================================================

/// ARGB から UYVY への変換
pub fn argb_to_uyvy(
    src: &ArgbImage<'_>,
    dst: &mut UyvyImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToUYVY")?;
    dst.validate(size, "ARGBToUYVY")?;

    let result = unsafe {
        sys::ARGBToUYVY(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToUYVY")
}

/// I420 から UYVY への変換
pub fn i420_to_uyvy(
    src: &I420Image<'_>,
    dst: &mut UyvyImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToUYVY")?;
    dst.validate(size, "I420ToUYVY")?;

    let result = unsafe {
        sys::I420ToUYVY(
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

    Error::check(result, "I420ToUYVY")
}

/// I422 から UYVY への変換
pub fn i422_to_uyvy(
    src: &I422Image<'_>,
    dst: &mut UyvyImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToUYVY")?;
    dst.validate(size, "I422ToUYVY")?;

    let result = unsafe {
        sys::I422ToUYVY(
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

    Error::check(result, "I422ToUYVY")
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
fn validate_alpha_src(
    src_a: &[u8],
    src_stride_a: usize,
    size: ImageSize,
    function: &'static str,
) -> Result<(), Error> {
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
fn validate_alpha_dst(
    dst_a: &[u8],
    dst_stride_a: usize,
    size: ImageSize,
    function: &'static str,
) -> Result<(), Error> {
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

// ============================================================
// J400 (JPEG グレースケール) 変換
// ============================================================

/// J400 (JPEG グレースケール) から ARGB への変換
pub fn j400_to_argb(
    src: &J400Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J400ToARGB")?;
    dst.validate(size, "J400ToARGB")?;

    let result = unsafe {
        sys::J400ToARGB(
            src.y.as_ptr(),
            src.y_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "J400ToARGB")
}

// ============================================================
// J420 (JPEG 4:2:0) 変換
// ============================================================

/// J420 (JPEG I420) から ARGB への変換
pub fn j420_to_argb(
    src: &J420Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J420ToARGB")?;
    dst.validate(size, "J420ToARGB")?;

    let result = unsafe {
        sys::J420ToARGB(
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

    Error::check(result, "J420ToARGB")
}

/// J420 (JPEG I420) から ABGR への変換
pub fn j420_to_abgr(
    src: &J420Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J420ToABGR")?;
    dst.validate(size, "J420ToABGR")?;

    let result = unsafe {
        sys::J420ToABGR(
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

    Error::check(result, "J420ToABGR")
}

/// J420 (JPEG I420) から RAW への変換
pub fn j420_to_raw(
    src: &J420Image<'_>,
    dst: &mut RawImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J420ToRAW")?;
    dst.validate(size, "J420ToRAW")?;

    let result = unsafe {
        sys::J420ToRAW(
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

    Error::check(result, "J420ToRAW")
}

/// J420 (JPEG I420) から RGB24 への変換
pub fn j420_to_rgb24(
    src: &J420Image<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J420ToRGB24")?;
    dst.validate(size, "J420ToRGB24")?;

    let result = unsafe {
        sys::J420ToRGB24(
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

    Error::check(result, "J420ToRGB24")
}

/// J420 (JPEG I420) から RGB565 への変換
pub fn j420_to_rgb565(
    src: &J420Image<'_>,
    dst: &mut Rgb565ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J420ToRGB565")?;
    dst.validate(size, "J420ToRGB565")?;

    let result = unsafe {
        sys::J420ToRGB565(
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

    Error::check(result, "J420ToRGB565")
}

/// J420 (JPEG I420) から I420 への変換
pub fn j420_to_i420(
    src: &J420Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J420ToI420")?;
    dst.validate(size, "J420ToI420")?;

    let result = unsafe {
        sys::J420ToI420(
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

    Error::check(result, "J420ToI420")
}

// ============================================================
// J422 (JPEG 4:2:2) 変換
// ============================================================

/// J422 (JPEG I422) から ARGB への変換
pub fn j422_to_argb(
    src: &J422Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J422ToARGB")?;
    dst.validate(size, "J422ToARGB")?;

    let result = unsafe {
        sys::J422ToARGB(
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

    Error::check(result, "J422ToARGB")
}

/// J422 (JPEG I422) から ABGR への変換
pub fn j422_to_abgr(
    src: &J422Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J422ToABGR")?;
    dst.validate(size, "J422ToABGR")?;

    let result = unsafe {
        sys::J422ToABGR(
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

    Error::check(result, "J422ToABGR")
}

// ============================================================
// J444 (JPEG 4:4:4) 変換
// ============================================================

/// J444 (JPEG I444) から ARGB への変換
pub fn j444_to_argb(
    src: &J444Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J444ToARGB")?;
    dst.validate(size, "J444ToARGB")?;

    let result = unsafe {
        sys::J444ToARGB(
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

    Error::check(result, "J444ToARGB")
}

/// J444 (JPEG I444) から ABGR への変換
pub fn j444_to_abgr(
    src: &J444Image<'_>,
    dst: &mut AbgrImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "J444ToABGR")?;
    dst.validate(size, "J444ToABGR")?;

    let result = unsafe {
        sys::J444ToABGR(
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

    Error::check(result, "J444ToABGR")
}

// ============================================================
// ARGB -> J系 変換
// ============================================================

/// ARGB から J400 (JPEG グレースケール) への変換
pub fn argb_to_j400(
    src: &ArgbImage<'_>,
    dst: &mut J400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToJ400")?;
    dst.validate(size, "ARGBToJ400")?;

    let result = unsafe {
        sys::ARGBToJ400(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToJ400")
}

/// ARGB から J420 (JPEG I420) への変換
pub fn argb_to_j420(
    src: &ArgbImage<'_>,
    dst: &mut J420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToJ420")?;
    dst.validate(size, "ARGBToJ420")?;

    let result = unsafe {
        sys::ARGBToJ420(
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

    Error::check(result, "ARGBToJ420")
}

/// ARGB から J422 (JPEG I422) への変換
pub fn argb_to_j422(
    src: &ArgbImage<'_>,
    dst: &mut J422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToJ422")?;
    dst.validate(size, "ARGBToJ422")?;

    let result = unsafe {
        sys::ARGBToJ422(
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

    Error::check(result, "ARGBToJ422")
}

/// ARGB から J444 (JPEG I444) への変換
pub fn argb_to_j444(
    src: &ArgbImage<'_>,
    dst: &mut J444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToJ444")?;
    dst.validate(size, "ARGBToJ444")?;

    let result = unsafe {
        sys::ARGBToJ444(
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

    Error::check(result, "ARGBToJ444")
}

// ============================================================
// ABGR -> J系 変換
// ============================================================

/// ABGR から J400 (JPEG グレースケール) への変換
pub fn abgr_to_j400(
    src: &AbgrImage<'_>,
    dst: &mut J400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ABGRToJ400")?;
    dst.validate(size, "ABGRToJ400")?;

    let result = unsafe {
        sys::ABGRToJ400(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ABGRToJ400")
}

/// ABGR から J420 (JPEG I420) への変換
pub fn abgr_to_j420(
    src: &AbgrImage<'_>,
    dst: &mut J420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ABGRToJ420")?;
    dst.validate(size, "ABGRToJ420")?;

    let result = unsafe {
        sys::ABGRToJ420(
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

    Error::check(result, "ABGRToJ420")
}

/// ABGR から J422 (JPEG I422) への変換
pub fn abgr_to_j422(
    src: &AbgrImage<'_>,
    dst: &mut J422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ABGRToJ422")?;
    dst.validate(size, "ABGRToJ422")?;

    let result = unsafe {
        sys::ABGRToJ422(
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

    Error::check(result, "ABGRToJ422")
}

// ============================================================
// RAW -> J系 変換
// ============================================================

/// RAW から J400 (JPEG グレースケール) への変換
pub fn raw_to_j400(
    src: &RawImage<'_>,
    dst: &mut J400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RAWToJ400")?;
    dst.validate(size, "RAWToJ400")?;

    let result = unsafe {
        sys::RAWToJ400(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RAWToJ400")
}

/// RAW から J420 (JPEG I420) への変換
pub fn raw_to_j420(
    src: &RawImage<'_>,
    dst: &mut J420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RAWToJ420")?;
    dst.validate(size, "RAWToJ420")?;

    let result = unsafe {
        sys::RAWToJ420(
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

    Error::check(result, "RAWToJ420")
}

/// RAW から J444 (JPEG I444) への変換
pub fn raw_to_j444(
    src: &RawImage<'_>,
    dst: &mut J444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RAWToJ444")?;
    dst.validate(size, "RAWToJ444")?;

    let result = unsafe {
        sys::RAWToJ444(
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

    Error::check(result, "RAWToJ444")
}

/// RAW から JNV21 (JPEG NV21) への変換
pub fn raw_to_jnv21(
    src: &RawImage<'_>,
    dst: &mut Nv21ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RAWToJNV21")?;
    dst.validate(size, "RAWToJNV21")?;

    let result = unsafe {
        sys::RAWToJNV21(
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

    Error::check(result, "RAWToJNV21")
}

// ============================================================
// RGB24 -> J系 変換
// ============================================================

/// RGB24 から J400 (JPEG グレースケール) への変換
pub fn rgb24_to_j400(
    src: &Rgb24Image<'_>,
    dst: &mut J400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGB24ToJ400")?;
    dst.validate(size, "RGB24ToJ400")?;

    let result = unsafe {
        sys::RGB24ToJ400(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RGB24ToJ400")
}

/// RGB24 から J420 (JPEG I420) への変換
pub fn rgb24_to_j420(
    src: &Rgb24Image<'_>,
    dst: &mut J420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGB24ToJ420")?;
    dst.validate(size, "RGB24ToJ420")?;

    let result = unsafe {
        sys::RGB24ToJ420(
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

    Error::check(result, "RGB24ToJ420")
}

// ============================================================
// RGBA -> J系 変換
// ============================================================

/// RGBA から J400 (JPEG グレースケール) への変換
pub fn rgba_to_j400(
    src: &RgbaImage<'_>,
    dst: &mut J400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGBAToJ400")?;
    dst.validate(size, "RGBAToJ400")?;

    let result = unsafe {
        sys::RGBAToJ400(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RGBAToJ400")
}

// ============================================================
// RGBA 変換
// ============================================================

/// ARGB から RGBA への変換
pub fn argb_to_rgba(
    src: &ArgbImage<'_>,
    dst: &mut RgbaImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToRGBA")?;
    dst.validate(size, "ARGBToRGBA")?;

    let result = unsafe {
        sys::ARGBToRGBA(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToRGBA")
}

/// RGBA から ARGB への変換
pub fn rgba_to_argb(
    src: &RgbaImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGBAToARGB")?;
    dst.validate(size, "RGBAToARGB")?;

    let result = unsafe {
        sys::RGBAToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RGBAToARGB")
}

/// RGBA から I420 への変換
pub fn rgba_to_i420(
    src: &RgbaImage<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGBAToI420")?;
    dst.validate(size, "RGBAToI420")?;

    let result = unsafe {
        sys::RGBAToI420(
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

    Error::check(result, "RGBAToI420")
}

/// I420 から RGBA への変換
pub fn i420_to_rgba(
    src: &I420Image<'_>,
    dst: &mut RgbaImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToRGBA")?;
    dst.validate(size, "I420ToRGBA")?;

    let result = unsafe {
        sys::I420ToRGBA(
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

    Error::check(result, "I420ToRGBA")
}

/// I422 から RGBA への変換
pub fn i422_to_rgba(
    src: &I422Image<'_>,
    dst: &mut RgbaImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToRGBA")?;
    dst.validate(size, "I422ToRGBA")?;

    let result = unsafe {
        sys::I422ToRGBA(
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

    Error::check(result, "I422ToRGBA")
}

// ============================================================
// BGRA 変換
// ============================================================

/// BGRA から ARGB への変換
pub fn bgra_to_argb(
    src: &BgraImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "BGRAToARGB")?;
    dst.validate(size, "BGRAToARGB")?;

    let result = unsafe {
        sys::BGRAToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "BGRAToARGB")
}

/// BGRA から I420 への変換
pub fn bgra_to_i420(
    src: &BgraImage<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "BGRAToI420")?;
    dst.validate(size, "BGRAToI420")?;

    let result = unsafe {
        sys::BGRAToI420(
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

    Error::check(result, "BGRAToI420")
}

/// I420 から BGRA への変換
pub fn i420_to_bgra(
    src: &I420Image<'_>,
    dst: &mut BgraImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToBGRA")?;
    dst.validate(size, "I420ToBGRA")?;

    let result = unsafe {
        sys::I420ToBGRA(
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

    Error::check(result, "I420ToBGRA")
}

/// I422 から BGRA への変換
pub fn i422_to_bgra(
    src: &I422Image<'_>,
    dst: &mut BgraImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToBGRA")?;
    dst.validate(size, "I422ToBGRA")?;

    let result = unsafe {
        sys::I422ToBGRA(
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

    Error::check(result, "I422ToBGRA")
}

/// ARGB から BGRA への変換
pub fn argb_to_bgra(
    src: &ArgbImage<'_>,
    dst: &mut BgraImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToBGRA")?;
    dst.validate(size, "ARGBToBGRA")?;

    let result = unsafe {
        sys::ARGBToBGRA(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToBGRA")
}

// ============================================================
// RAW 変換
// ============================================================

/// RAW から ARGB への変換
pub fn raw_to_argb(
    src: &RawImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RAWToARGB")?;
    dst.validate(size, "RAWToARGB")?;

    let result = unsafe {
        sys::RAWToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RAWToARGB")
}

/// RAW から I420 への変換
pub fn raw_to_i420(
    src: &RawImage<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RAWToI420")?;
    dst.validate(size, "RAWToI420")?;

    let result = unsafe {
        sys::RAWToI420(
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

    Error::check(result, "RAWToI420")
}

/// RAW から RGB24 への変換
pub fn raw_to_rgb24(
    src: &RawImage<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RAWToRGB24")?;
    dst.validate(size, "RAWToRGB24")?;

    let result = unsafe {
        sys::RAWToRGB24(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RAWToRGB24")
}

/// RAW から RGBA への変換
pub fn raw_to_rgba(
    src: &RawImage<'_>,
    dst: &mut RgbaImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RAWToRGBA")?;
    dst.validate(size, "RAWToRGBA")?;

    let result = unsafe {
        sys::RAWToRGBA(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RAWToRGBA")
}

/// ARGB から RAW への変換
pub fn argb_to_raw(
    src: &ArgbImage<'_>,
    dst: &mut RawImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToRAW")?;
    dst.validate(size, "ARGBToRAW")?;

    let result = unsafe {
        sys::ARGBToRAW(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToRAW")
}

/// I420 から RAW への変換
pub fn i420_to_raw(
    src: &I420Image<'_>,
    dst: &mut RawImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToRAW")?;
    dst.validate(size, "I420ToRAW")?;

    let result = unsafe {
        sys::I420ToRAW(
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

    Error::check(result, "I420ToRAW")
}

/// I422 から RAW への変換
pub fn i422_to_raw(
    src: &I422Image<'_>,
    dst: &mut RawImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToRAW")?;
    dst.validate(size, "I422ToRAW")?;

    let result = unsafe {
        sys::I422ToRAW(
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

    Error::check(result, "I422ToRAW")
}

/// I444 から RAW への変換
pub fn i444_to_raw(
    src: &I444Image<'_>,
    dst: &mut RawImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I444ToRAW")?;
    dst.validate(size, "I444ToRAW")?;

    let result = unsafe {
        sys::I444ToRAW(
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

    Error::check(result, "I444ToRAW")
}

/// RAW から I444 への変換
pub fn raw_to_i444(
    src: &RawImage<'_>,
    dst: &mut I444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RAWToI444")?;
    dst.validate(size, "RAWToI444")?;

    let result = unsafe {
        sys::RAWToI444(
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

    Error::check(result, "RAWToI444")
}

// ============================================================
// RGB565 変換
// ============================================================

/// RGB565 から ARGB への変換
pub fn rgb565_to_argb(
    src: &Rgb565Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGB565ToARGB")?;
    dst.validate(size, "RGB565ToARGB")?;

    let result = unsafe {
        sys::RGB565ToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RGB565ToARGB")
}

/// RGB565 から I420 への変換
pub fn rgb565_to_i420(
    src: &Rgb565Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGB565ToI420")?;
    dst.validate(size, "RGB565ToI420")?;

    let result = unsafe {
        sys::RGB565ToI420(
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

    Error::check(result, "RGB565ToI420")
}

/// ARGB から RGB565 への変換
pub fn argb_to_rgb565(
    src: &ArgbImage<'_>,
    dst: &mut Rgb565ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToRGB565")?;
    dst.validate(size, "ARGBToRGB565")?;

    let result = unsafe {
        sys::ARGBToRGB565(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToRGB565")
}

/// ARGB から RGB565 へのディザリング付き変換
///
/// dither4x4 は 4x4 のディザリングテーブル (16 バイト)。
pub fn argb_to_rgb565_dither(
    src: &ArgbImage<'_>,
    dst: &mut Rgb565ImageMut<'_>,
    size: ImageSize,
    dither4x4: &[u8; 16],
) -> Result<(), Error> {
    src.validate(size, "ARGBToRGB565Dither")?;
    dst.validate(size, "ARGBToRGB565Dither")?;

    let result = unsafe {
        sys::ARGBToRGB565Dither(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            dither4x4.as_ptr(),
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToRGB565Dither")
}

/// I420 から RGB565 への変換
pub fn i420_to_rgb565(
    src: &I420Image<'_>,
    dst: &mut Rgb565ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToRGB565")?;
    dst.validate(size, "I420ToRGB565")?;

    let result = unsafe {
        sys::I420ToRGB565(
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

    Error::check(result, "I420ToRGB565")
}

/// I422 (8bit) から RGB565 への変換
pub fn i422_to_rgb565(
    src: &I422Image<'_>,
    dst: &mut Rgb565ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToRGB565")?;
    dst.validate(size, "I422ToRGB565")?;

    let result = unsafe {
        sys::I422ToRGB565(
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

    Error::check(result, "I422ToRGB565")
}

/// I420 から RGB565 へのディザリング付き変換
///
/// dither4x4 は 4x4 のディザリングテーブル (16 バイト)。
pub fn i420_to_rgb565_dither(
    src: &I420Image<'_>,
    dst: &mut Rgb565ImageMut<'_>,
    size: ImageSize,
    dither4x4: &[u8; 16],
) -> Result<(), Error> {
    src.validate(size, "I420ToRGB565Dither")?;
    dst.validate(size, "I420ToRGB565Dither")?;

    let result = unsafe {
        sys::I420ToRGB565Dither(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            dither4x4.as_ptr(),
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I420ToRGB565Dither")
}

// ============================================================
// ARGB1555 変換
// ============================================================

/// ARGB1555 から ARGB への変換
pub fn argb1555_to_argb(
    src: &Argb1555Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGB1555ToARGB")?;
    dst.validate(size, "ARGB1555ToARGB")?;

    let result = unsafe {
        sys::ARGB1555ToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGB1555ToARGB")
}

/// ARGB1555 から I420 への変換
pub fn argb1555_to_i420(
    src: &Argb1555Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGB1555ToI420")?;
    dst.validate(size, "ARGB1555ToI420")?;

    let result = unsafe {
        sys::ARGB1555ToI420(
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

    Error::check(result, "ARGB1555ToI420")
}

/// ARGB から ARGB1555 への変換
pub fn argb_to_argb1555(
    src: &ArgbImage<'_>,
    dst: &mut Argb1555ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToARGB1555")?;
    dst.validate(size, "ARGBToARGB1555")?;

    let result = unsafe {
        sys::ARGBToARGB1555(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToARGB1555")
}

/// I420 から ARGB1555 への変換
pub fn i420_to_argb1555(
    src: &I420Image<'_>,
    dst: &mut Argb1555ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToARGB1555")?;
    dst.validate(size, "I420ToARGB1555")?;

    let result = unsafe {
        sys::I420ToARGB1555(
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

    Error::check(result, "I420ToARGB1555")
}

// ============================================================
// ARGB4444 変換
// ============================================================

/// ARGB4444 から ARGB への変換
pub fn argb4444_to_argb(
    src: &Argb4444Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGB4444ToARGB")?;
    dst.validate(size, "ARGB4444ToARGB")?;

    let result = unsafe {
        sys::ARGB4444ToARGB(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGB4444ToARGB")
}

/// ARGB4444 から I420 への変換
pub fn argb4444_to_i420(
    src: &Argb4444Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGB4444ToI420")?;
    dst.validate(size, "ARGB4444ToI420")?;

    let result = unsafe {
        sys::ARGB4444ToI420(
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

    Error::check(result, "ARGB4444ToI420")
}

/// ARGB から ARGB4444 への変換
pub fn argb_to_argb4444(
    src: &ArgbImage<'_>,
    dst: &mut Argb4444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToARGB4444")?;
    dst.validate(size, "ARGBToARGB4444")?;

    let result = unsafe {
        sys::ARGBToARGB4444(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBToARGB4444")
}

/// I420 から ARGB4444 への変換
pub fn i420_to_argb4444(
    src: &I420Image<'_>,
    dst: &mut Argb4444ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToARGB4444")?;
    dst.validate(size, "I420ToARGB4444")?;

    let result = unsafe {
        sys::I420ToARGB4444(
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

    Error::check(result, "I420ToARGB4444")
}
