//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/um/shared/sysdep/tls.h
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

// Change name to avoid conflicts with the original one from <asm/ldt.h>, which
// may be named user_desc (but in 2.4 and in header matching its API was named
// modify_ldt_ldt_s).

pub type user_desc_t = user_desc;

extern "C" {
    pub fn os_set_thread_area(info: *mut user_desc_t, pid: c_int) -> c_int;
}
extern "C" {
    pub fn os_get_thread_area(info: *mut user_desc_t, pid: c_int) -> c_int;
}

pub const GDT_ENTRY_TLS_MIN_I386: c_int = 6;
pub const GDT_ENTRY_TLS_MIN_X86_64: c_int = 12;

