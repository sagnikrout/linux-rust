//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/cx231xx/cx231xx-vbi.h
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
//

pub const NTSC_VBI_END_LINE: c_int = 21;

pub const PAL_VBI_START_LINE: c_int = 6;
pub const PAL_VBI_END_LINE: c_int = 23;

pub const VBI_STRIDE: c_int = 1440;
pub const VBI_SAMPLES_PER_LINE: c_int = 1440;
pub const CX231XX_NUM_VBI_PACKETS: c_int = 4;
pub const CX231XX_NUM_VBI_BUFS: c_int = 5;
// stream functions
extern "C" {
    pub fn cx231xx_uninit_vbi_isoc(dev: *mut cx231xx);
}
// vbi data copy functions
