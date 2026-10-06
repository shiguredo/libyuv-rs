//! フォーマット変換関数 (subsampling)

use std::ffi::c_int;

use crate::{
    Error, I422Image, I422ImageMut, I444Image, I444ImageMut, ImageSize, Rgb24ImageMut, sys,
};

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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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
