//! Automatically rewritten from C to Rust
//! Source: drivers/media/common/uvc.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-or-later

// ------------------------------------------------------------------------
// Video formats
//
    static const struct uvc_format_desc uvc_fmts[] = {
    {
    .guid		= UVC_GUID_FORMAT_YUY2,
    .fcc		= V4L2_PIX_FMT_YUYV,
    },
    {
    .guid		= UVC_GUID_FORMAT_YUY2_ISIGHT,
    .fcc		= V4L2_PIX_FMT_YUYV,
    },
    {
    .guid		= UVC_GUID_FORMAT_NV12,
    .fcc		= V4L2_PIX_FMT_NV12,
    },
    {
    .guid		= UVC_GUID_FORMAT_MJPEG,
    .fcc		= V4L2_PIX_FMT_MJPEG,
    },
    {
    .guid		= UVC_GUID_FORMAT_YV12,
    .fcc		= V4L2_PIX_FMT_YVU420,
    },
    {
    .guid		= UVC_GUID_FORMAT_I420,
    .fcc		= V4L2_PIX_FMT_YUV420,
    },
    {
    .guid		= UVC_GUID_FORMAT_M420,
    .fcc		= V4L2_PIX_FMT_M420,
    },
    {
    .guid		= UVC_GUID_FORMAT_P010,
    .fcc		= V4L2_PIX_FMT_P010,
    },
    {
    .guid		= UVC_GUID_FORMAT_UYVY,
    .fcc		= V4L2_PIX_FMT_UYVY,
    },
    {
    .guid		= UVC_GUID_FORMAT_Y800,
    .fcc		= V4L2_PIX_FMT_GREY,
    },
    {
    .guid		= UVC_GUID_FORMAT_Y8,
    .fcc		= V4L2_PIX_FMT_GREY,
    },
    {
    .guid		= UVC_GUID_FORMAT_D3DFMT_L8,
    .fcc		= V4L2_PIX_FMT_GREY,
    },
    {
    .guid		= UVC_GUID_FORMAT_KSMEDIA_L8_IR,
    .fcc		= V4L2_PIX_FMT_GREY,
    },
    {
    .guid		= UVC_GUID_FORMAT_Y10,
    .fcc		= V4L2_PIX_FMT_Y10,
    },
    {
    .guid		= UVC_GUID_FORMAT_Y12,
    .fcc		= V4L2_PIX_FMT_Y12,
    },
    {
    .guid		= UVC_GUID_FORMAT_Y16,
    .fcc		= V4L2_PIX_FMT_Y16,
    },
    {
    .guid		= UVC_GUID_FORMAT_BY8,
    .fcc		= V4L2_PIX_FMT_SBGGR8,
    },
    {
    .guid		= UVC_GUID_FORMAT_BA81,
    .fcc		= V4L2_PIX_FMT_SBGGR8,
    },
    {
    .guid		= UVC_GUID_FORMAT_GBRG,
    .fcc		= V4L2_PIX_FMT_SGBRG8,
    },
    {
    .guid		= UVC_GUID_FORMAT_GRBG,
    .fcc		= V4L2_PIX_FMT_SGRBG8,
    },
    {
    .guid		= UVC_GUID_FORMAT_RGGB,
    .fcc		= V4L2_PIX_FMT_SRGGB8,
    },
    {
    .guid		= UVC_GUID_FORMAT_RGBP,
    .fcc		= V4L2_PIX_FMT_RGB565,
    },
    {
    .guid		= UVC_GUID_FORMAT_D3DFMT_R5G6B5,
    .fcc		= V4L2_PIX_FMT_RGB565,
    },
    {
    .guid		= UVC_GUID_FORMAT_BGR3,
    .fcc		= V4L2_PIX_FMT_BGR24,
    },
    {
    .guid		= UVC_GUID_FORMAT_BGR4,
    .fcc		= V4L2_PIX_FMT_XBGR32,
    },
    {
    .guid		= UVC_GUID_FORMAT_H264,
    .fcc		= V4L2_PIX_FMT_H264,
    },
    {
    .guid		= UVC_GUID_FORMAT_H265,
    .fcc		= V4L2_PIX_FMT_HEVC,
    },
    {
    .guid		= UVC_GUID_FORMAT_Y8I,
    .fcc		= V4L2_PIX_FMT_Y8I,
    },
    {
    .guid		= UVC_GUID_FORMAT_Y12I,
    .fcc		= V4L2_PIX_FMT_Y12I,
    },
    {
    .guid		= UVC_GUID_FORMAT_Y16I,
    .fcc		= V4L2_PIX_FMT_Y16I,
    },
    {
    .guid		= UVC_GUID_FORMAT_Z16,
    .fcc		= V4L2_PIX_FMT_Z16,
    },
    {
    .guid		= UVC_GUID_FORMAT_RW10,
    .fcc		= V4L2_PIX_FMT_SRGGB10P,
    },
    {
    .guid		= UVC_GUID_FORMAT_BG16,
    .fcc		= V4L2_PIX_FMT_SBGGR16,
    },
    {
    .guid		= UVC_GUID_FORMAT_GB16,
    .fcc		= V4L2_PIX_FMT_SGBRG16,
    },
    {
    .guid		= UVC_GUID_FORMAT_RG16,
    .fcc		= V4L2_PIX_FMT_SRGGB16,
    },
    {
    .guid		= UVC_GUID_FORMAT_GR16,
    .fcc		= V4L2_PIX_FMT_SGRBG16,
    },
    {
    .guid		= UVC_GUID_FORMAT_INVZ,
    .fcc		= V4L2_PIX_FMT_Z16,
    },
    {
    .guid		= UVC_GUID_FORMAT_INVI,
    .fcc		= V4L2_PIX_FMT_Y10,
    },
    {
    .guid		= UVC_GUID_FORMAT_INZI,
    .fcc		= V4L2_PIX_FMT_INZI,
    },
    {
    .guid		= UVC_GUID_FORMAT_CNF4,
    .fcc		= V4L2_PIX_FMT_CNF4,
    },
    {
    .guid		= UVC_GUID_FORMAT_HEVC,
    .fcc		= V4L2_PIX_FMT_HEVC,
    },
    };
    const struct uvc_format_desc *uvc_format_by_guid(const u8 guid[16])
    {
    let mut len: c_uint = ARRAY_SIZE(uvc_fmts);
    unsigned int i;
    for (i = 0; i < len; ++i) {
    if (memcmp(guid, uvc_fmts[i].guid, 16) == 0)
    return &uvc_fmts[i];
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(uvc_format_by_guid);
    MODULE_DESCRIPTION("USB Video Class common code");
    MODULE_LICENSE("GPL");
