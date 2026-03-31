//! スケーリング関数
#![allow(clippy::too_many_arguments)]

use std::ffi::c_int;

use crate::{
    ArgbImage, ArgbImageMut, Error, FilterMode, I012Image, I012ImageMut, I212Image, I212ImageMut,
    I412Image, I412ImageMut, I420Image, I420ImageMut, I422Image, I422ImageMut, I444Image,
    I444ImageMut, ImageSize, Nv12Image, Nv12ImageMut, Nv24Image, Nv24ImageMut, checked_buf_size,
    require_c_int, sys,
};

// ---------------------------------------------------------------------------
// I420 / I422 / I444 8bit スケーリング
// ---------------------------------------------------------------------------

/// I420 形式の YUV データをリサイズする
pub fn i420_scale(
    src: &I420Image<'_>,
    src_size: ImageSize,
    dst: &mut I420ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "I420Scale")?;
    dst.validate(dst_size, "I420Scale")?;

    let result = unsafe {
        sys::I420Scale(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "I420Scale")
}

/// I422 形式の YUV データをリサイズする
pub fn i422_scale(
    src: &I422Image<'_>,
    src_size: ImageSize,
    dst: &mut I422ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "I422Scale")?;
    dst.validate(dst_size, "I422Scale")?;

    let result = unsafe {
        sys::I422Scale(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "I422Scale")
}

/// I444 形式の YUV データをリサイズする
pub fn i444_scale(
    src: &I444Image<'_>,
    src_size: ImageSize,
    dst: &mut I444ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "I444Scale")?;
    dst.validate(dst_size, "I444Scale")?;

    let result = unsafe {
        sys::I444Scale(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "I444Scale")
}

// ---------------------------------------------------------------------------
// 高ビット深度スケーリング（12bit / 16bit）
// ---------------------------------------------------------------------------

/// I420 形式の 12bit YUV データをリサイズする
pub fn i420_scale_12(
    src: &I012Image<'_>,
    src_size: ImageSize,
    dst: &mut I012ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "I420Scale_12")?;
    dst.validate(dst_size, "I420Scale_12")?;

    let result = unsafe {
        sys::I420Scale_12(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "I420Scale_12")
}

/// I420 形式の 16bit YUV データをリサイズする
pub fn i420_scale_16(
    src: &I012Image<'_>,
    src_size: ImageSize,
    dst: &mut I012ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "I420Scale_16")?;
    dst.validate(dst_size, "I420Scale_16")?;

    let result = unsafe {
        sys::I420Scale_16(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "I420Scale_16")
}

/// I422 形式の 12bit YUV データをリサイズする
pub fn i422_scale_12(
    src: &I212Image<'_>,
    src_size: ImageSize,
    dst: &mut I212ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "I422Scale_12")?;
    dst.validate(dst_size, "I422Scale_12")?;

    let result = unsafe {
        sys::I422Scale_12(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "I422Scale_12")
}

/// I422 形式の 16bit YUV データをリサイズする
pub fn i422_scale_16(
    src: &I212Image<'_>,
    src_size: ImageSize,
    dst: &mut I212ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "I422Scale_16")?;
    dst.validate(dst_size, "I422Scale_16")?;

    let result = unsafe {
        sys::I422Scale_16(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "I422Scale_16")
}

/// I444 形式の 12bit YUV データをリサイズする
pub fn i444_scale_12(
    src: &I412Image<'_>,
    src_size: ImageSize,
    dst: &mut I412ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "I444Scale_12")?;
    dst.validate(dst_size, "I444Scale_12")?;

    let result = unsafe {
        sys::I444Scale_12(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "I444Scale_12")
}

/// I444 形式の 16bit YUV データをリサイズする
pub fn i444_scale_16(
    src: &I412Image<'_>,
    src_size: ImageSize,
    dst: &mut I412ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "I444Scale_16")?;
    dst.validate(dst_size, "I444Scale_16")?;

    let result = unsafe {
        sys::I444Scale_16(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.u.as_ptr(),
            src.u_stride as c_int,
            src.v.as_ptr(),
            src.v_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.u.as_mut_ptr(),
            dst.u_stride as c_int,
            dst.v.as_mut_ptr(),
            dst.v_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "I444Scale_16")
}

