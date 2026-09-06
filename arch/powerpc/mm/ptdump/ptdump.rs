//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/mm/ptdump/ptdump.h
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
pub struct flag_info {
    pub mask: u64,
    pub val: u64,
    pub set: *const c_char,
    pub clear: *const c_char,
    pub is_val: bool,
    pub shift: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptdump_pg_level {
    pub flag: *const flag_info,
    pub name: [c_char; 4],
    pub num: usize,
    pub mask: u64,
}

extern "C" {
    pub fn pt_dump_size(m: *mut seq_file, delta: c_ulong);
}
