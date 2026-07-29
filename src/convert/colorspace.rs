//! フォーマット変換関数 (colorspace)

use std::ffi::c_int;

use crate::{
    Ab30ImageMut, AbgrImageMut, Ar30ImageMut, ArgbImageMut, Error, H010Image, H210Image, H420Image,
    H422Image, H444Image, ImageSize, RawImageMut, Rgb24ImageMut, Rgb565ImageMut, U010Image,
    U210Image, U420Image, U422Image, U444Image, sys,
};

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
