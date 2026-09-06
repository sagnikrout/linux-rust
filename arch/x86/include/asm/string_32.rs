//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/string_32.h
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

// Let gcc decide whether to inline or use the out of line functions
extern "C" {
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
}
extern "C" {
    pub fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int;
}
extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
}
//
// This looks ugly, but the compiler can optimize it totally,
// as the count is constant.
//
// (char *)to = *(char *)from;
// (short *)to = *(short *)from;
// (int *)to = *(int *)from;
// (short *)to = *(short *)from;
// ((char *)to + 2) = *((char *)from + 2);
// (int *)to = *(int *)from;
// ((char *)to + 4) = *((char *)from + 4);
// (int *)to = *(int *)from;
// ((short *)to + 2) = *((short *)from + 2);
// (int *)to = *(int *)from;
// ((int *)to + 1) = *((int *)from + 1);
// large block: use rep prefix
// small block: don't clobber ecx + smaller code
// tail

extern "C" {
    pub fn memcmp(: *const c_void, : *const c_void, _arg: usize) -> c_int;
}

// we might want to write optimized versions of these later

// Added by Gertjan van Wingerde to make minix and sysv module work
extern "C" {
    pub fn strnlen(s: *const c_char, count: usize) -> usize;
}
// end of additional stuff

//
// find the first occurrence of byte 'c', or 1 past the area if none
//

