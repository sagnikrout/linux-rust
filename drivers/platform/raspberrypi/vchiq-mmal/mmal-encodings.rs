//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/raspberrypi/vchiq-mmal/mmal-encodings.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Broadcom BCM2835 V4L2 driver
//
// Copyright © 2013 Raspberry Pi (Trading) Ltd.
//
// Authors: Vincent Sanders @ Collabora
// Dave Stevenson @ Broadcom
// (now dave.stevenson@raspberrypi.org)
// Simon Mellor @ Broadcom
// Luke Diamand @ Broadcom
//

// SAND Video (YUVUV128) format, native format understood by VideoCore.
// This format is *not* opaque - if requested you will receive full frames
// of YUV_UV video.
//

// VideoCore opaque image format, image handles are returned to
// the host but not the actual image data.
//

// An EGL image handle
//

// }@
// \name Pre-defined audio encodings
// @{

// Pre-defined H264 encoding variants
// ISO 14496-10 Annex B byte stream format
pub const MMAL_ENCODING_VARIANT_H264_DEFAULT: c_int = 0;
// ISO 14496-15 AVC stream format

// Implicitly delineated NAL units without emulation prevention

// \defgroup MmalColorSpace List of pre-defined video color spaces
// This defines a list of common color spaces. This list isn't exhaustive and
// is only provided as a convenience to avoid clients having to use FourCC
// codes directly. However components are allowed to define and use their own
// FourCC codes.
//
// @{
// Unknown color space
pub const MMAL_COLOR_SPACE_UNKNOWN: c_int = 0;
// ITU-R BT.601-5 [SDTV]

// ITU-R BT.709-3 [HDTV]

// JPEG JFIF

// Title 47 Code of Federal Regulations (2003) 73.682 (a) (20)

// Society of Motion Picture and Television Engineers 240M (1999)

// ITU-R BT.470-2 System M

// ITU-R BT.470-2 System BG

// JPEG JFIF, but with 16..255 luma

// @} MmalColorSpace List
