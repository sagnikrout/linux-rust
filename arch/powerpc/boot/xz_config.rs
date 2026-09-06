//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/boot/xz_config.h
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
// most of this is copied from lib/xz/xz_private.h, we can't use their defines
// since the boot wrapper is not built in the same environment as the rest of
// the kernel.
//

extern "C" {
    pub fn swab32(_arg: *mut q) -> return;
}

extern "C" {
    pub fn swab32p()p: *mut (u32) -> return;
}

extern "C" {
    pub fn be32_to_cpup(_arg: p) -> return;
}
// ((u32 *)p) = cpu_to_be32(val);

// prevent the inclusion of the xz-preboot MM headers

// xz.h needs to be included directly since we need enum xz_mode

