//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/fb_decoder.h
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
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Kevin Tian <kevin.tian@intel.com>
//
// Contributors:
// Bing Niu <bing.niu@intel.com>
// Xu Han <xu.han@intel.com>
// Ping Gao <ping.a.gao@intel.com>
// Xiaoguang Chen <xiaoguang.chen@intel.com>
// Yang Liu <yang2.liu@intel.com>
// Tina Zhang <tina.zhang@intel.com>
//

pub const _PLANE_CTL_FORMAT_SHIFT: c_int = 24;
pub const _PLANE_CTL_TILED_SHIFT: c_int = 10;
pub const _PIPE_V_SRCSZ_SHIFT: c_int = 0;

pub const _PIPE_H_SRCSZ_SHIFT: c_int = 16;

pub const _PRI_PLANE_FMT_SHIFT: c_int = 26;

pub const _PRI_PLANE_X_OFF_SHIFT: c_int = 0;

pub const _PRI_PLANE_Y_OFF_SHIFT: c_int = 16;

pub const _CURSOR_MODE: c_uint = 0x3f;
pub const _CURSOR_ALPHA_FORCE_SHIFT: c_int = 8;

pub const _CURSOR_ALPHA_PLANE_SHIFT: c_int = 10;

pub const _CURSOR_POS_X_SHIFT: c_int = 0;

pub const _CURSOR_SIGN_X_SHIFT: c_int = 15;

pub const _CURSOR_POS_Y_SHIFT: c_int = 16;

pub const _CURSOR_SIGN_Y_SHIFT: c_int = 31;

pub const _SPRITE_FMT_SHIFT: c_int = 25;
pub const _SPRITE_COLOR_ORDER_SHIFT: c_int = 20;
pub const _SPRITE_YUV_ORDER_SHIFT: c_int = 16;
pub const _SPRITE_STRIDE_SHIFT: c_int = 6;

pub const _SPRITE_SIZE_WIDTH_SHIFT: c_int = 0;
pub const _SPRITE_SIZE_HEIGHT_SHIFT: c_int = 16;

pub const _SPRITE_POS_X_SHIFT: c_int = 0;
pub const _SPRITE_POS_Y_SHIFT: c_int = 16;

pub const _SPRITE_OFFSET_START_X_SHIFT: c_int = 0;
pub const _SPRITE_OFFSET_START_Y_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GVT_FB_EVENT {
    FB_MODE_SET_START = 1,
    FB_MODE_SET_END,
    FB_DISPLAY_FLIP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DDI_PORT {
    DDI_PORT_NONE	= 0,
    DDI_PORT_B	= 1,
    DDI_PORT_C	= 2,
    DDI_PORT_D	= 3,
    DDI_PORT_E	= 4
}

// color space conversion and gamma correction are not included
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_primary_plane_format {
    pub /: *mut *mut u8 enabled; / plane is enabled,
    pub /: *mut *mut u32 tiled; / tiling mode: linear, X-tiled, Y tiled, etc,
    pub /: *mut *mut u8 bpp; / bits per pixel,
    pub /: *mut *mut u32 hw_format; / format field in the PRI_CTL register,
    pub /: *mut *mut u32 drm_format; / format in DRM definition,
    pub /: *mut *mut u32 base; / framebuffer base in graphics memory,
    pub base_gpa: u64,
    pub /: *mut *mut u32 x_offset; / in pixels,
    pub /: *mut *mut u32 y_offset; / in lines,
    pub /: *mut *mut u32 width; / in pixels,
    pub /: *mut *mut u32 height; / in lines,
    pub /: *mut *mut u32 stride; / in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_sprite_plane_format {
    pub /: *mut *mut u8 enabled; / plane is enabled,
    pub /: *mut *mut u8 tiled; / X-tiled,
    pub /: *mut *mut u8 bpp; / bits per pixel,
    pub /: *mut *mut u32 hw_format; / format field in the SPR_CTL register,
    pub /: *mut *mut u32 drm_format; / format in DRM definition,
    pub /: *mut *mut u32 base; / sprite base in graphics memory,
    pub base_gpa: u64,
    pub /: *mut *mut u32 x_pos; / in pixels,
    pub /: *mut *mut u32 y_pos; / in lines,
    pub /: *mut *mut u32 x_offset; / in pixels,
    pub /: *mut *mut u32 y_offset; / in lines,
    pub /: *mut *mut u32 width; / in pixels,
    pub /: *mut *mut u32 height; / in lines,
    pub /: *mut *mut u32 stride; / in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_cursor_plane_format {
    pub enabled: u8,
    pub /: *mut *mut u8 mode; / cursor mode select,
    pub /: *mut *mut u8 bpp; / bits per pixel,
    pub /: *mut *mut u32 drm_format; / format in DRM definition,
    pub /: *mut *mut u32 base; / cursor base in graphics memory,
    pub base_gpa: u64,
    pub /: *mut *mut u32 x_pos; / in pixels,
    pub /: *mut *mut u32 y_pos; / in lines,
    pub /: *mut *mut u8 x_sign; / X Position Sign,
    pub /: *mut *mut u8 y_sign; / Y Position Sign,
    pub /: *mut *mut u32 width; / in pixels,
    pub /: *mut *mut u32 height; / in lines,
    pub /: *mut *mut u32 x_hot; / in pixels,
    pub /: *mut *mut u32 y_hot; / in pixels,
}
