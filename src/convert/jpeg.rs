//! フォーマット変換関数 (jpeg)

use std::ffi::c_int;

use crate::{
    AbgrImage, AbgrImageMut, ArgbImage, ArgbImageMut, Error, I420ImageMut, ImageSize, J400Image,
    J400ImageMut, J420Image, J420ImageMut, J422Image, J422ImageMut, J444Image, J444ImageMut,
    Nv21ImageMut, RawImage, RawImageMut, Rgb24Image, Rgb24ImageMut, Rgb565ImageMut, RgbaImage, sys,
};

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