// ---------------------------------------------------------------------------
// プレーンスケーリング（8bit / 12bit / 16bit）
// ---------------------------------------------------------------------------

/// 単一プレーンのスケーリング
pub fn scale_plane(
    src: &[u8],
    src_stride: usize,
    src_size: ImageSize,
    dst: &mut [u8],
    dst_stride: usize,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "ScalePlane",
        "source width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "ScalePlane",
        "source height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "ScalePlane",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_size.width,
        "ScalePlane",
        "destination width exceeds c_int range",
    )?;
    require_c_int(
        dst_size.height,
        "ScalePlane",
        "destination height exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "ScalePlane",
        "destination stride exceeds c_int range",
    )?;
    // stride >= width チェック
    if src_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "ScalePlane",
            "source stride smaller than width",
        ));
    }
    if dst_stride < dst_size.width {
        return Err(Error::with_reason(
            -1,
            "ScalePlane",
            "destination stride smaller than width",
        ));
    }
    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "ScalePlane",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "ScalePlane",
            "source buffer too small",
        ));
    }
    let dst_buf = checked_buf_size(
        dst_stride,
        dst_size.height,
        "ScalePlane",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "ScalePlane",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::ScalePlane(
            src.as_ptr(),
            src_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "ScalePlane")
}

/// 単一プレーンの 12bit スケーリング
pub fn scale_plane_12(
    src: &[u16],
    src_stride: usize,
    src_size: ImageSize,
    dst: &mut [u16],
    dst_stride: usize,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "ScalePlane_12",
        "source width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "ScalePlane_12",
        "source height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "ScalePlane_12",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_size.width,
        "ScalePlane_12",
        "destination width exceeds c_int range",
    )?;
    require_c_int(
        dst_size.height,
        "ScalePlane_12",
        "destination height exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "ScalePlane_12",
        "destination stride exceeds c_int range",
    )?;
    // stride >= width チェック（stride は要素数、width はピクセル数）
    if src_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "ScalePlane_12",
            "source stride smaller than width",
        ));
    }
    if dst_stride < dst_size.width {
        return Err(Error::with_reason(
            -1,
            "ScalePlane_12",
            "destination stride smaller than width",
        ));
    }
    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "ScalePlane_12",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "ScalePlane_12",
            "source buffer too small",
        ));
    }
    let dst_buf = checked_buf_size(
        dst_stride,
        dst_size.height,
        "ScalePlane_12",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "ScalePlane_12",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::ScalePlane_12(
            src.as_ptr(),
            src_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "ScalePlane_12")
}

/// 単一プレーンの 16bit スケーリング
pub fn scale_plane_16(
    src: &[u16],
    src_stride: usize,
    src_size: ImageSize,
    dst: &mut [u16],
    dst_stride: usize,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "ScalePlane_16",
        "source width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "ScalePlane_16",
        "source height exceeds c_int range",
    )?;
    require_c_int(
        src_stride,
        "ScalePlane_16",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_size.width,
        "ScalePlane_16",
        "destination width exceeds c_int range",
    )?;
    require_c_int(
        dst_size.height,
        "ScalePlane_16",
        "destination height exceeds c_int range",
    )?;
    require_c_int(
        dst_stride,
        "ScalePlane_16",
        "destination stride exceeds c_int range",
    )?;
    // stride >= width チェック（stride は要素数、width はピクセル数）
    if src_stride < src_size.width {
        return Err(Error::with_reason(
            -1,
            "ScalePlane_16",
            "source stride smaller than width",
        ));
    }
    if dst_stride < dst_size.width {
        return Err(Error::with_reason(
            -1,
            "ScalePlane_16",
            "destination stride smaller than width",
        ));
    }
    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride,
        src_size.height,
        "ScalePlane_16",
        "source buffer size overflow",
    )?;
    if src.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "ScalePlane_16",
            "source buffer too small",
        ));
    }
    let dst_buf = checked_buf_size(
        dst_stride,
        dst_size.height,
        "ScalePlane_16",
        "destination buffer size overflow",
    )?;
    if dst.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "ScalePlane_16",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::ScalePlane_16(
            src.as_ptr(),
            src_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.as_mut_ptr(),
            dst_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "ScalePlane_16")
}

