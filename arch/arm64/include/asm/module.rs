//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/module.h
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
// Copyright (C) 2012 ARM Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_plt_sec {
    pub plt_shndx: c_int,
    pub plt_num_entries: c_int,
    pub plt_max_entries: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_arch_specific {
    pub core: mod_plt_sec,
    pub init: mod_plt_sec,
// for CONFIG_DYNAMIC_FTRACE
    pub ftrace_trampolines: *mut plt_entry,
    pub init_ftrace_trampolines: *mut plt_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plt_entry {
//
// A program that conforms to the AArch64 Procedure Call Standard
// (AAPCS64) must assume that a veneer that alters IP0 (x16) and/or
// IP1 (x17) may be inserted at any branch instruction that is
// exposed to a relocation that supports long branches. Since that
// is exactly what we are dealing with here, we are free to use x16
// as a scratch register in the PLT veneers.
//
    pub /: *mut *mut __le32 adrp; / adrp x16, ....,
    pub /: *mut *mut __le32 add; / add x16, x16, #0x....,
    pub /: *mut *mut __le32 br; / br x16,
}

extern "C" {
    pub fn get_plt_entry(dst: u64, pc: *mut c_void) -> plt_entry;
}
