//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/linux_logo.h
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
// Linux logo to be displayed on boot
//
// Copyright (C) 1996 Larry Ewing (lewing@isc.tamu.edu)
// Copyright (C) 1996,1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
// Copyright (C) 2001 Greg Banks <gnb@alphalink.com.au>
// Copyright (C) 2001 Jan-Benedict Glaw <jbglaw@lug-owl.de>
// Copyright (C) 2003 Geert Uytterhoeven <geert@linux-m68k.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_logo {
    pub /: *mut *mut *mut int type; / one of LINUX_LOGO_,
    pub width: c_uint,
    pub height: c_uint,
    pub /: *mut *mut unsigned int clutsize; / LINUX_LOGO_CLUT224 only,
    pub /: *const *const *const unsigned char clut; / LINUX_LOGO_CLUT224 only,
    pub data: *const c_uchar,
}

