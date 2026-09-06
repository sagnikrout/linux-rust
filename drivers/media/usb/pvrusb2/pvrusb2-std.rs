//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pvrusb2/pvrusb2-std.h
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
//
// Copyright (C) 2005 Mike Isely <isely@pobox.com>
//

// Convert string describing one or more video standards into a mask of V4L
// standard bits.  Return true if conversion succeeds otherwise return
// false.  String is expected to be of the form: C1-x/y;C2-a/b where C1 and
// C2 are color system names (e.g. "PAL", "NTSC") and x, y, a, and b are
// modulation schemes (e.g. "M", "B", "G", etc).
// Convert any arbitrary set of video standard bits into an unambiguous
// readable string.  Return value is the number of bytes consumed in the
// buffer.  The formatted string is of a form that can be parsed by our
// sibling std_std_to_id() function.
// Return mask of which video standard bits are valid
extern "C" {
    pub fn pvr2_std_get_usable() -> v4l2_std_id;
}
