//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/alternative.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2021 Sifive.
//

// add the relative offset to the address of the offset to get the absolute address

extern "C" {
    pub fn apply_boot_alternatives() -> void __init;
}
extern "C" {
    pub fn apply_early_boot_alternatives() -> void __init;
}
extern "C" {
    pub fn apply_module_alternatives(start: *mut c_void, length: usize);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alt_entry {
    pub /: *mut *mut s32 old_offset; / offset relative to original instruction or data,
    pub /: *mut *mut s32 alt_offset; / offset relative to replacement instruction or data,
    pub /: *mut *mut u16 vendor_id; / CPU vendor ID,
    pub /: *mut *mut u16 alt_len; / The replacement size,
    pub /: *mut *mut u32 patch_id; / The patch ID (erratum ID or cpufeature ID),
}