// ---------------------------------------------------------------------------
// NV12 / NV24 スケーリング
// ---------------------------------------------------------------------------

/// NV12 形式のデータをリサイズする
pub fn nv12_scale(
    src: &Nv12Image<'_>,
    src_size: ImageSize,
    dst: &mut Nv12ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "NV12Scale")?;
    dst.validate(dst_size, "NV12Scale")?;

    let result = unsafe {
        sys::NV12Scale(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.uv.as_ptr(),
            src.uv_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "NV12Scale")
}

/// NV24 形式のデータをリサイズする
pub fn nv24_scale(
    src: &Nv24Image<'_>,
    src_size: ImageSize,
    dst: &mut Nv24ImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "NV24Scale")?;
    dst.validate(dst_size, "NV24Scale")?;

    let result = unsafe {
        sys::NV24Scale(
            src.y.as_ptr(),
            src.y_stride as c_int,
            src.uv.as_ptr(),
            src.uv_stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.y.as_mut_ptr(),
            dst.y_stride as c_int,
            dst.uv.as_mut_ptr(),
            dst.uv_stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "NV24Scale")
}

// ---------------------------------------------------------------------------
// UV スケーリング
// ---------------------------------------------------------------------------

/// インターリーブ UV プレーンのスケーリング
pub fn uv_scale(
    src_uv: &[u8],
    src_stride_uv: usize,
    src_size: ImageSize,
    dst_uv: &mut [u8],
    dst_stride_uv: usize,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "UVScale",
        "source width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "UVScale",
        "source height exceeds c_int range",
    )?;
    require_c_int(
        src_stride_uv,
        "UVScale",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_size.width,
        "UVScale",
        "destination width exceeds c_int range",
    )?;
    require_c_int(
        dst_size.height,
        "UVScale",
        "destination height exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_uv,
        "UVScale",
        "destination stride exceeds c_int range",
    )?;
    // stride >= width * 2 チェック（UV インターリーブなので 1 ピクセルあたり 2 バイト）
    let src_min_stride = src_size
        .width
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, "UVScale", "source width * 2 overflow"))?;
    if src_stride_uv < src_min_stride {
        return Err(Error::with_reason(
            -1,
            "UVScale",
            "source stride smaller than width * 2",
        ));
    }
    let dst_min_stride = dst_size
        .width
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, "UVScale", "destination width * 2 overflow"))?;
    if dst_stride_uv < dst_min_stride {
        return Err(Error::with_reason(
            -1,
            "UVScale",
            "destination stride smaller than width * 2",
        ));
    }
    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride_uv,
        src_size.height,
        "UVScale",
        "source buffer size overflow",
    )?;
    if src_uv.len() < src_buf {
        return Err(Error::with_reason(-1, "UVScale", "source buffer too small"));
    }
    let dst_buf = checked_buf_size(
        dst_stride_uv,
        dst_size.height,
        "UVScale",
        "destination buffer size overflow",
    )?;
    if dst_uv.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "UVScale",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::UVScale(
            src_uv.as_ptr(),
            src_stride_uv as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst_uv.as_mut_ptr(),
            dst_stride_uv as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "UVScale")
}

