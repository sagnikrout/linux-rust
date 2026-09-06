//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/sh_keysc.h
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
pub const SH_KEYSC_MAXKEYS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_keysc_info {
    pub mode: SH_KEYSC_MODE_4, SH_KEYSC_MODE_5, SH_KEYSC_MODE_6 },
    pub /: *mut *mut int scan_timing; / 0 -> 7, see KYCR1, SCN[2:0],
    pub delay: c_int,
    pub kycr2_delay: c_int,
    pub /: *mut *mut *mut int keycodes[SH_KEYSC_MAXKEYS]; / KEYIN  KEYOUT,
}
