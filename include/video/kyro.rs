//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/kyro.h
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
// linux/drivers/video/kyro/kryo.h
//
// Copyright (C) 2002 STMicroelectronics
// Copyright (C) 2004 Paul Mundt
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kyrofb_info {
    pub regbase: *mut void __iomem,
    pub palette: [u32; 16],
    pub /: *mut *mut u32 HTot; / Hor Total Time,
    pub /: *mut *mut u32 HFP; / Hor Front Porch,
    pub /: *mut *mut u32 HST; / Hor Sync Time,
    pub /: *mut *mut u32 HBP; / Hor Back Porch,
    pub /: *mut *mut s32 HSP; / Hor Sync Polarity,
    pub /: *mut *mut u32 VTot; / Ver Total Time,
    pub /: *mut *mut u32 VFP; / Ver Front Porch,
    pub /: *mut *mut u32 VST; / Ver Sync Time,
    pub /: *mut *mut u32 VBP; / Ver Back Porch,
    pub /: *mut *mut s32 VSP; / Ver Sync Polarity,
    pub /: *mut *mut u32 XRES; / X Resolution,
    pub /: *mut *mut u32 YRES; / Y Resolution,
    pub /: *mut *mut u32 VFREQ; / Ver Frequency,
    pub /: *mut *mut u32 PIXCLK; / Pixel Clock,
    pub /: *mut *mut u32 HCLK; / Hor Clock,
// Useful to hold depth here for Linux
    pub PIXDEPTH: u8,
    pub wc_cookie: c_int,
}

//
// benedict.gaster@superh.com
// Added the follow IOCTLS for the creation of overlay services...
//

//
// The follow 3 structures are used to pass data from user space into the kernel
// for the creation of overlay surfaces and setting the video mode.
//