/// インターリーブ UV プレーンの 16bit スケーリング
pub fn uv_scale_16(
    src_uv: &[u16],
    src_stride_uv: usize,
    src_size: ImageSize,
    dst_uv: &mut [u16],
    dst_stride_uv: usize,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    // c_int 範囲チェック
    require_c_int(
        src_size.width,
        "UVScale_16",
        "source width exceeds c_int range",
    )?;
    require_c_int(
        src_size.height,
        "UVScale_16",
        "source height exceeds c_int range",
    )?;
    require_c_int(
        src_stride_uv,
        "UVScale_16",
        "source stride exceeds c_int range",
    )?;
    require_c_int(
        dst_size.width,
        "UVScale_16",
        "destination width exceeds c_int range",
    )?;
    require_c_int(
        dst_size.height,
        "UVScale_16",
        "destination height exceeds c_int range",
    )?;
    require_c_int(
        dst_stride_uv,
        "UVScale_16",
        "destination stride exceeds c_int range",
    )?;
    // stride >= width * 2 チェック（UV インターリーブなので 1 ピクセルあたり 2 要素）
    let src_min_stride = src_size
        .width
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, "UVScale_16", "source width * 2 overflow"))?;
    if src_stride_uv < src_min_stride {
        return Err(Error::with_reason(
            -1,
            "UVScale_16",
            "source stride smaller than width * 2",
        ));
    }
    let dst_min_stride = dst_size
        .width
        .checked_mul(2)
        .ok_or_else(|| Error::with_reason(-1, "UVScale_16", "destination width * 2 overflow"))?;
    if dst_stride_uv < dst_min_stride {
        return Err(Error::with_reason(
            -1,
            "UVScale_16",
            "destination stride smaller than width * 2",
        ));
    }
    // バッファサイズチェック（オーバーフロー安全）
    let src_buf = checked_buf_size(
        src_stride_uv,
        src_size.height,
        "UVScale_16",
        "source buffer size overflow",
    )?;
    if src_uv.len() < src_buf {
        return Err(Error::with_reason(
            -1,
            "UVScale_16",
            "source buffer too small",
        ));
    }
    let dst_buf = checked_buf_size(
        dst_stride_uv,
        dst_size.height,
        "UVScale_16",
        "destination buffer size overflow",
    )?;
    if dst_uv.len() < dst_buf {
        return Err(Error::with_reason(
            -1,
            "UVScale_16",
            "destination buffer too small",
        ));
    }

    let result = unsafe {
        sys::UVScale_16(
            src_uv.as_ptr(),
            src_stride_uv as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst_uv.as_mut_ptr(),
            dst_stride_uv as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "UVScale_16")
}

// ---------------------------------------------------------------------------
// ARGB スケーリング
// ---------------------------------------------------------------------------

/// ARGB 画像のスケーリング
pub fn argb_scale(
    src: &ArgbImage<'_>,
    src_size: ImageSize,
    dst: &mut ArgbImageMut<'_>,
    dst_size: ImageSize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "ARGBScale")?;
    dst.validate(dst_size, "ARGBScale")?;

    let result = unsafe {
        sys::ARGBScale(
            src.data.as_ptr(),
            src.stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "ARGBScale")
}

/// ARGB 画像のクリップ領域付きスケーリング
pub fn argb_scale_clip(
    src: &ArgbImage<'_>,
    src_size: ImageSize,
    dst: &mut ArgbImageMut<'_>,
    dst_size: ImageSize,
    clip_x: usize,
    clip_y: usize,
    clip_width: usize,
    clip_height: usize,
    filtering: FilterMode,
) -> Result<(), Error> {
    src.validate(src_size, "ARGBScaleClip")?;
    dst.validate(dst_size, "ARGBScaleClip")?;
    // クリップパラメータの c_int 範囲チェック
    require_c_int(clip_x, "ARGBScaleClip", "clip_x exceeds c_int range")?;
    require_c_int(clip_y, "ARGBScaleClip", "clip_y exceeds c_int range")?;
    require_c_int(
        clip_width,
        "ARGBScaleClip",
        "clip_width exceeds c_int range",
    )?;
    require_c_int(
        clip_height,
        "ARGBScaleClip",
        "clip_height exceeds c_int range",
    )?;

    let result = unsafe {
        sys::ARGBScaleClip(
            src.data.as_ptr(),
            src.stride as c_int,
            src_size.width as c_int,
            src_size.height as c_int,
            dst.data.as_mut_ptr(),
            dst.stride as c_int,
            dst_size.width as c_int,
            dst_size.height as c_int,
            clip_x as c_int,
            clip_y as c_int,
            clip_width as c_int,
            clip_height as c_int,
            filtering.to_sys(),
        )
    };

    Error::check(result, "ARGBScaleClip")
}
