//! フォーマット変換関数 (mjpeg)

use std::ffi::c_int;

use crate::{
    ArgbImageMut, Error, I420ImageMut, ImageSize, Nv12ImageMut, Nv21ImageMut, require_c_int, sys,
};

// ============================================================
// MJPEG (Motion JPEG, libjpeg-turbo)
// ============================================================
//
// MJPEG は可変長の JPEG 圧縮データなので、入力は `&[u8]` で受ける。
// 本クレートではスケーリングは無効化し、`size` (= dst のサイズ) を
// `MJPGTo*` の src_width/src_height/dst_width/dst_height 全てに同値で渡す。
// JPEG ヘッダの実サイズと `size` が不一致な場合は libyuv 側がエラーを返す。

/// MJPEG 入力に対する共通の入力検証
///
/// libyuv の `MJPGTo*` を呼ぶ前に、`src` の長さ・`size` の値を `c_int` 範囲で受け、
/// 空入力・ゼロサイズを早期にエラーにする。
fn validate_mjpeg_input(src: &[u8], size: ImageSize, function: &'static str) -> Result<(), Error> {
    if src.is_empty() {
        return Err(Error::with_reason(-1, function, "src is empty"));
    }
    require_c_int(src.len(), function, "src length exceeds c_int range")?;
    require_c_int(size.width, function, "width exceeds c_int range")?;
    require_c_int(size.height, function, "height exceeds c_int range")?;
    if size.width == 0 || size.height == 0 {
        return Err(Error::with_reason(
            -1,
            function,
            "size width/height is zero",
        ));
    }
    Ok(())
}

/// MJPEG 圧縮データから画像サイズを取得する。
///
/// JPEG SOF マーカーをパースし、元画像の幅と高さを返す。`MJPGTo*` 系の関数を呼ぶ前に、
/// 正しい `ImageSize` を取得する用途で使う。
pub fn mjpeg_size(src: &[u8]) -> Result<ImageSize, Error> {
    if src.is_empty() {
        return Err(Error::with_reason(-1, "MJPGSize", "src is empty"));
    }
    require_c_int(src.len(), "MJPGSize", "src length exceeds c_int range")?;

    let mut width: c_int = 0;
    let mut height: c_int = 0;
    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::MJPGSize(
            src.as_ptr(),
            src.len(),
            &mut width as *mut c_int,
            &mut height as *mut c_int,
        )
    };

    Error::check(result, "MJPGSize")?;

    // libyuv は壊れた JPEG に対しても 0 を返すことがあるので、戻り値の範囲も追加で確認する
    if width <= 0 || height <= 0 {
        return Err(Error::with_reason(
            -1,
            "MJPGSize",
            "decoded JPEG dimensions are non-positive",
        ));
    }
    Ok(ImageSize::new(width as usize, height as usize))
}

/// MJPEG 圧縮データから I420 へデコードする (スケーリング非対応)。
///
/// `size` には `mjpeg_size(src)` で取得した値を渡す。JPEG ヘッダの実サイズと
/// 異なる `size` が渡されたときは libyuv の `MJPGToI420` がエラーを返す。
pub fn mjpeg_to_i420(src: &[u8], dst: &mut I420ImageMut<'_>, size: ImageSize) -> Result<(), Error> {
    validate_mjpeg_input(src, size, "MJPGToI420")?;
    dst.validate(size, "MJPGToI420")?;

    // libyuv の MJPGTo* は src_width / src_height / dst_width / dst_height を取るが、
    // スケーリング無効化のため 4 つすべてに同じ width / height を渡す
    let w = size.width as c_int;
    let h = size.height as c_int;
    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::MJPGToI420(
            src.as_ptr(),
            src.len(),
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            w,
            h,
            w,
            h,
        )
    };

    Error::check(result, "MJPGToI420")
}

/// MJPEG 圧縮データから NV12 へデコードする (スケーリング非対応)。
pub fn mjpeg_to_nv12(src: &[u8], dst: &mut Nv12ImageMut<'_>, size: ImageSize) -> Result<(), Error> {
    validate_mjpeg_input(src, size, "MJPGToNV12")?;
    dst.validate(size, "MJPGToNV12")?;

    let w = size.width as c_int;
    let h = size.height as c_int;
    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::MJPGToNV12(
            src.as_ptr(),
            src.len(),
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            w,
            h,
            w,
            h,
        )
    };

    Error::check(result, "MJPGToNV12")
}

/// MJPEG 圧縮データから NV21 へデコードする (スケーリング非対応)。
pub fn mjpeg_to_nv21(src: &[u8], dst: &mut Nv21ImageMut<'_>, size: ImageSize) -> Result<(), Error> {
    validate_mjpeg_input(src, size, "MJPGToNV21")?;
    dst.validate(size, "MJPGToNV21")?;

    let w = size.width as c_int;
    let h = size.height as c_int;
    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::MJPGToNV21(
            src.as_ptr(),
            src.len(),
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            w,
            h,
            w,
            h,
        )
    };

    Error::check(result, "MJPGToNV21")
}

/// MJPEG 圧縮データから ARGB へデコードする (スケーリング非対応)。
pub fn mjpeg_to_argb(src: &[u8], dst: &mut ArgbImageMut<'_>, size: ImageSize) -> Result<(), Error> {
    validate_mjpeg_input(src, size, "MJPGToARGB")?;
    dst.validate(size, "MJPGToARGB")?;

    let w = size.width as c_int;
    let h = size.height as c_int;
    // SAFETY: .validate() が全前提条件を検査済み。
    let result = unsafe {
        sys::MJPGToARGB(
            src.as_ptr(),
            src.len(),
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            w,
            h,
            w,
            h,
        )
    };

    Error::check(result, "MJPGToARGB")
}
