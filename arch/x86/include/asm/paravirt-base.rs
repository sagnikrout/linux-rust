//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/paravirt-base.h
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
// Wrapper type for pointers to code which uses the non-standard
// calling convention.  See PV_CALL_SAVE_REGS_THUNK below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct paravirt_callee_save {
    pub func: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_info {

    pub /: *mut *mut u16 extra_user_64bit_cs; / __USER_CS if none,

    pub io_delay: bool,
    pub name: *const c_char,
}

extern "C" {
    pub fn default_banner();
}
extern "C" {
    pub fn paravirt_ret0() -> c_ulong;
}

extern "C" {
    pub fn _paravirt_ident_64(_arg: u64) -> u64;
}

extern "C" {
    pub fn paravirt_set_cap();
}

