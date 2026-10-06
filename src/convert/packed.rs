//! フォーマット変換関数 (packed)

use std::ffi::c_int;

use crate::{
    Argb1555Image, Argb1555ImageMut, Argb4444Image, Argb4444ImageMut, ArgbImage, ArgbImageMut,
    Error, I010ImageMut, I420Image, I420ImageMut, I422Image, I422ImageMut, ImageSize, Nv12ImageMut,
    P010Image, P210Image, P410ImageMut, Rgb565Image, Rgb565ImageMut, UyvyImage, UyvyImageMut,
    Yuy2Image, Yuy2ImageMut, checked_buf_size, require_c_int, sys,
};

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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// P210 は 4:2:2 サブサンプリングのため、src の UV 高さは luma と同じになる。
/// P410 は 4:4:4 サブサンプリングのため、dst の UV 高さも luma と同じになる。
/// `P210Image::validate` は UV を height で検証し、`P410ImageMut::validate` も UV を height で検証する。
pub fn p210_to_p410(
    src: &P210Image<'_>,
    dst: &mut P410ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "P210ToP410")?;
    dst.validate(size, "P210ToP410")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
// YUY2 -> 他
// ============================================================

// YUY2 / UYVY ソースの最終行の読み越しを考慮した必要サイズ検証。
// libyuv の行関数ラッパー (ANY11 / ANY11C / ANY12 / ANY12S / ANY21S) の余り処理は
// width 奇数で 1 行あたり width * 2 + 2 バイト読み出す (row_any.cc。libyuv 更新時に
// 見直すべき箇所)。width 偶数では読み越しは発生しない。
// なおプラットフォーム非依存の検証のため、CPU ディスパッチによっては読み越えない
// 実装 (例: UYVYToYRow_Any_AVX2) でも最悪ケースの +2 バイトを要求する (安全側)。
// coalesce は C 側の行合体 (Coalesce) が発動するかどうか (関数ごとの条件を呼び出し
// 側で判定して渡す)。Coalesce 発動時は合体後の width * height が奇数のときのみ
// +2 バイトを要求し、非発動時は width 奇数で +2 バイトを要求する
// (必要サイズ = stride * (height - 1) + width * 2 + 2)。
// height == 0 は検証をスキップして現行どおり C 経由の Err に委ねる
// (ゼロサイズの扱いは別途統一予定)。
fn check_yuy2_uyvy_src_overread(
    src: &[u8],
    src_stride: usize,
    size: ImageSize,
    coalesce: bool,
    function: &'static str,
) -> Result<(), Error> {
    // height == 0 では checked_sub が None になるため検証をスキップする
    let Some(height_minus_1) = size.height.checked_sub(1) else {
        return Ok(());
    };
    let base_size = checked_buf_size(
        src_stride,
        size.height,
        function,
        "source buffer size overflow",
    )?;
    let required = if coalesce {
        // 合体後は width * height が 1 行になる。奇数のときのみ +2 バイト読み越す
        let wh = size
            .width
            .checked_mul(size.height)
            .ok_or_else(|| Error::with_reason(-1, function, "source buffer size overflow"))?;
        if wh % 2 == 1 {
            base_size
                .checked_add(2)
                .ok_or_else(|| Error::with_reason(-1, function, "source buffer size overflow"))?
        } else {
            base_size
        }
    } else if size.width % 2 == 1 {
        // 行単位処理。最終行は width * 2 + 2 バイト読み出す
        let head_size = checked_buf_size(
            src_stride,
            height_minus_1,
            function,
            "source buffer size overflow",
        )?;
        let last_row = size
            .width
            .checked_mul(2)
            .and_then(|w2| w2.checked_add(2))
            .ok_or_else(|| Error::with_reason(-1, function, "source buffer size overflow"))?;
        head_size
            .checked_add(last_row)
            .ok_or_else(|| Error::with_reason(-1, function, "source buffer size overflow"))?
    } else {
        base_size
    };
    if src.len() < required {
        return Err(Error::with_reason(-1, function, "source buffer too small"));
    }
    Ok(())
}

