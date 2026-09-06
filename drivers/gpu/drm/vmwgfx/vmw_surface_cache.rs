//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmw_surface_cache.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (c) 2021-2024 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// vmw_surface_get_desc - Look up the appropriate SVGA3dSurfaceDesc for the
// given format.
//
// vmw_surface_get_mip_size -  Given a base level size and the mip level,
// compute the size of the mip level.
//
// vmw_surface_get_image_buffer_size - Calculates image buffer size.
//
// Return the number of bytes of buffer space required to store one image of a
// surface, optionally using the specified pitch.
//
// If pitch is zero, it is assumed that rows are tightly packed.
//
// This function is overflow-safe. If the result would have overflowed, instead
// we return MAX_UINT32.
//
// vmw_surface_get_serialized_size - Get the serialized size for the image.
//
// vmw_surface_get_serialized_size_extended - Returns the number of bytes
// required for a surface with given parameters. Support for sample count.
//
extern "C" {
    pub fn min_t(_arg: u64, _arg: total_size, _arg: (uint64_t)U32_MAX) -> return;
}
//
// vmw_surface_get_pixel_offset - Compute the offset (in bytes) to a pixel
// in an image (or volume).
//
// @width: The image width in pixels.
// @height: The image height in pixels
//
// vmw_surface_is_gb_screen_target_format - Is the specified format usable as
// a ScreenTarget?
// (with just the GBObjects cap-bit
// set)
// @format: format to queried
//
// RETURNS:
// true if queried format is valid for screen targets
//
// vmw_surface_is_dx_screen_target_format - Is the specified format usable as
// a ScreenTarget?
// (with DX10 enabled)
//
// @format: format to queried
//
// Results:
// true if queried format is valid for screen targets
//
// vmw_surface_is_screen_target_format - Is the specified format usable as a
// ScreenTarget?
// (for some combination of caps)
//
// @format: format to queried
//
// Results:
// true if queried format is valid for screen targets
//
extern "C" {
    pub fn vmw_surface_is_dx_screen_target_format(_arg: format) -> return;
}
//
// struct vmw_surface_mip - Mimpmap level information
// @bytes: Bytes required in the backing store of this mipmap level.
// @img_stride: Byte stride per image.
// @row_stride: Byte stride per block row.
// @size: The size of the mipmap.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_surface_mip {
    pub bytes: usize,
    pub img_stride: usize,
    pub row_stride: usize,
    pub size: drm_vmw_size,
}

//
// struct vmw_surface_cache - Cached surface information
// @desc: Pointer to the surface descriptor
// @mip: Array of mipmap level information. Valid size is @num_mip_levels.
// @mip_chain_bytes: Bytes required in the backing store for the whole chain
// of mip levels.
// @sheet_bytes: Bytes required in the backing store for a sheet
// representing a single sample.
// @num_mip_levels: Valid size of the @mip array. Number of mipmap levels in
// a chain.
// @num_layers: Number of slices in an array texture or number of faces in
// a cubemap texture.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_surface_cache {
    pub desc: *const SVGA3dSurfaceDesc,
    pub mip: [vmw_surface_mip; DRM_VMW_MAX_MIP_LEVELS],
    pub mip_chain_bytes: usize,
    pub sheet_bytes: usize,
    pub num_mip_levels: u32,
    pub num_layers: u32,
}

//
// struct vmw_surface_loc - Surface location
// @sheet: The multisample sheet.
// @sub_resource: Surface subresource. Defined as layer * num_mip_levels +
// mip_level.
// @x: X coordinate.
// @y: Y coordinate.
// @z: Z coordinate.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_surface_loc {
    pub sheet: u32,
    pub sub_resource: u32,
    pub z: u32 x, y,,
}

//
// vmw_surface_subres - Compute the subresource from layer and mipmap.
// @cache: Surface layout data.
// @mip_level: The mipmap level.
// @layer: The surface layer (face or array slice).
//
// Return: The subresource.
//
// vmw_surface_setup_cache - Build a surface cache entry
// @size: The surface base level dimensions.
// @format: The surface format.
// @num_mip_levels: Number of mipmap levels.
// @num_layers: Number of layers.
// @cache: Pointer to a struct vmw_surface_cach object to be filled in.
//
// Return: Zero on success, -EINVAL on invalid surface layout.
//
// vmw_surface_get_loc - Get a surface location from an offset into the
// backing store
// @cache: Surface layout data.
// @loc: Pointer to a struct vmw_surface_loc to be filled in.
// @offset: Offset into the surface backing store.
//
// vmw_surface_inc_loc - Clamp increment a surface location with one block
// size
// in each dimension.
// @loc: Pointer to a struct vmw_surface_loc to be incremented.
//
// When computing the size of a range as size = end - start, the range does not
// include the end element. However a location representing the last byte
// of a touched region in the backing store *is* included in the range.
// This function modifies such a location to match the end definition
// given as start + size which is the one used in a SVGA3dBox.
//
// vmw_surface_min_loc - The start location in a subresource
// @cache: Surface layout data.
// @sub_resource: The subresource.
// @loc: Pointer to a struct vmw_surface_loc to be filled in.
//
// vmw_surface_min_loc - The end location in a subresource
// @cache: Surface layout data.
// @sub_resource: The subresource.
// @loc: Pointer to a struct vmw_surface_loc to be filled in.
//
// Following the end definition given in vmw_surface_inc_loc(),
// Compute the end location of a surface subresource.
//
