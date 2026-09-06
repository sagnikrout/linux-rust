//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/nvidia/nv_local.h
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


// \
//
// GPL Licensing Note - According to Mark Vojkovich, author of the Xorg
// XFree86 'nv' driver, this source code is provided under MIT-style licensing
// where the source code is provided "as is" without warranty of any kind.
// The only usage restriction is for the copyright notices to be retained
// whenever code is used.
//
// Antonino Daplas <adaplas@pol.net> 2005-03-11
//
// This file includes any environment or machine specific values to access the
// HW.  Put all affected includes, typdefs, etc. here so the riva_hw.* files
// can stay generic in nature.
//
// HW access macros.  These assume memory-mapped I/O, and not normal I/O space.
//

// VGA I/O is now always done through MMIO

