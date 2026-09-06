//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/udl/udl_proto.h
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


// SPDX-License-Identifier: GPL-2.0-only

pub const UDL_MSG_BULK: c_uint = 0xaf;
// Register access
pub const UDL_CMD_WRITEREG: c_uint = 0x20 /* See register constants below */;
// Framebuffer access
pub const UDL_CMD_WRITERAW8: c_uint = 0x60 /* 8 bit raw write command. */;
pub const UDL_CMD_WRITERL8: c_uint = 0x61 /* 8 bit run length command. */;
pub const UDL_CMD_WRITECOPY8: c_uint = 0x62 /* 8 bit copy command. */;
pub const UDL_CMD_WRITERLX8: c_uint = 0x63 /* 8 bit extended run length command. */;
pub const UDL_CMD_WRITERAW16: c_uint = 0x68 /* 16 bit raw write command. */;
pub const UDL_CMD_WRITERL16: c_uint = 0x69 /* 16 bit run length command. */;
pub const UDL_CMD_WRITECOPY16: c_uint = 0x6a /* 16 bit copy command. */;
pub const UDL_CMD_WRITERLX16: c_uint = 0x6b /* 16 bit extended run length command. */;
// Color depth
pub const UDL_REG_COLORDEPTH: c_uint = 0x00;
pub const UDL_COLORDEPTH_16BPP: c_int = 0;
pub const UDL_COLORDEPTH_24BPP: c_int = 1;
// Display-mode settings
pub const UDL_REG_XDISPLAYSTART: c_uint = 0x01;
pub const UDL_REG_XDISPLAYEND: c_uint = 0x03;
pub const UDL_REG_YDISPLAYSTART: c_uint = 0x05;
pub const UDL_REG_YDISPLAYEND: c_uint = 0x07;
pub const UDL_REG_XENDCOUNT: c_uint = 0x09;
pub const UDL_REG_HSYNCSTART: c_uint = 0x0b;
pub const UDL_REG_HSYNCEND: c_uint = 0x0d;
pub const UDL_REG_HPIXELS: c_uint = 0x0f;
pub const UDL_REG_YENDCOUNT: c_uint = 0x11;
pub const UDL_REG_VSYNCSTART: c_uint = 0x13;
pub const UDL_REG_VSYNCEND: c_uint = 0x15;
pub const UDL_REG_VPIXELS: c_uint = 0x17;
pub const UDL_REG_PIXELCLOCK5KHZ: c_uint = 0x1b;
// On/Off for driving the DisplayLink framebuffer to the display
pub const UDL_REG_BLANKMODE: c_uint = 0x1f;
pub const UDL_BLANKMODE_ON: c_uint = 0x00 /* hsync and vsync on, visible */;
pub const UDL_BLANKMODE_BLANKED: c_uint = 0x01 /* hsync and vsync on, blanked */;
pub const UDL_BLANKMODE_VSYNC_OFF: c_uint = 0x03 /* vsync off, blanked */;
pub const UDL_BLANKMODE_HSYNC_OFF: c_uint = 0x05 /* hsync off, blanked */;
pub const UDL_BLANKMODE_POWERDOWN: c_uint = 0x07 /* powered off; requires modeset */;
// Framebuffer address
pub const UDL_REG_BASE16BPP_ADDR2: c_uint = 0x20;
pub const UDL_REG_BASE16BPP_ADDR1: c_uint = 0x21;
pub const UDL_REG_BASE16BPP_ADDR0: c_uint = 0x22;
pub const UDL_REG_BASE8BPP_ADDR2: c_uint = 0x26;
pub const UDL_REG_BASE8BPP_ADDR1: c_uint = 0x27;
pub const UDL_REG_BASE8BPP_ADDR0: c_uint = 0x28;

// Lock/unlock video registers
pub const UDL_REG_VIDREG: c_uint = 0xff;
pub const UDL_VIDREG_LOCK: c_uint = 0x00;
pub const UDL_VIDREG_UNLOCK: c_uint = 0xff;
