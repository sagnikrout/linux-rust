//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/vdso.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdso_image {
    pub data: *mut c_void,
    pub /: *mut *mut unsigned long size; / Always a multiple of PAGE_SIZE,
    pub alt_len: unsigned long alt,,
    pub extable_len: unsigned long extable_base,,
    pub extable: *const c_void,
    pub sym___kernel_sigreturn: c_long,
    pub sym___kernel_rt_sigreturn: c_long,
    pub sym___kernel_vsyscall: c_long,
    pub sym_int80_landing_pad: c_long,
    pub sym_vdso32_sigreturn_landing_pad: c_long,
    pub sym_vdso32_rt_sigreturn_landing_pad: c_long,
    pub sym___futex_list64_try_unlock_cs_start: c_long,
    pub sym___futex_list64_try_unlock_cs_end: c_long,
    pub sym___futex_list32_try_unlock_cs_start: c_long,
    pub sym___futex_list32_try_unlock_cs_end: c_long,
}

extern "C" {
    pub fn init_vdso_image(image: *const vdso_image) -> int __init;
}
extern "C" {
    pub fn map_vdso_once(image: *const vdso_image, addr: c_ulong) -> c_int;
}

