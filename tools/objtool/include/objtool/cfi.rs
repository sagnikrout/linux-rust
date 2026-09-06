//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/include/objtool/cfi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2015-2017 Josh Poimboeuf <jpoimboe@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_reg {
    pub base: c_int,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_init_state {
    pub regs: [cfi_reg; CFI_NUM_REGS],
    pub cfa: cfi_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_state {
    pub /: *mut *mut hlist_node hash; / must be first, cficmp(),
    pub regs: [cfi_reg; CFI_NUM_REGS],
    pub vals: [cfi_reg; CFI_NUM_REGS],
    pub cfa: cfi_reg,
    pub stack_size: c_int,
    pub drap_offset: int drap_reg,,
    pub type: c_uchar,
    pub bp_scratch: bool,
    pub drap: bool,
    pub signal: bool,
    pub end: bool,
    pub force_undefined: bool,
}
