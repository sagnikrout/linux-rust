//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/aspeed/aspeed_gfx.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2018 IBM Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_gfx {
    pub drm: drm_device,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub scu: *mut regmap,
    pub dac_reg: u32,
    pub int_clr_reg: u32,
    pub vga_scratch_reg: u32,
    pub throd_val: u32,
    pub scan_line_max: u32,
    pub pipe: drm_simple_display_pipe,
    pub connector: drm_connector,
}

extern "C" {
    pub fn aspeed_gfx_create_pipe(drm: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn aspeed_gfx_create_output(drm: *mut drm_device) -> c_int;
}
pub const CRT_CTRL1: c_uint = 0x60 /* CRT Control I */;
pub const CRT_CTRL2: c_uint = 0x64 /* CRT Control II */;
pub const CRT_STATUS: c_uint = 0x68 /* CRT Status */;
pub const CRT_MISC: c_uint = 0x6c /* CRT Misc Setting */;
pub const CRT_HORIZ0: c_uint = 0x70 /* CRT Horizontal Total & Display Enable End */;
pub const CRT_HORIZ1: c_uint = 0x74 /* CRT Horizontal Retrace Start & End */;
pub const CRT_VERT0: c_uint = 0x78 /* CRT Vertical Total & Display Enable End */;
pub const CRT_VERT1: c_uint = 0x7C /* CRT Vertical Retrace Start & End */;
pub const CRT_ADDR: c_uint = 0x80 /* CRT Display Starting Address */;
pub const CRT_OFFSET: c_uint = 0x84 /* CRT Display Offset & Terminal Count */;
pub const CRT_THROD: c_uint = 0x88 /* CRT Threshold */;
pub const CRT_XSCALE: c_uint = 0x8C /* CRT Scaling-Up Factor */;
pub const CRT_CURSOR0: c_uint = 0x90 /* CRT Hardware Cursor X & Y Offset */;
pub const CRT_CURSOR1: c_uint = 0x94 /* CRT Hardware Cursor X & Y Position */;
pub const CRT_CURSOR2: c_uint = 0x98 /* CRT Hardware Cursor Pattern Address */;
pub const CRT_9C: c_uint = 0x9C;
pub const CRT_OSD_H: c_uint = 0xA0 /* CRT OSD Horizontal Start/End */;
pub const CRT_OSD_V: c_uint = 0xA4 /* CRT OSD Vertical Start/End */;
pub const CRT_OSD_ADDR: c_uint = 0xA8 /* CRT OSD Pattern Address */;
pub const CRT_OSD_DISP: c_uint = 0xAC /* CRT OSD Offset */;
pub const CRT_OSD_THRESH: c_uint = 0xB0 /* CRT OSD Threshold & Alpha */;
pub const CRT_B4: c_uint = 0xB4;
pub const CRT_STS_V: c_uint = 0xB8 /* CRT Status V */;
pub const CRT_SCRATCH: c_uint = 0xBC /* Scratchpad */;
pub const CRT_BB0_ADDR: c_uint = 0xD0 /* CRT Display BB0 Starting Address */;
pub const CRT_BB1_ADDR: c_uint = 0xD4 /* CRT Display BB1 Starting Address */;
pub const CRT_BB_COUNT: c_uint = 0xD8 /* CRT Display BB Terminal Count */;
pub const OSD_COLOR1: c_uint = 0xE0 /* OSD Color Palette Index 1 & 0 */;
pub const OSD_COLOR2: c_uint = 0xE4 /* OSD Color Palette Index 3 & 2 */;
pub const OSD_COLOR3: c_uint = 0xE8 /* OSD Color Palette Index 5 & 4 */;
pub const OSD_COLOR4: c_uint = 0xEC /* OSD Color Palette Index 7 & 6 */;
pub const OSD_COLOR5: c_uint = 0xF0 /* OSD Color Palette Index 9 & 8 */;
pub const OSD_COLOR6: c_uint = 0xF4 /* OSD Color Palette Index 11 & 10 */;
pub const OSD_COLOR7: c_uint = 0xF8 /* OSD Color Palette Index 13 & 12 */;
pub const OSD_COLOR8: c_uint = 0xFC /* OSD Color Palette Index 15 & 14 */;
// CTRL1

// CTRL2

// CRT_HORIZ0

// CRT_HORIZ1

// CRT_VIRT0

// CRT_VIRT1

// CRT_OFFSET

// CRT_THROD

