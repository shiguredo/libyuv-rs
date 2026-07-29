//! フォーマット変換関数 (high_bitdepth)

use std::ffi::c_int;

use crate::{
    Ab30ImageMut, AbgrImageMut, Ar30ImageMut, ArgbImageMut, Error, I010Image, I010ImageMut,
    I012Image, I012ImageMut, I210Image, I210ImageMut, I212Image, I410Image, I410ImageMut,
    I412Image, I420Image, I420ImageMut, I422Image, I422ImageMut, I444ImageMut, ImageSize,
    Nv12ImageMut, P010ImageMut, P012Image, P012ImageMut, P210ImageMut, P212ImageMut, sys,
};

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
