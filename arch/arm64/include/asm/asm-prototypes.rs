//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/asm-prototypes.h
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
// CONFIG_MODVERSIONS requires a C declaration to generate the appropriate CRC
// for each symbol. Since commit:
//
// 4efca4ed05cbdfd1 ("kbuild: modversions for EXPORT_SYMBOL() for asm")
//
// ... kbuild will automatically pick these up from <asm/asm-prototypes.h> and
// feed this to genksyms when building assembly files.
//

extern "C" {
    pub fn __ashlti3(a: c_longlong, b: c_int) -> c_longlong;
}
extern "C" {
    pub fn __ashrti3(a: c_longlong, b: c_int) -> c_longlong;
}
extern "C" {
    pub fn __lshrti3(a: c_longlong, b: c_int) -> c_longlong;
}
//
// This function uses a custom calling convention and cannot be called from C so
// this prototype is not entirely accurate.
//
extern "C" {
    pub fn __hwasan_tag_mismatch(addr: c_ulong, access_info: c_ulong);
}