// YUY2 / UYVY デスティネーションの最終行の書き込み越えを考慮した必要サイズ検証。
// libyuv の行関数ラッパー (ANY31) の余り処理は width 奇数で 1 行あたり
// width * 2 + 2 バイト書き込む (row_any.cc。libyuv 更新時に見直すべき箇所)。
// width 偶数では書き込み越えは発生しない。i420 / i422 系の行合体 (Coalesce) は
// src_stride_u * 2 == width の条件により偶数幅でのみ発動し、合体後も width * height
// が偶数になるため、width 奇数で常に行単位の書き込み越えが発生する
// (argb 系はそもそも Coalesce を持たない)。
// height == 0 は検証をスキップして現行どおり C 経由の Err に委ねる。
fn check_yuy2_uyvy_dst_overwrite(
    dst: &[u8],
    dst_stride: usize,
    size: ImageSize,
    function: &'static str,
) -> Result<(), Error> {
    // height == 0 では checked_sub が None になるため検証をスキップする
    let Some(height_minus_1) = size.height.checked_sub(1) else {
        return Ok(());
    };
    let required = if size.width % 2 == 1 {
        // 行単位処理。最終行は width * 2 + 2 バイト書き込む
        let head_size = checked_buf_size(
            dst_stride,
            height_minus_1,
            function,
            "destination buffer size overflow",
        )?;
        let last_row = size
            .width
            .checked_mul(2)
            .and_then(|w2| w2.checked_add(2))
            .ok_or_else(|| Error::with_reason(-1, function, "destination buffer size overflow"))?;
        head_size
            .checked_add(last_row)
            .ok_or_else(|| Error::with_reason(-1, function, "destination buffer size overflow"))?
    } else {
        checked_buf_size(
            dst_stride,
            size.height,
            function,
            "destination buffer size overflow",
        )?
    };
    if dst.len() < required {
        return Err(Error::with_reason(
            -1,
            function,
            "destination buffer too small",
        ));
    }
    Ok(())
}

