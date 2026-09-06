//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/omapfb.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// File: include/linux/omapfb.h
//
// Framebuffer driver for TI OMAP boards
//
// Copyright (C) 2004 Nokia Corporation
// Author: Imre Deak <imre.deak@nokia.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 2 of the License, or (at your
// option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 59 Temple Place - Suite 330, Boston, MA  02111-1307, USA.
//

// IOCTL commands.

pub const OMAPFB_CAPS_GENERIC_MASK: c_uint = 0x00000fff;
pub const OMAPFB_CAPS_LCDC_MASK: c_uint = 0x00fff000;
pub const OMAPFB_CAPS_PANEL_MASK: c_uint = 0xff000000;
pub const OMAPFB_CAPS_MANUAL_UPDATE: c_uint = 0x00001000;
pub const OMAPFB_CAPS_TEARSYNC: c_uint = 0x00002000;
pub const OMAPFB_CAPS_PLANE_RELOCATE_MEM: c_uint = 0x00004000;
pub const OMAPFB_CAPS_PLANE_SCALE: c_uint = 0x00008000;
pub const OMAPFB_CAPS_WINDOW_PIXEL_DOUBLE: c_uint = 0x00010000;
pub const OMAPFB_CAPS_WINDOW_SCALE: c_uint = 0x00020000;
pub const OMAPFB_CAPS_WINDOW_OVERLAY: c_uint = 0x00040000;
pub const OMAPFB_CAPS_WINDOW_ROTATE: c_uint = 0x00080000;
pub const OMAPFB_CAPS_SET_BACKLIGHT: c_uint = 0x01000000;
// Values from DSP must map to lower 16-bits
pub const OMAPFB_FORMAT_MASK: c_uint = 0x00ff;
pub const OMAPFB_FORMAT_FLAG_DOUBLE: c_uint = 0x0100;
pub const OMAPFB_FORMAT_FLAG_TEARSYNC: c_uint = 0x0200;
pub const OMAPFB_FORMAT_FLAG_FORCE_VSYNC: c_uint = 0x0400;
pub const OMAPFB_FORMAT_FLAG_ENABLE_OVERLAY: c_uint = 0x0800;
pub const OMAPFB_FORMAT_FLAG_DISABLE_OVERLAY: c_uint = 0x1000;
pub const OMAPFB_MEMTYPE_SDRAM: c_int = 0;
pub const OMAPFB_MEMTYPE_SRAM: c_int = 1;
pub const OMAPFB_MEMTYPE_MAX: c_int = 1;
pub const OMAPFB_MEM_IDX_ENABLED: c_uint = 0x80;
pub const OMAPFB_MEM_IDX_MASK: c_uint = 0x7f;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omapfb_color_format {
    OMAPFB_COLOR_RGB565 = 0,
    OMAPFB_COLOR_YUV422,
    OMAPFB_COLOR_YUV420,
    OMAPFB_COLOR_CLUT_8BPP,
    OMAPFB_COLOR_CLUT_4BPP,
    OMAPFB_COLOR_CLUT_2BPP,
    OMAPFB_COLOR_CLUT_1BPP,
    OMAPFB_COLOR_RGB444,
    OMAPFB_COLOR_YUY422,

    OMAPFB_COLOR_ARGB16,
    OMAPFB_COLOR_RGB24U,	/* RGB24, 32-bit container */
    OMAPFB_COLOR_RGB24P,	/* RGB24, 24-bit container */
    OMAPFB_COLOR_ARGB32,
    OMAPFB_COLOR_RGBA32,
    OMAPFB_COLOR_RGBX32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_update_window {
    pub y: __u32 x,,
    pub height: __u32 width,,
    pub format: __u32,
    pub out_y: __u32 out_x,,
    pub out_height: __u32 out_width,,
    pub reserved: [__u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_update_window_old {
    pub y: __u32 x,,
    pub height: __u32 width,,
    pub format: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omapfb_plane {
    OMAPFB_PLANE_GFX = 0,
    OMAPFB_PLANE_VID1,
    OMAPFB_PLANE_VID2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omapfb_channel_out {
    OMAPFB_CHANNEL_OUT_LCD = 0,
    OMAPFB_CHANNEL_OUT_DIGIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_plane_info {
    pub pos_x: __u32,
    pub pos_y: __u32,
    pub enabled: __u8,
    pub channel_out: __u8,
    pub mirror: __u8,
    pub mem_idx: __u8,
    pub out_width: __u32,
    pub out_height: __u32,
    pub reserved2: [__u32; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_mem_info {
    pub size: __u32,
    pub type: __u8,
    pub reserved: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_caps {
    pub ctrl: __u32,
    pub plane_color: __u32,
    pub wnd_color: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omapfb_color_key_type {
    OMAPFB_COLOR_KEY_DISABLED = 0,
    OMAPFB_COLOR_KEY_GFX_DST,
    OMAPFB_COLOR_KEY_VID_SRC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_color_key {
    pub channel_out: __u8,
    pub background: __u32,
    pub trans_key: __u32,
    pub key_type: __u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omapfb_update_mode {
    OMAPFB_UPDATE_DISABLED = 0,
    OMAPFB_AUTO_UPDATE,
    OMAPFB_MANUAL_UPDATE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_memory_read {
    pub x: __u16,
    pub y: __u16,
    pub w: __u16,
    pub h: __u16,
    pub buffer_size: usize,
    pub buffer: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_ovl_colormode {
    pub overlay_idx: __u8,
    pub mode_idx: __u8,
    pub bits_per_pixel: __u32,
    pub nonstd: __u32,
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_vram_info {
    pub total: __u32,
    pub free: __u32,
    pub largest_free_block: __u32,
    pub reserved: [__u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_tearsync_info {
    pub enabled: __u8,
    pub reserved1: [__u8; 3],
    pub line: __u16,
    pub reserved2: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_display_info {
    pub xres: __u16,
    pub yres: __u16,
    pub /: *mut *mut __u32 width; / phys width of the display in micrometers,
    pub /: *mut *mut __u32 height; / phys height of the display in micrometers,
    pub reserved: [__u32; 5],
}
