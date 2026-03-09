//! プレーン操作関数
#![allow(clippy::too_many_arguments)]

use std::ffi::c_int;

use crate::{
    ArgbImage, ArgbImageMut, Error, I400Image, I400ImageMut, I420Image, I420ImageMut, ImageSize,
    Nv12Image, Nv12ImageMut, Rgb24Image, Rgb24ImageMut, sys,
};

// ============================================================
// プレーンのコピーと塗りつぶし
// ============================================================

/// プレーンのコピー
pub fn copy_plane(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_stride: usize,
    size: ImageSize,
) -> Result<(), Error> {
    let src_required_size = if size.height > 0 {
        (size.height - 1) * src_stride + size.width
    } else {
        0
    };

    let dst_required_size = if size.height > 0 {
        (size.height - 1) * dst_stride + size.width
    } else {
        0
    };

    if src.len() < src_required_size {
        return Err(Error::with_reason(
            -1,
            "CopyPlane",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_required_size {
        return Err(Error::with_reason(
            -1,
            "CopyPlane",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::CopyPlane(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// プレーンを指定値で塗りつぶし
pub fn set_plane(
    dst: &mut [u8],
    dst_stride: usize,
    size: ImageSize,
    value: u8,
) -> Result<(), Error> {
    let dst_size = dst_stride * size.height;

    if dst.len() < dst_size {
        return Err(Error::with_reason(
            -1,
            "SetPlane",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::SetPlane(
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
            value as u32,
        )
    };

    Ok(())
}

/// I400 (グレースケール) 画像のコピー
pub fn i400_copy(
    src: &I400Image<'_>,
    dst: &mut I400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I400Copy")?;
    dst.validate(size, "I400Copy")?;

    let result = unsafe {
        sys::I400Copy(
            src.y.as_ptr(),
            src.y_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I400Copy")
}

// ============================================================
// UV プレーンの分割・結合
// ============================================================

/// インターリーブ UV プレーンを U と V に分割する
pub fn split_uv_plane(
    src_uv: &[u8],
    src_stride_uv: usize,
    dst_u: &mut [u8],
    dst_stride_u: usize,
    dst_v: &mut [u8],
    dst_stride_v: usize,
    size: ImageSize,
) -> Result<(), Error> {
    if src_uv.len() < src_stride_uv * size.height {
        return Err(Error::with_reason(
            -1,
            "SplitUVPlane",
            "source UV buffer too small",
        ));
    }
    if dst_u.len() < dst_stride_u * size.height {
        return Err(Error::with_reason(
            -1,
            "SplitUVPlane",
            "destination U buffer too small",
        ));
    }
    if dst_v.len() < dst_stride_v * size.height {
        return Err(Error::with_reason(
            -1,
            "SplitUVPlane",
            "destination V buffer too small",
        ));
    }

    unsafe {
        sys::SplitUVPlane(
            src_uv.as_ptr(),
            src_stride_uv as c_int,
            dst_u.as_mut_ptr(),
            dst_stride_u as c_int,
            dst_v.as_mut_ptr(),
            dst_stride_v as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// 個別の U と V プレーンをインターリーブ UV に結合する
pub fn merge_uv_plane(
    src_u: &[u8],
    src_stride_u: usize,
    src_v: &[u8],
    src_stride_v: usize,
    dst_uv: &mut [u8],
    dst_stride_uv: usize,
    size: ImageSize,
) -> Result<(), Error> {
    if src_u.len() < src_stride_u * size.height {
        return Err(Error::with_reason(
            -1,
            "MergeUVPlane",
            "source U buffer too small",
        ));
    }
    if src_v.len() < src_stride_v * size.height {
        return Err(Error::with_reason(
            -1,
            "MergeUVPlane",
            "source V buffer too small",
        ));
    }
    if dst_uv.len() < dst_stride_uv * size.height {
        return Err(Error::with_reason(
            -1,
            "MergeUVPlane",
            "destination UV buffer too small",
        ));
    }

    unsafe {
        sys::MergeUVPlane(
            src_u.as_ptr(),
            src_stride_u as c_int,
            src_v.as_ptr(),
            src_stride_v as c_int,
            dst_uv.as_mut_ptr(),
            dst_stride_uv as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// UV プレーンの U と V を入れ替える
pub fn swap_uv_plane(
    src_uv: &[u8],
    src_stride_uv: usize,
    dst_vu: &mut [u8],
    dst_stride_vu: usize,
    size: ImageSize,
) -> Result<(), Error> {
    if src_uv.len() < src_stride_uv * size.height {
        return Err(Error::with_reason(
            -1,
            "SwapUVPlane",
            "source UV buffer too small",
        ));
    }
    if dst_vu.len() < dst_stride_vu * size.height {
        return Err(Error::with_reason(
            -1,
            "SwapUVPlane",
            "destination VU buffer too small",
        ));
    }

    unsafe {
        sys::SwapUVPlane(
            src_uv.as_ptr(),
            src_stride_uv as c_int,
            dst_vu.as_mut_ptr(),
            dst_stride_vu as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

// ============================================================
// RGB プレーンの分割・結合
// ============================================================

/// RGB プレーンを R, G, B に分割する
///
/// width は 32 の倍数である必要がある。
/// libyuv 内部の AVX2 端数処理 (ANY13 マクロ) にバッファオーバーフローのバグがあるため、
/// 32 の倍数以外の width はサポートしない。
pub fn split_rgb_plane(
    src: &Rgb24Image<'_>,
    dst_r: &mut [u8],
    dst_stride_r: usize,
    dst_g: &mut [u8],
    dst_stride_g: usize,
    dst_b: &mut [u8],
    dst_stride_b: usize,
    size: ImageSize,
) -> Result<(), Error> {
    if !size.width.is_multiple_of(32) {
        return Err(Error::with_reason(
            -1,
            "SplitRGBPlane",
            "width must be a multiple of 32",
        ));
    }
    src.validate(size, "SplitRGBPlane")?;
    if dst_r.len() < dst_stride_r * size.height {
        return Err(Error::with_reason(
            -1,
            "SplitRGBPlane",
            "destination R buffer too small",
        ));
    }
    if dst_g.len() < dst_stride_g * size.height {
        return Err(Error::with_reason(
            -1,
            "SplitRGBPlane",
            "destination G buffer too small",
        ));
    }
    if dst_b.len() < dst_stride_b * size.height {
        return Err(Error::with_reason(
            -1,
            "SplitRGBPlane",
            "destination B buffer too small",
        ));
    }

    unsafe {
        sys::SplitRGBPlane(
            src.data.as_ptr(),
            src.stride as c_int,
            dst_r.as_mut_ptr(),
            dst_stride_r as c_int,
            dst_g.as_mut_ptr(),
            dst_stride_g as c_int,
            dst_b.as_mut_ptr(),
            dst_stride_b as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// R, G, B プレーンを RGB に結合する
pub fn merge_rgb_plane(
    src_r: &[u8],
    src_stride_r: usize,
    src_g: &[u8],
    src_stride_g: usize,
    src_b: &[u8],
    src_stride_b: usize,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    if src_r.len() < src_stride_r * size.height {
        return Err(Error::with_reason(
            -1,
            "MergeRGBPlane",
            "source R buffer too small",
        ));
    }
    if src_g.len() < src_stride_g * size.height {
        return Err(Error::with_reason(
            -1,
            "MergeRGBPlane",
            "source G buffer too small",
        ));
    }
    if src_b.len() < src_stride_b * size.height {
        return Err(Error::with_reason(
            -1,
            "MergeRGBPlane",
            "source B buffer too small",
        ));
    }
    dst.validate(size, "MergeRGBPlane")?;

    unsafe {
        sys::MergeRGBPlane(
            src_r.as_ptr(),
            src_stride_r as c_int,
            src_g.as_ptr(),
            src_stride_g as c_int,
            src_b.as_ptr(),
            src_stride_b as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

// ============================================================
// ミラー（左右反転）
// ============================================================

/// I400 (グレースケール) 画像の左右反転
pub fn i400_mirror(
    src: &I400Image<'_>,
    dst: &mut I400ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I400Mirror")?;
    dst.validate(size, "I400Mirror")?;

    let result = unsafe {
        sys::I400Mirror(
            src.y.as_ptr(),
            src.y_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "I400Mirror")
}

/// I420 画像の左右反転
pub fn i420_mirror(
    src: &I420Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "I420Mirror")?;
    dst.validate(size, "I420Mirror")?;

    let result = unsafe {
        sys::I420Mirror(
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

    Error::check(result, "I420Mirror")
}

/// NV12 画像の左右反転
pub fn nv12_mirror(
    src: &Nv12Image<'_>,
    dst: &mut Nv12ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "NV12Mirror")?;
    dst.validate(size, "NV12Mirror")?;

    let result = unsafe {
        sys::NV12Mirror(
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

    Error::check(result, "NV12Mirror")
}

/// ARGB 画像の左右反転
pub fn argb_mirror(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBMirror")?;
    dst.validate(size, "ARGBMirror")?;

    let result = unsafe {
        sys::ARGBMirror(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBMirror")
}

/// RGB24 画像の左右反転
pub fn rgb24_mirror(
    src: &Rgb24Image<'_>,
    dst: &mut Rgb24ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "RGB24Mirror")?;
    dst.validate(size, "RGB24Mirror")?;

    let result = unsafe {
        sys::RGB24Mirror(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RGB24Mirror")
}

/// 単一プレーンの左右反転
pub fn mirror_plane(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_stride: usize,
    size: ImageSize,
) {
    unsafe {
        sys::MirrorPlane(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };
}

// ============================================================
// ブレンド
// ============================================================

/// I420 画像のアルファブレンド
///
/// dst = src0 * alpha + src1 * (1 - alpha)
pub fn i420_blend(
    src0: &I420Image<'_>,
    src1: &I420Image<'_>,
    alpha: &[u8],
    alpha_stride: usize,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src0.validate(size, "I420Blend")?;
    src1.validate(size, "I420Blend")?;
    dst.validate(size, "I420Blend")?;

    if alpha.len() < alpha_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "I420Blend",
            "alpha buffer too small",
        ));
    }

    let result = unsafe {
        sys::I420Blend(
            src0.y.as_ptr(),
            src0.y_stride as c_int,
            src0.u.as_ptr(),
            src0.u_stride as c_int,
            src0.v.as_ptr(),
            src0.v_stride as c_int,
            src1.y.as_ptr(),
            src1.y_stride as c_int,
            src1.u.as_ptr(),
            src1.u_stride as c_int,
            src1.v.as_ptr(),
            src1.v_stride as c_int,
            alpha.as_ptr(),
            alpha_stride as c_int,
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

    Error::check(result, "I420Blend")
}

/// ARGB 画像のアルファブレンド
///
/// dst = src0 * src0.alpha + src1 * (1 - src0.alpha)
pub fn argb_blend(
    src0: &ArgbImage<'_>,
    src1: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src0.validate(size, "ARGBBlend")?;
    src1.validate(size, "ARGBBlend")?;
    dst.validate(size, "ARGBBlend")?;

    let result = unsafe {
        sys::ARGBBlend(
            src0.data.as_ptr(),
            src0.stride as c_int,
            src1.data.as_ptr(),
            src1.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBBlend")
}

/// 単一プレーンのブレンド
///
/// dst = src0 * alpha / 256 + src1 * (256 - alpha) / 256
pub fn blend_plane(
    src0: &[u8],
    src0_stride: usize,
    src1: &[u8],
    src1_stride: usize,
    alpha: &[u8],
    alpha_stride: usize,
    dst: &mut [u8],
    dst_stride: usize,
    size: ImageSize,
) -> Result<(), Error> {
    let required = |stride: usize| stride * size.height;
    if src0.len() < required(src0_stride) {
        return Err(Error::with_reason(
            -1,
            "BlendPlane",
            "source 0 buffer too small",
        ));
    }
    if src1.len() < required(src1_stride) {
        return Err(Error::with_reason(
            -1,
            "BlendPlane",
            "source 1 buffer too small",
        ));
    }
    if alpha.len() < required(alpha_stride) {
        return Err(Error::with_reason(
            -1,
            "BlendPlane",
            "alpha buffer too small",
        ));
    }
    if dst.len() < required(dst_stride) {
        return Err(Error::with_reason(
            -1,
            "BlendPlane",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::BlendPlane(
            src0.as_ptr(),
            src0_stride as c_int,
            src1.as_ptr(),
            src1_stride as c_int,
            alpha.as_ptr(),
            alpha_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "BlendPlane")
}

// ============================================================
// プレーン補間
// ============================================================

/// 2 つのプレーンを補間する
///
/// interpolation: 0 なら src0、255 なら src1、128 なら半々
pub fn interpolate_plane(
    src0: &[u8],
    src0_stride: usize,
    src1: &[u8],
    src1_stride: usize,
    dst: &mut [u8],
    dst_stride: usize,
    size: ImageSize,
    interpolation: u8,
) -> Result<(), Error> {
    let required = |stride: usize| stride * size.height;
    if src0.len() < required(src0_stride) {
        return Err(Error::with_reason(
            -1,
            "InterpolatePlane",
            "source 0 buffer too small",
        ));
    }
    if src1.len() < required(src1_stride) {
        return Err(Error::with_reason(
            -1,
            "InterpolatePlane",
            "source 1 buffer too small",
        ));
    }
    if dst.len() < required(dst_stride) {
        return Err(Error::with_reason(
            -1,
            "InterpolatePlane",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::InterpolatePlane(
            src0.as_ptr(),
            src0_stride as c_int,
            src1.as_ptr(),
            src1_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
            interpolation as c_int,
        )
    };

    Ok(())
}

// ============================================================
// ARGB 加工
// ============================================================

/// ARGB 画像のアルファ値を事前乗算する（プリマルチプライドアルファ）
pub fn argb_attenuate(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBAttenuate")?;
    dst.validate(size, "ARGBAttenuate")?;

    let result = unsafe {
        sys::ARGBAttenuate(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBAttenuate")
}

/// ARGB 画像の事前乗算アルファを元に戻す
pub fn argb_unattenuate(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBUnattenuate")?;
    dst.validate(size, "ARGBUnattenuate")?;

    let result = unsafe {
        sys::ARGBUnattenuate(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBUnattenuate")
}

/// ARGB 画像の色を指定色で暗くする（シェーディング）
///
/// `shade_value` は ARGB 形式の u32 で、各チャンネルの乗算値を指定する
pub fn argb_shade(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    shade_value: u32,
) -> Result<(), Error> {
    src.validate(size, "ARGBShade")?;
    dst.validate(size, "ARGBShade")?;

    let result = unsafe {
        sys::ARGBShade(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
            shade_value,
        )
    };

    Error::check(result, "ARGBShade")
}

/// ARGB 画像をグレースケールに変換する（インプレース可能）
pub fn argb_gray(dst: &mut ArgbImageMut<'_>, size: ImageSize) -> Result<(), Error> {
    dst.validate(size, "ARGBGray")?;

    let result = unsafe {
        sys::ARGBGray(
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            0,
            0,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBGray")
}

/// ARGB 画像にセピア調の効果を適用する（インプレース可能）
pub fn argb_sepia(dst: &mut ArgbImageMut<'_>, size: ImageSize) -> Result<(), Error> {
    dst.validate(size, "ARGBSepia")?;

    let result = unsafe {
        sys::ARGBSepia(
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            0,
            0,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBSepia")
}

// ============================================================
// I420 矩形塗りつぶし
// ============================================================

/// I420 画像に矩形を描画する
pub fn i420_rect(
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
    x: usize,
    y: usize,
    rect_width: usize,
    rect_height: usize,
    value_y: u8,
    value_u: u8,
    value_v: u8,
) -> Result<(), Error> {
    dst.validate(size, "I420Rect")?;

    let result = unsafe {
        sys::I420Rect(
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            x as c_int,
            y as c_int,
            rect_width as c_int,
            rect_height as c_int,
            value_y as c_int,
            value_u as c_int,
            value_v as c_int,
        )
    };

    Error::check(result, "I420Rect")
}

/// ARGB 画像に矩形を描画する
pub fn argb_rect(
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    x: usize,
    y: usize,
    rect_width: usize,
    rect_height: usize,
    argb_color: u32,
) -> Result<(), Error> {
    dst.validate(size, "ARGBRect")?;

    let result = unsafe {
        sys::ARGBRect(
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            x as c_int,
            y as c_int,
            rect_width as c_int,
            rect_height as c_int,
            argb_color,
        )
    };

    Error::check(result, "ARGBRect")
}

// ============================================================
// I420 / ARGB 補間
// ============================================================

/// 2 つの I420 画像を補間する
///
/// `interpolation`: 0 なら src0、255 なら src1、128 なら半々
pub fn i420_interpolate(
    src0: &I420Image<'_>,
    src1: &I420Image<'_>,
    dst: &mut I420ImageMut<'_>,
    size: ImageSize,
    interpolation: u8,
) -> Result<(), Error> {
    src0.validate(size, "I420Interpolate")?;
    src1.validate(size, "I420Interpolate")?;
    dst.validate(size, "I420Interpolate")?;

    let result = unsafe {
        sys::I420Interpolate(
            src0.y.as_ptr(),
            src0.y_stride as c_int,
            src0.u.as_ptr(),
            src0.u_stride as c_int,
            src0.v.as_ptr(),
            src0.v_stride as c_int,
            src1.y.as_ptr(),
            src1.y_stride as c_int,
            src1.u.as_ptr(),
            src1.u_stride as c_int,
            src1.v.as_ptr(),
            src1.v_stride as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            size.width as c_int,
            size.height as c_int,
            interpolation as c_int,
        )
    };

    Error::check(result, "I420Interpolate")
}

/// 2 つの ARGB 画像を補間する
///
/// `interpolation`: 0 なら src0、255 なら src1、128 なら半々
pub fn argb_interpolate(
    src0: &ArgbImage<'_>,
    src1: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    interpolation: u8,
) -> Result<(), Error> {
    src0.validate(size, "ARGBInterpolate")?;
    src1.validate(size, "ARGBInterpolate")?;
    dst.validate(size, "ARGBInterpolate")?;

    let result = unsafe {
        sys::ARGBInterpolate(
            src0.data.as_ptr(),
            src0.stride as c_int,
            src1.data.as_ptr(),
            src1.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
            interpolation as c_int,
        )
    };

    Error::check(result, "ARGBInterpolate")
}

/// 2 つの 16bit プレーンを補間する
///
/// `interpolation`: 0 なら src0、255 なら src1、128 なら半々
pub fn interpolate_plane_16(
    src0: &[u16],
    src0_stride: usize,
    src1: &[u16],
    src1_stride: usize,
    dst: &mut [u16],
    dst_stride: usize,
    size: ImageSize,
    interpolation: u8,
) -> Result<(), Error> {
    let required = |stride: usize| stride * size.height;
    if src0.len() < required(src0_stride) {
        return Err(Error::with_reason(
            -1,
            "InterpolatePlane_16",
            "source 0 buffer too small",
        ));
    }
    if src1.len() < required(src1_stride) {
        return Err(Error::with_reason(
            -1,
            "InterpolatePlane_16",
            "source 1 buffer too small",
        ));
    }
    if dst.len() < required(dst_stride) {
        return Err(Error::with_reason(
            -1,
            "InterpolatePlane_16",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::InterpolatePlane_16(
            src0.as_ptr(),
            src0_stride as c_int,
            src1.as_ptr(),
            src1_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
            interpolation as c_int,
        )
    };

    Ok(())
}

// ============================================================
// エッジ検出（Sobel フィルタ）
// ============================================================

/// ARGB 画像に Sobel エッジ検出フィルタを適用する
pub fn argb_sobel(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBSobel")?;
    dst.validate(size, "ARGBSobel")?;

    let result = unsafe {
        sys::ARGBSobel(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBSobel")
}

/// ARGB 画像に Sobel エッジ検出フィルタを適用し、結果をプレーンに出力する
pub fn argb_sobel_to_plane(
    src: &ArgbImage<'_>,
    dst: &mut [u8],
    dst_stride: usize,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBSobelToPlane")?;
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "ARGBSobelToPlane",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::ARGBSobelToPlane(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBSobelToPlane")
}

/// ARGB 画像に Sobel XY エッジ検出フィルタを適用する
pub fn argb_sobel_xy(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBSobelXY")?;
    dst.validate(size, "ARGBSobelXY")?;

    let result = unsafe {
        sys::ARGBSobelXY(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBSobelXY")
}

// ============================================================
// ARGB カラー変換・行列演算
// ============================================================

/// ARGB 画像にカラーマトリックスを適用する
///
/// `matrix_argb` は 4x4 の変換行列（行優先、16 要素）
pub fn argb_color_matrix(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    matrix_argb: &[i8; 16],
) -> Result<(), Error> {
    src.validate(size, "ARGBColorMatrix")?;
    dst.validate(size, "ARGBColorMatrix")?;

    let result = unsafe {
        sys::ARGBColorMatrix(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            matrix_argb.as_ptr(),
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBColorMatrix")
}

/// RGB カラーマトリックスをインプレースで適用する
///
/// `matrix_rgb` は 3x3 の変換行列（9 要素）
pub fn rgb_color_matrix(
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    matrix_rgb: &[i8; 9],
) -> Result<(), Error> {
    dst.validate(size, "RGBColorMatrix")?;

    let result = unsafe {
        sys::RGBColorMatrix(
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            matrix_rgb.as_ptr(),
            0,
            0,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RGBColorMatrix")
}

/// ARGB 画像に多項式変換を適用する
///
/// `poly` は各チャンネルに対する 4 次多項式の係数（最低 16 要素）
pub fn argb_polynomial(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    poly: &[f32],
) -> Result<(), Error> {
    if poly.len() < 16 {
        return Err(Error::with_reason(
            -1,
            "ARGBPolynomial",
            "poly must have at least 16 elements",
        ));
    }
    src.validate(size, "ARGBPolynomial")?;
    dst.validate(size, "ARGBPolynomial")?;

    let result = unsafe {
        sys::ARGBPolynomial(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            poly.as_ptr(),
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBPolynomial")
}

// ============================================================
// ARGB 算術演算
// ============================================================

/// 2 つの ARGB 画像をピクセル単位で加算する
pub fn argb_add(
    src0: &ArgbImage<'_>,
    src1: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src0.validate(size, "ARGBAdd")?;
    src1.validate(size, "ARGBAdd")?;
    dst.validate(size, "ARGBAdd")?;

    let result = unsafe {
        sys::ARGBAdd(
            src0.data.as_ptr(),
            src0.stride as c_int,
            src1.data.as_ptr(),
            src1.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBAdd")
}

/// 2 つの ARGB 画像をピクセル単位で減算する
pub fn argb_subtract(
    src0: &ArgbImage<'_>,
    src1: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src0.validate(size, "ARGBSubtract")?;
    src1.validate(size, "ARGBSubtract")?;
    dst.validate(size, "ARGBSubtract")?;

    let result = unsafe {
        sys::ARGBSubtract(
            src0.data.as_ptr(),
            src0.stride as c_int,
            src1.data.as_ptr(),
            src1.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBSubtract")
}

/// 2 つの ARGB 画像をピクセル単位で乗算する
pub fn argb_multiply(
    src0: &ArgbImage<'_>,
    src1: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src0.validate(size, "ARGBMultiply")?;
    src1.validate(size, "ARGBMultiply")?;
    dst.validate(size, "ARGBMultiply")?;

    let result = unsafe {
        sys::ARGBMultiply(
            src0.data.as_ptr(),
            src0.stride as c_int,
            src1.data.as_ptr(),
            src1.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBMultiply")
}

// ============================================================
// ARGB ブラー・累積和
// ============================================================

/// ARGB 画像にボックスブラーを適用する
///
/// `cumsum` は累積和計算用のバッファ。`stride32_cumsum` は i32 単位のストライド。
/// `radius` はブラーの半径。
pub fn argb_blur(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    cumsum: &mut [i32],
    stride32_cumsum: usize,
    radius: i32,
) -> Result<(), Error> {
    src.validate(size, "ARGBBlur")?;
    dst.validate(size, "ARGBBlur")?;
    if cumsum.len() < stride32_cumsum * size.height {
        return Err(Error::with_reason(
            -1,
            "ARGBBlur",
            "cumsum buffer too small",
        ));
    }

    let result = unsafe {
        sys::ARGBBlur(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            cumsum.as_mut_ptr(),
            stride32_cumsum as c_int,
            size.width as c_int,
            size.height as c_int,
            radius as c_int,
        )
    };

    Error::check(result, "ARGBBlur")
}

/// ARGB 画像の累積和を計算する
pub fn argb_compute_cumulative_sum(
    src: &ArgbImage<'_>,
    dst_cumsum: &mut [i32],
    dst_stride32_cumsum: usize,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBComputeCumulativeSum")?;
    if dst_cumsum.len() < dst_stride32_cumsum * size.height {
        return Err(Error::with_reason(
            -1,
            "ARGBComputeCumulativeSum",
            "cumsum buffer too small",
        ));
    }

    let result = unsafe {
        sys::ARGBComputeCumulativeSum(
            src.data.as_ptr(),
            src.stride as c_int,
            dst_cumsum.as_mut_ptr(),
            dst_stride32_cumsum as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBComputeCumulativeSum")
}

// ============================================================
// ARGB グレースケール・量子化・色テーブル
// ============================================================

/// ARGB 画像をグレースケールに変換する（src と dst が別）
pub fn argb_gray_to(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBGrayTo")?;
    dst.validate(size, "ARGBGrayTo")?;

    let result = unsafe {
        sys::ARGBGrayTo(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBGrayTo")
}

/// ARGB 画像をインプレースで量子化する
///
/// `scale` は量子化スケール、`interval_size` は量子化間隔、
/// `interval_offset` は量子化オフセット
pub fn argb_quantize(
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    scale: i32,
    interval_size: i32,
    interval_offset: i32,
) -> Result<(), Error> {
    dst.validate(size, "ARGBQuantize")?;

    let result = unsafe {
        sys::ARGBQuantize(
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            scale as c_int,
            interval_size as c_int,
            interval_offset as c_int,
            0,
            0,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBQuantize")
}

/// ARGB 画像に輝度ベースのカラーテーブルを適用する
///
/// `luma` は 256 * 4 要素のルックアップテーブル
pub fn argb_luma_color_table(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    luma: &[u8],
) -> Result<(), Error> {
    if luma.len() < 256 * 4 {
        return Err(Error::with_reason(
            -1,
            "ARGBLumaColorTable",
            "luma table must have at least 1024 elements",
        ));
    }
    src.validate(size, "ARGBLumaColorTable")?;
    dst.validate(size, "ARGBLumaColorTable")?;

    let result = unsafe {
        sys::ARGBLumaColorTable(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            luma.as_ptr(),
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBLumaColorTable")
}

/// ARGB 画像にカラーテーブルをインプレースで適用する
///
/// `table_argb` は 256 * 4 要素のルックアップテーブル
pub fn argb_color_table(
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    table_argb: &[u8; 256 * 4],
) -> Result<(), Error> {
    dst.validate(size, "ARGBColorTable")?;

    let result = unsafe {
        sys::ARGBColorTable(
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            table_argb.as_ptr(),
            0,
            0,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBColorTable")
}

/// RGB チャンネルにカラーテーブルをインプレースで適用する
///
/// `table_argb` は 256 * 4 要素のルックアップテーブル
pub fn rgb_color_table(
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    table_argb: &[u8; 256 * 4],
) -> Result<(), Error> {
    dst.validate(size, "RGBColorTable")?;

    let result = unsafe {
        sys::RGBColorTable(
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            table_argb.as_ptr(),
            0,
            0,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "RGBColorTable")
}

// ============================================================
// ARGB チャンネル操作
// ============================================================

/// ARGB 画像のチャンネルをシャッフルする
///
/// `shuffler` は出力チャンネルの並び順を指定する 4 要素の配列
pub fn argb_shuffle(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
    shuffler: &[u8; 4],
) -> Result<(), Error> {
    src.validate(size, "ARGBShuffle")?;
    dst.validate(size, "ARGBShuffle")?;

    let result = unsafe {
        sys::ARGBShuffle(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            shuffler.as_ptr(),
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBShuffle")
}

/// ARGB 画像からアルファチャンネルをプレーンとして抽出する
pub fn argb_extract_alpha(
    src: &ArgbImage<'_>,
    dst: &mut [u8],
    dst_stride: usize,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBExtractAlpha")?;
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "ARGBExtractAlpha",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::ARGBExtractAlpha(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBExtractAlpha")
}

/// ARGB 画像のアルファチャンネルのみをコピーする
pub fn argb_copy_alpha(
    src: &ArgbImage<'_>,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    src.validate(size, "ARGBCopyAlpha")?;
    dst.validate(size, "ARGBCopyAlpha")?;

    let result = unsafe {
        sys::ARGBCopyAlpha(
            src.data.as_ptr(),
            src.stride as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBCopyAlpha")
}

/// Y プレーンの値を ARGB 画像のアルファチャンネルにコピーする
pub fn argb_copy_y_to_alpha(
    src_y: &[u8],
    src_stride_y: usize,
    dst: &mut ArgbImageMut<'_>,
    size: ImageSize,
) -> Result<(), Error> {
    if src_y.len() < src_stride_y * size.height {
        return Err(Error::with_reason(
            -1,
            "ARGBCopyYToAlpha",
            "source Y buffer too small",
        ));
    }
    dst.validate(size, "ARGBCopyYToAlpha")?;

    let result = unsafe {
        sys::ARGBCopyYToAlpha(
            src_y.as_ptr(),
            src_stride_y as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "ARGBCopyYToAlpha")
}

// ============================================================
// ARGB フォーマット検出
// ============================================================

/// ARGB 画像のアルファチャンネルの値からフォーマットを検出する
///
/// 戻り値は検出されたフォーマットを表す u32
pub fn argb_detect(src: &ArgbImage<'_>, size: ImageSize) -> Result<u32, Error> {
    src.validate(size, "ARGBDetect")?;

    let result = unsafe {
        sys::ARGBDetect(
            src.data.as_ptr(),
            src.stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(result)
}

// ============================================================
// 16bit プレーンのコピー・分割・結合
// ============================================================

/// 16bit プレーンのコピー
pub fn copy_plane_16(
    src: &[u16],
    src_stride: usize,
    dst: &mut [u16],
    dst_stride: usize,
    size: ImageSize,
) -> Result<(), Error> {
    let src_required = if size.height > 0 {
        (size.height - 1) * src_stride + size.width
    } else {
        0
    };
    let dst_required = if size.height > 0 {
        (size.height - 1) * dst_stride + size.width
    } else {
        0
    };

    if src.len() < src_required {
        return Err(Error::with_reason(
            -1,
            "CopyPlane_16",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_required {
        return Err(Error::with_reason(
            -1,
            "CopyPlane_16",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::CopyPlane_16(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// 16bit U/V プレーンをインターリーブ UV に結合する
pub fn merge_uv_plane_16(
    src_u: &[u16],
    src_stride_u: usize,
    src_v: &[u16],
    src_stride_v: usize,
    dst_uv: &mut [u16],
    dst_stride_uv: usize,
    size: ImageSize,
    depth: i32,
) -> Result<(), Error> {
    if src_u.len() < src_stride_u * size.height {
        return Err(Error::with_reason(
            -1,
            "MergeUVPlane_16",
            "source U buffer too small",
        ));
    }
    if src_v.len() < src_stride_v * size.height {
        return Err(Error::with_reason(
            -1,
            "MergeUVPlane_16",
            "source V buffer too small",
        ));
    }
    if dst_uv.len() < dst_stride_uv * size.height {
        return Err(Error::with_reason(
            -1,
            "MergeUVPlane_16",
            "destination UV buffer too small",
        ));
    }

    unsafe {
        sys::MergeUVPlane_16(
            src_u.as_ptr(),
            src_stride_u as c_int,
            src_v.as_ptr(),
            src_stride_v as c_int,
            dst_uv.as_mut_ptr(),
            dst_stride_uv as c_int,
            size.width as c_int,
            size.height as c_int,
            depth as c_int,
        )
    };

    Ok(())
}

/// 16bit インターリーブ UV プレーンを U と V に分割する
pub fn split_uv_plane_16(
    src_uv: &[u16],
    src_stride_uv: usize,
    dst_u: &mut [u16],
    dst_stride_u: usize,
    dst_v: &mut [u16],
    dst_stride_v: usize,
    size: ImageSize,
    depth: i32,
) -> Result<(), Error> {
    if src_uv.len() < src_stride_uv * size.height {
        return Err(Error::with_reason(
            -1,
            "SplitUVPlane_16",
            "source UV buffer too small",
        ));
    }
    if dst_u.len() < dst_stride_u * size.height {
        return Err(Error::with_reason(
            -1,
            "SplitUVPlane_16",
            "destination U buffer too small",
        ));
    }
    if dst_v.len() < dst_stride_v * size.height {
        return Err(Error::with_reason(
            -1,
            "SplitUVPlane_16",
            "destination V buffer too small",
        ));
    }

    unsafe {
        sys::SplitUVPlane_16(
            src_uv.as_ptr(),
            src_stride_uv as c_int,
            dst_u.as_mut_ptr(),
            dst_stride_u as c_int,
            dst_v.as_mut_ptr(),
            dst_stride_v as c_int,
            size.width as c_int,
            size.height as c_int,
            depth as c_int,
        )
    };

    Ok(())
}

// ============================================================
// ARGB プレーンの分割・結合
// ============================================================

/// R, G, B, A プレーンを ARGB に結合する
pub fn merge_argb_plane(
    src_r: &[u8],
    src_stride_r: usize,
    src_g: &[u8],
    src_stride_g: usize,
    src_b: &[u8],
    src_stride_b: usize,
    src_a: &[u8],
    src_stride_a: usize,
    dst_argb: &mut [u8],
    dst_stride_argb: usize,
    size: ImageSize,
) -> Result<(), Error> {
    let required = |stride: usize| stride * size.height;
    if src_r.len() < required(src_stride_r) {
        return Err(Error::with_reason(
            -1,
            "MergeARGBPlane",
            "source R buffer too small",
        ));
    }
    if src_g.len() < required(src_stride_g) {
        return Err(Error::with_reason(
            -1,
            "MergeARGBPlane",
            "source G buffer too small",
        ));
    }
    if src_b.len() < required(src_stride_b) {
        return Err(Error::with_reason(
            -1,
            "MergeARGBPlane",
            "source B buffer too small",
        ));
    }
    if src_a.len() < required(src_stride_a) {
        return Err(Error::with_reason(
            -1,
            "MergeARGBPlane",
            "source A buffer too small",
        ));
    }
    if dst_argb.len() < required(dst_stride_argb) {
        return Err(Error::with_reason(
            -1,
            "MergeARGBPlane",
            "destination ARGB buffer too small",
        ));
    }

    unsafe {
        sys::MergeARGBPlane(
            src_r.as_ptr(),
            src_stride_r as c_int,
            src_g.as_ptr(),
            src_stride_g as c_int,
            src_b.as_ptr(),
            src_stride_b as c_int,
            src_a.as_ptr(),
            src_stride_a as c_int,
            dst_argb.as_mut_ptr(),
            dst_stride_argb as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// ARGB を R, G, B, A プレーンに分割する
pub fn split_argb_plane(
    src_argb: &[u8],
    src_stride_argb: usize,
    dst_r: &mut [u8],
    dst_stride_r: usize,
    dst_g: &mut [u8],
    dst_stride_g: usize,
    dst_b: &mut [u8],
    dst_stride_b: usize,
    dst_a: &mut [u8],
    dst_stride_a: usize,
    size: ImageSize,
) -> Result<(), Error> {
    let required = |stride: usize| stride * size.height;
    if src_argb.len() < required(src_stride_argb) {
        return Err(Error::with_reason(
            -1,
            "SplitARGBPlane",
            "source ARGB buffer too small",
        ));
    }
    if dst_r.len() < required(dst_stride_r) {
        return Err(Error::with_reason(
            -1,
            "SplitARGBPlane",
            "destination R buffer too small",
        ));
    }
    if dst_g.len() < required(dst_stride_g) {
        return Err(Error::with_reason(
            -1,
            "SplitARGBPlane",
            "destination G buffer too small",
        ));
    }
    if dst_b.len() < required(dst_stride_b) {
        return Err(Error::with_reason(
            -1,
            "SplitARGBPlane",
            "destination B buffer too small",
        ));
    }
    if dst_a.len() < required(dst_stride_a) {
        return Err(Error::with_reason(
            -1,
            "SplitARGBPlane",
            "destination A buffer too small",
        ));
    }

    unsafe {
        sys::SplitARGBPlane(
            src_argb.as_ptr(),
            src_stride_argb as c_int,
            dst_r.as_mut_ptr(),
            dst_stride_r as c_int,
            dst_g.as_mut_ptr(),
            dst_stride_g as c_int,
            dst_b.as_mut_ptr(),
            dst_stride_b as c_int,
            dst_a.as_mut_ptr(),
            dst_stride_a as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// 16bit R, G, B, A プレーンを AR64 に結合する
pub fn merge_ar64_plane(
    src_r: &[u16],
    src_stride_r: usize,
    src_g: &[u16],
    src_stride_g: usize,
    src_b: &[u16],
    src_stride_b: usize,
    src_a: &[u16],
    src_stride_a: usize,
    dst_ar64: &mut [u16],
    dst_stride_ar64: usize,
    size: ImageSize,
    depth: i32,
) -> Result<(), Error> {
    let required = |stride: usize| stride * size.height;
    if src_r.len() < required(src_stride_r) {
        return Err(Error::with_reason(
            -1,
            "MergeAR64Plane",
            "source R buffer too small",
        ));
    }
    if src_g.len() < required(src_stride_g) {
        return Err(Error::with_reason(
            -1,
            "MergeAR64Plane",
            "source G buffer too small",
        ));
    }
    if src_b.len() < required(src_stride_b) {
        return Err(Error::with_reason(
            -1,
            "MergeAR64Plane",
            "source B buffer too small",
        ));
    }
    if src_a.len() < required(src_stride_a) {
        return Err(Error::with_reason(
            -1,
            "MergeAR64Plane",
            "source A buffer too small",
        ));
    }
    if dst_ar64.len() < required(dst_stride_ar64) {
        return Err(Error::with_reason(
            -1,
            "MergeAR64Plane",
            "destination AR64 buffer too small",
        ));
    }

    unsafe {
        sys::MergeAR64Plane(
            src_r.as_ptr(),
            src_stride_r as c_int,
            src_g.as_ptr(),
            src_stride_g as c_int,
            src_b.as_ptr(),
            src_stride_b as c_int,
            src_a.as_ptr(),
            src_stride_a as c_int,
            dst_ar64.as_mut_ptr(),
            dst_stride_ar64 as c_int,
            size.width as c_int,
            size.height as c_int,
            depth as c_int,
        )
    };

    Ok(())
}

/// 16bit R, G, B プレーンから XR30 に結合する
pub fn merge_xr30_plane(
    src_r: &[u16],
    src_stride_r: usize,
    src_g: &[u16],
    src_stride_g: usize,
    src_b: &[u16],
    src_stride_b: usize,
    dst_ar30: &mut [u8],
    dst_stride_ar30: usize,
    size: ImageSize,
    depth: i32,
) -> Result<(), Error> {
    let required = |stride: usize| stride * size.height;
    if src_r.len() < required(src_stride_r) {
        return Err(Error::with_reason(
            -1,
            "MergeXR30Plane",
            "source R buffer too small",
        ));
    }
    if src_g.len() < required(src_stride_g) {
        return Err(Error::with_reason(
            -1,
            "MergeXR30Plane",
            "source G buffer too small",
        ));
    }
    if src_b.len() < required(src_stride_b) {
        return Err(Error::with_reason(
            -1,
            "MergeXR30Plane",
            "source B buffer too small",
        ));
    }
    if dst_ar30.len() < required(dst_stride_ar30) {
        return Err(Error::with_reason(
            -1,
            "MergeXR30Plane",
            "destination AR30 buffer too small",
        ));
    }

    unsafe {
        sys::MergeXR30Plane(
            src_r.as_ptr(),
            src_stride_r as c_int,
            src_g.as_ptr(),
            src_stride_g as c_int,
            src_b.as_ptr(),
            src_stride_b as c_int,
            dst_ar30.as_mut_ptr(),
            dst_stride_ar30 as c_int,
            size.width as c_int,
            size.height as c_int,
            depth as c_int,
        )
    };

    Ok(())
}

/// 16bit R, G, B, A プレーンから 8bit ARGB に結合する
pub fn merge_argb16_to_8_plane(
    src_r: &[u16],
    src_stride_r: usize,
    src_g: &[u16],
    src_stride_g: usize,
    src_b: &[u16],
    src_stride_b: usize,
    src_a: &[u16],
    src_stride_a: usize,
    dst_argb: &mut [u8],
    dst_stride_argb: usize,
    size: ImageSize,
    depth: i32,
) -> Result<(), Error> {
    let required = |stride: usize| stride * size.height;
    if src_r.len() < required(src_stride_r) {
        return Err(Error::with_reason(
            -1,
            "MergeARGB16To8Plane",
            "source R buffer too small",
        ));
    }
    if src_g.len() < required(src_stride_g) {
        return Err(Error::with_reason(
            -1,
            "MergeARGB16To8Plane",
            "source G buffer too small",
        ));
    }
    if src_b.len() < required(src_stride_b) {
        return Err(Error::with_reason(
            -1,
            "MergeARGB16To8Plane",
            "source B buffer too small",
        ));
    }
    if src_a.len() < required(src_stride_a) {
        return Err(Error::with_reason(
            -1,
            "MergeARGB16To8Plane",
            "source A buffer too small",
        ));
    }
    if dst_argb.len() < required(dst_stride_argb) {
        return Err(Error::with_reason(
            -1,
            "MergeARGB16To8Plane",
            "destination ARGB buffer too small",
        ));
    }

    unsafe {
        sys::MergeARGB16To8Plane(
            src_r.as_ptr(),
            src_stride_r as c_int,
            src_g.as_ptr(),
            src_stride_g as c_int,
            src_b.as_ptr(),
            src_stride_b as c_int,
            src_a.as_ptr(),
            src_stride_a as c_int,
            dst_argb.as_mut_ptr(),
            dst_stride_argb as c_int,
            size.width as c_int,
            size.height as c_int,
            depth as c_int,
        )
    };

    Ok(())
}

// ============================================================
// ミラー（追加）
// ============================================================

/// インターリーブ UV プレーンの左右反転
pub fn mirror_uv_plane(
    src_uv: &[u8],
    src_stride_uv: usize,
    dst_uv: &mut [u8],
    dst_stride_uv: usize,
    size: ImageSize,
) -> Result<(), Error> {
    if src_uv.len() < src_stride_uv * size.height {
        return Err(Error::with_reason(
            -1,
            "MirrorUVPlane",
            "source UV buffer too small",
        ));
    }
    if dst_uv.len() < dst_stride_uv * size.height {
        return Err(Error::with_reason(
            -1,
            "MirrorUVPlane",
            "destination UV buffer too small",
        ));
    }

    unsafe {
        sys::MirrorUVPlane(
            src_uv.as_ptr(),
            src_stride_uv as c_int,
            dst_uv.as_mut_ptr(),
            dst_stride_uv as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

// ============================================================
// ビット深度変換
// ============================================================

/// 16bit プレーンから 8bit プレーンへの変換
pub fn convert_16_to_8_plane(
    src: &[u16],
    src_stride: usize,
    dst: &mut [u8],
    dst_stride: usize,
    scale: i32,
    size: ImageSize,
) -> Result<(), Error> {
    if src.len() < src_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "Convert16To8Plane",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "Convert16To8Plane",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::Convert16To8Plane(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            scale as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// 8bit プレーンから 16bit プレーンへの変換
pub fn convert_8_to_16_plane(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u16],
    dst_stride: usize,
    scale: i32,
    size: ImageSize,
) -> Result<(), Error> {
    if src.len() < src_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "Convert8To16Plane",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "Convert8To16Plane",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::Convert8To16Plane(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            scale as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// 8bit プレーンのスケール・バイアス変換
pub fn convert_8_to_8_plane(
    src: &[u8],
    src_stride: usize,
    dst: &mut [u8],
    dst_stride: usize,
    scale: i32,
    bias: i32,
    size: ImageSize,
) -> Result<(), Error> {
    if src.len() < src_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "Convert8To8Plane",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "Convert8To8Plane",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::Convert8To8Plane(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            scale as c_int,
            bias as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// 16bit プレーンの LSB 変換
pub fn convert_to_lsb_plane_16(
    src: &[u16],
    src_stride: usize,
    dst: &mut [u16],
    dst_stride: usize,
    size: ImageSize,
    depth: i32,
) -> Result<(), Error> {
    if src.len() < src_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "ConvertToLSBPlane_16",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "ConvertToLSBPlane_16",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::ConvertToLSBPlane_16(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
            depth as c_int,
        )
    };

    Ok(())
}

/// 16bit プレーンの MSB 変換
pub fn convert_to_msb_plane_16(
    src: &[u16],
    src_stride: usize,
    dst: &mut [u16],
    dst_stride: usize,
    size: ImageSize,
    depth: i32,
) -> Result<(), Error> {
    if src.len() < src_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "ConvertToMSBPlane_16",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "ConvertToMSBPlane_16",
            "destination buffer too small",
        ));
    }

    unsafe {
        sys::ConvertToMSBPlane_16(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
            depth as c_int,
        )
    };

    Ok(())
}

// ============================================================
// フロート変換
// ============================================================

/// 16bit プレーンをハーフフロートに変換する
pub fn half_float_plane(
    src: &[u16],
    src_stride: usize,
    dst: &mut [u16],
    dst_stride: usize,
    scale: f32,
    size: ImageSize,
) -> Result<(), Error> {
    if src.len() < src_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "HalfFloatPlane",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "HalfFloatPlane",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::HalfFloatPlane(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            scale,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "HalfFloatPlane")
}

/// U/V プレーンを半分のサイズで結合する
pub fn half_merge_uv_plane(
    src_u: &[u8],
    src_stride_u: usize,
    src_v: &[u8],
    src_stride_v: usize,
    dst_uv: &mut [u8],
    dst_stride_uv: usize,
    size: ImageSize,
) -> Result<(), Error> {
    if src_u.len() < src_stride_u * size.height {
        return Err(Error::with_reason(
            -1,
            "HalfMergeUVPlane",
            "source U buffer too small",
        ));
    }
    if src_v.len() < src_stride_v * size.height {
        return Err(Error::with_reason(
            -1,
            "HalfMergeUVPlane",
            "source V buffer too small",
        ));
    }
    if dst_uv.len() < dst_stride_uv * size.height.div_ceil(2) {
        return Err(Error::with_reason(
            -1,
            "HalfMergeUVPlane",
            "destination UV buffer too small",
        ));
    }

    unsafe {
        sys::HalfMergeUVPlane(
            src_u.as_ptr(),
            src_stride_u as c_int,
            src_v.as_ptr(),
            src_stride_v as c_int,
            dst_uv.as_mut_ptr(),
            dst_stride_uv as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Ok(())
}

/// バイト配列をフロート配列に変換する
///
/// 1 行分のみ処理する。`width` は変換する要素数。
pub fn byte_to_float(src: &[u8], dst: &mut [f32], scale: f32, width: usize) -> Result<(), Error> {
    if src.len() < width {
        return Err(Error::with_reason(
            -1,
            "ByteToFloat",
            "source buffer too small",
        ));
    }
    if dst.len() < width {
        return Err(Error::with_reason(
            -1,
            "ByteToFloat",
            "destination buffer too small",
        ));
    }

    let result = unsafe { sys::ByteToFloat(src.as_ptr(), dst.as_mut_ptr(), scale, width as c_int) };

    Error::check(result, "ByteToFloat")
}

// ============================================================
// ガウスフィルタ
// ============================================================

/// f32 プレーンにガウスフィルタを適用する
pub fn gauss_plane_f32(
    src: &[f32],
    src_stride: usize,
    dst: &mut [f32],
    dst_stride: usize,
    size: ImageSize,
) -> Result<(), Error> {
    if src.len() < src_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "GaussPlane_F32",
            "source buffer too small",
        ));
    }
    if dst.len() < dst_stride * size.height {
        return Err(Error::with_reason(
            -1,
            "GaussPlane_F32",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::GaussPlane_F32(
            src.as_ptr(),
            src_stride as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            size.width as c_int,
            size.height as c_int,
        )
    };

    Error::check(result, "GaussPlane_F32")
}
