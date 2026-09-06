//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/unwind.h
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
// Most of this ideas comes from x86.
//
// Copyright (C) 2022 Loongson Technology Corporation Limited
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum unwinder_type {
    UNWINDER_GUESS,
    UNWINDER_PROLOGUE,
    UNWINDER_ORC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_state {
    pub /: *mut *mut char type; / UNWINDER_XXX,
    pub stack_info: stack_info,
    pub task: *mut task_struct,
    pub reset: bool first, error,,
    pub graph_idx: c_int,
    pub ra: unsigned long sp, fp, pc,,
}

extern "C" {
    pub fn default_next_frame(state: *mut unwind_state) -> bool;
}
extern "C" {
    pub fn unwind_next_frame(state: *mut unwind_state) -> bool;
}
extern "C" {
    pub fn unwind_get_return_address(state: *mut unwind_state) -> c_ulong;
}

extern "C" {
    pub fn unwind_init();
}
extern "C" {
    pub fn unwind_module_init(mod: *mut module, orc_ip: *mut c_void, orc_ip_size: usize, orc: *mut c_void, orc_size: usize);
}