/// YUY2 から ARGB への変換
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （行合体 (Coalesce) が発動するときは `width * height` が奇数の場合のみ
/// `+2` バイト追加）。
pub fn yuy2_to_argb(
    src: &Yuy2Image<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToARGB")?;
    dst.validate(size, "YUY2ToARGB")?;
    // YUY2ToARGB は行合体 (Coalesce) する: src_stride == width * 2 &&
    // dst_stride == width * 4 && width * height <= INT_MAX (convert_argb.cc)。
    // 合体時の +2 の条件は docstring 参照
    let wh_fits = size
        .width
        .checked_mul(size.height)
        .is_some_and(|wh| wh <= c_int::MAX as usize);
    let coalesce = src.stride == size.width * 2 && dst.stride == size.width * 4 && wh_fits;
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, coalesce, "YUY2ToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （この関数の行合体 (Coalesce) はないため、width 奇数では常に `+2` バイト追加）。
pub fn yuy2_to_i420(
    src: &Yuy2Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToI420")?;
    dst.validate(size, "YUY2ToI420")?;
    // YUY2ToI420 は行合体 (Coalesce) を持たず 2 行単位ループのため (convert.cc)、
    // width 奇数で常に最終行の読み越しが発生する
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, false, "YUY2ToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （行合体 (Coalesce) は偶数幅でのみ発動し、合体後も `width * height` が偶数になる
/// ため `+2` バイトは発生しない）。
pub fn yuy2_to_i422(
    src: &Yuy2Image<'_>,
    dst: &mut I422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToI422")?;
    dst.validate(size, "YUY2ToI422")?;
    // YUY2ToI422 の行合体 (Coalesce) 条件は dst_stride_u * 2 == width を含むため
    // 偶数幅でのみ発動し、合体後も width * height が偶数になって +2 が発生しない
    // (planar_functions.cc。なお合体条件には width * height <= 32768 も含まれる)。
    // したがって行単位処理 (width 奇数で +2) のみ考慮すればよい
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, false, "YUY2ToI422")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （この関数の行合体 (Coalesce) はないため、width 奇数では常に `+2` バイト追加）。
pub fn yuy2_to_nv12(
    src: &Yuy2Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToNV12")?;
    dst.validate(size, "YUY2ToNV12")?;
    // YUY2ToNV12 は行合体 (Coalesce) を持たず 2 行単位ループのため (planar_functions.cc)、
    // width 奇数で常に最終行の読み越しが発生する
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, false, "YUY2ToNV12")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// `dst_stride_y` は `size.width` 以上である必要がある（libyuv は 1 行あたり
/// `width` バイトを書き込むため）。
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （行合体 (Coalesce) が発動するときは `width * height` が奇数の場合のみ
/// `+2` バイト追加）。
pub fn yuy2_to_y(
    src: &Yuy2Image<'_>,
    dst_y: &mut [u8],
    dst_stride_y: usize,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "YUY2ToY")?;

    // c_int 範囲チェック（width / height は src.validate が検査済み）
    require_c_int(
        dst_stride_y,
        "YUY2ToY",
        "destination stride exceeds c_int range",
    )?;

    // stride >= width チェック
    if dst_stride_y < size.width {
        return Err(Error::with_reason(
            -1,
            "YUY2ToY",
            "destination stride smaller than width",
        ));
    }

    // バッファサイズ検証（オーバーフロー安全）
    let dst_size = checked_buf_size(
        dst_stride_y,
        size.height,
        "YUY2ToY",
        "destination buffer size overflow",
    )?;
    if dst_y.len() < dst_size {
        return Err(Error::with_reason(
            -1,
            "YUY2ToY",
            "destination Y buffer too small",
        ));
    }

    // YUY2ToY は行合体 (Coalesce) する: src_stride == width * 2 &&
    // dst_stride_y == width && width * height <= INT_MAX (planar_functions.cc)。
    // 合体時の +2 の条件は docstring 参照
    let wh_fits = size
        .width
        .checked_mul(size.height)
        .is_some_and(|wh| wh <= c_int::MAX as usize);
    let coalesce = src.stride == size.width * 2 && dst_stride_y == size.width && wh_fits;
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, coalesce, "YUY2ToY")?;

    // SAFETY: src は .validate()、dst は上記のインライン検証で全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行に `width * 2 + 2` バイト書き込む
/// ため、デスティネーションバッファは `stride * (height - 1) + width * 2 + 2` バイト
/// 以上必要。
pub fn argb_to_yuy2(
    src: &ArgbImage<'_>,
    dst: &mut Yuy2ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToYUY2")?;
    dst.validate(size, "ARGBToYUY2")?;
    // ARGBToYUY2 は行合体 (Coalesce) を持たず、デスティネーション書き込みに
    // I422ToYUY2Row_Any_* を使うため (convert_from_argb.cc)、width 奇数で最終行の
    // 書き込み越えが発生する
    check_yuy2_uyvy_dst_overwrite(dst.data, dst.stride, size, "ARGBToYUY2")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行に `width * 2 + 2` バイト書き込む
/// ため、デスティネーションバッファは `stride * (height - 1) + width * 2 + 2` バイト
/// 以上必要。
pub fn i420_to_yuy2(
    src: &I420Image<'_>,
    dst: &mut Yuy2ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToYUY2")?;
    dst.validate(size, "I420ToYUY2")?;
    // I420ToYUY2 は行合体 (Coalesce) を持たず 2 行単位ループのため (convert_from.cc)、
    // width 奇数で最終行の書き込み越えが発生する
    check_yuy2_uyvy_dst_overwrite(dst.data, dst.stride, size, "I420ToYUY2")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行に `width * 2 + 2` バイト書き込む
/// ため、デスティネーションバッファは `stride * (height - 1) + width * 2 + 2` バイト
/// 以上必要。
pub fn i422_to_yuy2(
    src: &I422Image<'_>,
    dst: &mut Yuy2ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToYUY2")?;
    dst.validate(size, "I422ToYUY2")?;
    // I422ToYUY2 の行合体 (Coalesce) は src_stride_u * 2 == width の条件により
    // 偶数幅でのみ発動するため (convert_from.cc)、width 奇数では最終行の書き込み
    // 越えが発生する
    check_yuy2_uyvy_dst_overwrite(dst.data, dst.stride, size, "I422ToYUY2")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （行合体 (Coalesce) が発動するときは `width * height` が奇数の場合のみ
/// `+2` バイト追加）。
pub fn uyvy_to_argb(
    src: &UyvyImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToARGB")?;
    dst.validate(size, "UYVYToARGB")?;
    // UYVYToARGB は行合体 (Coalesce) する: src_stride == width * 2 &&
    // dst_stride == width * 4 && width * height <= INT_MAX (convert_argb.cc)。
    // 合体時の +2 の条件は docstring 参照
    let wh_fits = size
        .width
        .checked_mul(size.height)
        .is_some_and(|wh| wh <= c_int::MAX as usize);
    let coalesce = src.stride == size.width * 2 && dst.stride == size.width * 4 && wh_fits;
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, coalesce, "UYVYToARGB")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （この関数の行合体 (Coalesce) はないため、width 奇数では常に `+2` バイト追加）。
pub fn uyvy_to_i420(
    src: &UyvyImage<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToI420")?;
    dst.validate(size, "UYVYToI420")?;
    // UYVYToI420 は行合体 (Coalesce) を持たず 2 行単位ループのため (convert.cc)、
    // width 奇数で常に最終行の読み越しが発生する
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, false, "UYVYToI420")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （行合体 (Coalesce) は偶数幅でのみ発動し、合体後も `width * height` が偶数になる
/// ため `+2` バイトは発生しない）。
pub fn uyvy_to_i422(
    src: &UyvyImage<'_>,
    dst: &mut I422ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToI422")?;
    dst.validate(size, "UYVYToI422")?;
    // UYVYToI422 の行合体 (Coalesce) 条件は dst_stride_u * 2 == width を含むため
    // 偶数幅でのみ発動し、合体後も width * height が偶数になって +2 が発生しない
    // (planar_functions.cc。なお合体条件には width * height <= 32768 も含まれる)。
    // したがって行単位処理 (width 奇数で +2) のみ考慮すればよい
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, false, "UYVYToI422")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （この関数の行合体 (Coalesce) はないため、width 奇数では常に `+2` バイト追加）。
pub fn uyvy_to_nv12(
    src: &UyvyImage<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToNV12")?;
    dst.validate(size, "UYVYToNV12")?;
    // UYVYToNV12 は行合体 (Coalesce) を持たず 2 行単位ループのため (planar_functions.cc)、
    // width 奇数で常に最終行の読み越しが発生する
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, false, "UYVYToNV12")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// `dst_stride_y` は `size.width` 以上である必要がある（libyuv は 1 行あたり
/// `width` バイトを書き込むため）。
///
/// width 奇数では libyuv の行関数ラッパーが最終行を `width * 2 + 2` バイト読み出す
/// ため、ソースバッファは `stride * (height - 1) + width * 2 + 2` バイト以上必要
/// （行合体 (Coalesce) が発動するときは `width * height` が奇数の場合のみ
/// `+2` バイト追加）。
pub fn uyvy_to_y(
    src: &UyvyImage<'_>,
    dst_y: &mut [u8],
    dst_stride_y: usize,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "UYVYToY")?;

    // c_int 範囲チェック（width / height は src.validate が検査済み）
    require_c_int(
        dst_stride_y,
        "UYVYToY",
        "destination stride exceeds c_int range",
    )?;

    // stride >= width チェック
    if dst_stride_y < size.width {
        return Err(Error::with_reason(
            -1,
            "UYVYToY",
            "destination stride smaller than width",
        ));
    }

    // バッファサイズ検証（オーバーフロー安全）
    let dst_size = checked_buf_size(
        dst_stride_y,
        size.height,
        "UYVYToY",
        "destination buffer size overflow",
    )?;
    if dst_y.len() < dst_size {
        return Err(Error::with_reason(
            -1,
            "UYVYToY",
            "destination Y buffer too small",
        ));
    }

    // UYVYToY は行合体 (Coalesce) する: src_stride == width * 2 &&
    // dst_stride_y == width && width * height <= INT_MAX (planar_functions.cc)。
    // 合体時の +2 の条件は docstring 参照
    let wh_fits = size
        .width
        .checked_mul(size.height)
        .is_some_and(|wh| wh <= c_int::MAX as usize);
    let coalesce = src.stride == size.width * 2 && dst_stride_y == size.width && wh_fits;
    check_yuy2_uyvy_src_overread(src.data, src.stride, size, coalesce, "UYVYToY")?;

    // SAFETY: src は .validate()、dst は上記のインライン検証で全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行に `width * 2 + 2` バイト書き込む
/// ため、デスティネーションバッファは `stride * (height - 1) + width * 2 + 2` バイト
/// 以上必要。
pub fn argb_to_uyvy(
    src: &ArgbImage<'_>,
    dst: &mut UyvyImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBToUYVY")?;
    dst.validate(size, "ARGBToUYVY")?;
    // ARGBToUYVY は行合体 (Coalesce) を持たず、デスティネーション書き込みに
    // I422ToUYVYRow_Any_* を使うため (convert_from_argb.cc)、width 奇数で最終行の
    // 書き込み越えが発生する
    check_yuy2_uyvy_dst_overwrite(dst.data, dst.stride, size, "ARGBToUYVY")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行に `width * 2 + 2` バイト書き込む
/// ため、デスティネーションバッファは `stride * (height - 1) + width * 2 + 2` バイト
/// 以上必要。
pub fn i420_to_uyvy(
    src: &I420Image<'_>,
    dst: &mut UyvyImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420ToUYVY")?;
    dst.validate(size, "I420ToUYVY")?;
    // I420ToUYVY は行合体 (Coalesce) を持たず 2 行単位ループのため (convert_from.cc)、
    // width 奇数で最終行の書き込み越えが発生する
    check_yuy2_uyvy_dst_overwrite(dst.data, dst.stride, size, "I420ToUYVY")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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
///
/// width 奇数では libyuv の行関数ラッパーが最終行に `width * 2 + 2` バイト書き込む
/// ため、デスティネーションバッファは `stride * (height - 1) + width * 2 + 2` バイト
/// 以上必要。
pub fn i422_to_uyvy(
    src: &I422Image<'_>,
    dst: &mut UyvyImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I422ToUYVY")?;
    dst.validate(size, "I422ToUYVY")?;
    // I422ToUYVY の行合体 (Coalesce) は src_stride_u * 2 == width の条件により
    // 偶数幅でのみ発動するため (convert_from.cc)、width 奇数では最終行の書き込み
    // 越えが発生する
    check_yuy2_uyvy_dst_overwrite(dst.data, dst.stride, size, "I422ToUYVY")?;

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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

    // SAFETY: .validate() が全前提条件を検査済み。
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
