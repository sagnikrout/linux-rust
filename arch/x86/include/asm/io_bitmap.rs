//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/io_bitmap.h
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
pub struct io_bitmap {
    pub sequence: u64,
    pub refcnt: refcount_t,
// The maximum number of bytes to copy so all zero bits are covered
    pub max: c_uint,
    pub bitmap: [c_ulong; IO_BITMAP_LONGS],
}

extern "C" {
    pub fn io_bitmap_share(tsk: *mut task_struct);
}
extern "C" {
    pub fn io_bitmap_exit(tsk: *mut task_struct);
}
//
// Invalidate the I/O bitmap by moving io_bitmap_base outside the
// TSS limit so any subsequent I/O access from user space will
// trigger a #GP.
//
// This is correct even when VMEXIT rewrites the TSS limit
// to 0x67 as the only requirement is that the base points
// outside the limit.
//
extern "C" {
    pub fn native_tss_update_io_bitmap();
}

