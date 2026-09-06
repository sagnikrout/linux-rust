//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_fourcc.h
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


//
// Copyright (c) 2016 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//
// Permission to use, copy, modify, distribute, and sell this software and its
// documentation for any purpose is hereby granted without fee, provided that
// the above copyright notice appear in all copies and that both that copyright
// notice and this permission notice appear in supporting documentation, and
// that the name of the copyright holders not be used in advertising or
// publicity pertaining to distribution of the software without specific,
// written prior permission.  The copyright holders make no representations
// about the suitability of this software for any purpose.  It is provided "as
// is" without express or implied warranty.
//
// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.
//

//
// DRM_FORMAT_MAX_PLANES - maximum number of planes a DRM format can have
//

//
// DRM formats are little endian.  Define host endian variants for the
// most common formats here, to reduce the #ifdefs needed in drivers.
//
// Note that the DRM_FORMAT_BIG_ENDIAN flag should only be used in
// case the format can't be specified otherwise, so we don't end up
// with two values describing the same format.
//

//
// struct drm_format_info - information about a DRM format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_format_info {
// @format: 4CC format identifier (DRM_FORMAT_*)
    pub format: u32,
//
// @depth:
//
// Color depth (number of bits per pixel excluding padding bits),
// valid for a subset of RGB formats only. This is a legacy field, do
// not use in new code and set to 0 for new formats.
//
    pub depth: u8,
// @num_planes: Number of color planes (1 to 3)
    pub num_planes: u8,
//
// @cpp:
//
// Number of bytes per pixel (per plane), this is aliased with
// @char_per_block. It is deprecated in favour of using the
// triplet @char_per_block, @block_w, @block_h for better
// describing the pixel format.
//
    pub cpp: [u8; DRM_FORMAT_MAX_PLANES],
//
// @char_per_block:
//
// Number of bytes per block (per plane), where blocks are
// defined as a rectangle of pixels which are stored next to
// each other in a byte aligned memory region. Together with
// @block_w and @block_h this is used to properly describe tiles
// in tiled formats or to describe groups of pixels in packed
// formats for which the memory needed for a single pixel is not
// byte aligned.
//
// @cpp has been kept for historical reasons because there are
// a lot of places in drivers where it's used. In drm core for
// generic code paths the preferred way is to use
// @char_per_block, drm_format_info_block_width() and
// drm_format_info_block_height() which allows handling both
// block and non-block formats in the same way.
//
// For formats that are intended to be used only with non-linear
// modifiers both @cpp and @char_per_block must be 0 in the
// generic format table. Drivers could supply accurate
// information from their drm_mode_config.get_format_info hook
// if they want the core to be validating the pitch.
//
    pub char_per_block: [u8; DRM_FORMAT_MAX_PLANES],
}

//
// @block_w:
//
// Block width in pixels, this is intended to be accessed through
// drm_format_info_block_width()
//
// @block_h:
//
// Block height in pixels, this is intended to be accessed through
// drm_format_info_block_height()
//
// @hsub: Horizontal chroma subsampling factor
// @vsub: Vertical chroma subsampling factor
// @has_alpha: Does the format embeds an alpha component?
// @is_yuv: Is it a YUV format?
// @is_color_indexed: Is it a color-indexed format?
//
// drm_format_info_is_yuv_packed - check that the format info matches a YUV
// format with data laid in a single plane
// @info: format info
//
// Returns:
// A boolean indicating whether the format info matches a packed YUV format.
//
// drm_format_info_is_yuv_semiplanar - check that the format info matches a YUV
// format with data laid in two planes (luminance and chrominance)
// @info: format info
//
// Returns:
// A boolean indicating whether the format info matches a semiplanar YUV format.
//
// drm_format_info_is_yuv_planar - check that the format info matches a YUV
// format with data laid in three planes (one for each YUV component)
// @info: format info
//
// Returns:
// A boolean indicating whether the format info matches a planar YUV format.
//
// drm_format_info_is_yuv_sampling_410 - check that the format info matches a
// YUV format with 4:1:0 sub-sampling
// @info: format info
//
// Returns:
// A boolean indicating whether the format info matches a YUV format with 4:1:0
// sub-sampling.
//
// drm_format_info_is_yuv_sampling_411 - check that the format info matches a
// YUV format with 4:1:1 sub-sampling
// @info: format info
//
// Returns:
// A boolean indicating whether the format info matches a YUV format with 4:1:1
// sub-sampling.
//
// drm_format_info_is_yuv_sampling_420 - check that the format info matches a
// YUV format with 4:2:0 sub-sampling
// @info: format info
//
// Returns:
// A boolean indicating whether the format info matches a YUV format with 4:2:0
// sub-sampling.
//
// drm_format_info_is_yuv_sampling_422 - check that the format info matches a
// YUV format with 4:2:2 sub-sampling
// @info: format info
//
// Returns:
// A boolean indicating whether the format info matches a YUV format with 4:2:2
// sub-sampling.
//
// drm_format_info_is_yuv_sampling_444 - check that the format info matches a
// YUV format with 4:4:4 sub-sampling
// @info: format info
//
// Returns:
// A boolean indicating whether the format info matches a YUV format with 4:4:4
// sub-sampling.
//
// drm_format_info_plane_width - width of the plane given the first plane
// @info: pixel format info
// @width: width of the first plane
// @plane: plane index
//
// Returns:
// The width of @plane, given that the width of the first plane is @width.
//
extern "C" {
    pub fn DIV_ROUND_UP(_arg: width, _arg: info->hsub) -> return;
}
//
// drm_format_info_plane_height - height of the plane given the first plane
// @info: pixel format info
// @height: height of the first plane
// @plane: plane index
//
// Returns:
// The height of @plane, given that the height of the first plane is @height.
//
extern "C" {
    pub fn DIV_ROUND_UP(_arg: height, _arg: info->vsub) -> return;
}
extern "C" {
    pub fn drm_mode_legacy_fb_format(bpp: u32, depth: u32) -> u32;
}
extern "C" {
    pub fn drm_driver_color_mode_format(dev: *mut drm_device, color_mode: c_uint) -> u32;
}
extern "C" {
    pub fn drm_format_info_bpp(info: *const drm_format_info, plane: c_int) -> c_uint;
}
