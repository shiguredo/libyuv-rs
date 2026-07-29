//! フォーマット変換関数 (argb)

use std::ffi::c_int;

use crate::{
    Ab30ImageMut, Ab64Image, Ab64ImageMut, AbgrImage, AbgrImageMut, Ar30Image, Ar30ImageMut,
    Ar64Image, Ar64ImageMut, ArgbImage, ArgbImageMut, BgraImage, BgraImageMut, Error, I420Image,
    I420ImageMut, I422Image, I422ImageMut, I444Image, I444ImageMut, ImageSize, Nv12ImageMut,
    Nv21ImageMut, RawImage, RawImageMut, Rgb24Image, Rgb24ImageMut, RgbaImage, RgbaImageMut, sys,
};

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
// 追加 NV12/NV21 変換
// ============================================================

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
