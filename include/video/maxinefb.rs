//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/maxinefb.h
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
// linux/drivers/video/maxinefb.h
//
// DECstation 5000/xx onboard framebuffer support, Copyright (C) 1999 by
// Michael Engel <engel@unix-ag.org> and Karsten Merker <merker@guug.de>
// This file is subject to the terms and conditions of the GNU General
// Public License.  See the file COPYING in the main directory of this
// archive for more details.
//

//
// IMS332 video controller register base address
//

//
// Begin of DECstation 5000/xx onboard framebuffer memory, default resolution
// is 1024x768x8
//

//
// The IMS 332 video controller used in the DECstation 5000/xx series
// uses 32 bits wide registers; the following defines declare the
// register numbers, to get the real offset, these have to be multiplied
// by four.
//
pub const IMS332_REG_CURSOR_RAM: c_uint = 0x200	/* hardware cursor bitmap */;
//
// The color palette entries have the form 0x00BBGGRR
//
pub const IMS332_REG_COLOR_PALETTE: c_uint = 0x100	/* color palette, 256 entries */;
pub const IMS332_REG_CURSOR_COLOR_PALETTE: c_uint = 0x0a1	/* cursor color palette, */;
// 3 entries
