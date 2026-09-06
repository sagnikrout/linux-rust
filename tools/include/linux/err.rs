//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/err.h
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
// Original kernel header comment:
//
// Kernel pointers have redundant information, so we can use a
// scheme where we can return either an error code or a normal
// pointer with the same return value.
//
// This should be a per-architecture thing, to allow different
// error and pointer decisions.
//
// Userspace note:
// The same principle works for userspace, because 'error' pointers
// fall down to the unused hole far from user space, as described
// in Documentation/arch/x86/x86_64/mm.rst for x86_64 arch:
//
// 0000000000000000 - 00007fffffffffff (=47 bits) user space, different per mm hole caused by [48:63] sign extension
// ffffffffffe00000 - ffffffffffffffff (=2 MB) unused hole
//
// It should be the same case for other architectures, because
// this code is used in generic kernel code.
//
pub const MAX_ERRNO: c_int = 4095;

extern "C" {
    pub fn IS_ERR_VALUE(long)ptr: (unsigned) -> return;
}
extern "C" {
    pub fn unlikely(long)ptr: !ptr) || IS_ERR_VALUE((unsigned) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: ptr) -> return;
}
//
// ERR_CAST - Explicitly cast an error-valued pointer to another pointer type
// @ptr: The pointer to cast.
//
// Explicitly cast an error-valued pointer to another pointer type in such a
// way as to make it clear that's what's going on.
//
// cast away the const
