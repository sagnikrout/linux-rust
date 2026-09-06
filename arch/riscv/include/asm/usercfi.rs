//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/usercfi.h
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
// Copyright (C) 2024 Rivos, Inc.
// Deepak Gupta <debug@rivosinc.com>
//
pub const CMDLINE_DISABLE_RISCV_USERCFI_FCFI: c_int = 1;
pub const CMDLINE_DISABLE_RISCV_USERCFI_BCFI: c_int = 2;
pub const CMDLINE_DISABLE_RISCV_USERCFI: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_state {
    pub /: *mut *mut unsigned long ubcfi_en : 1; / Enable for backward cfi.,
    pub 1: unsigned long ubcfi_locked :,
    pub /: *mut *mut unsigned long ufcfi_en : 1; / Enable for forward cfi. Note that ELP goes in sstatus,
    pub 1: unsigned long ufcfi_locked :,
    pub /: *mut *mut unsigned long user_shdw_stk; / Current user shadow stack pointer,
    pub /: *mut *mut unsigned long shdw_stk_base; / Base address of shadow stack,
    pub /: *mut *mut unsigned long shdw_stk_size; / size of shadow stack,
}

extern "C" {
    pub fn shstk_release(tsk: *mut task_struct);
}
extern "C" {
    pub fn set_shstk_base(task: *mut task_struct, shstk_addr: c_ulong, size: c_ulong);
}
extern "C" {
    pub fn get_shstk_base(task: *mut task_struct, size: *mut c_ulong) -> c_ulong;
}
extern "C" {
    pub fn set_active_shstk(task: *mut task_struct, shstk_addr: c_ulong);
}
extern "C" {
    pub fn is_shstk_enabled(task: *mut task_struct) -> bool;
}
extern "C" {
    pub fn is_shstk_locked(task: *mut task_struct) -> bool;
}
extern "C" {
    pub fn is_shstk_allocated(task: *mut task_struct) -> bool;
}
extern "C" {
    pub fn set_shstk_lock(task: *mut task_struct, lock: bool);
}
extern "C" {
    pub fn set_shstk_status(task: *mut task_struct, enable: bool);
}
extern "C" {
    pub fn get_active_shstk(task: *mut task_struct) -> c_ulong;
}
extern "C" {
    pub fn restore_user_shstk(tsk: *mut task_struct, shstk_ptr: c_ulong) -> c_int;
}
extern "C" {
    pub fn save_user_shstk(tsk: *mut task_struct, saved_shstk_ptr: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn is_indir_lp_enabled(task: *mut task_struct) -> bool;
}
extern "C" {
    pub fn is_indir_lp_locked(task: *mut task_struct) -> bool;
}
extern "C" {
    pub fn set_indir_lp_status(task: *mut task_struct, enable: bool);
}
extern "C" {
    pub fn set_indir_lp_lock(task: *mut task_struct, lock: bool);
}

// Macro flag: #define shstk_release(tsk)

extern "C" {
    pub fn is_user_shstk_enabled() -> bool;
}
extern "C" {
    pub fn is_user_lpad_enabled() -> bool;
}

