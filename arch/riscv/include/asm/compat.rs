//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/compat.h
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
// Architecture specific compatibility types
//

extern "C" {
    pub fn test_thread_flag(_arg: TIF_32BIT) -> return;
}
extern "C" {
    pub fn test_ti_thread_flag(_arg: thread, _arg: TIF_32BIT) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_user_regs_struct {
    pub pc: compat_ulong_t,
    pub ra: compat_ulong_t,
    pub sp: compat_ulong_t,
    pub gp: compat_ulong_t,
    pub tp: compat_ulong_t,
    pub t0: compat_ulong_t,
    pub t1: compat_ulong_t,
    pub t2: compat_ulong_t,
    pub s0: compat_ulong_t,
    pub s1: compat_ulong_t,
    pub a0: compat_ulong_t,
    pub a1: compat_ulong_t,
    pub a2: compat_ulong_t,
    pub a3: compat_ulong_t,
    pub a4: compat_ulong_t,
    pub a5: compat_ulong_t,
    pub a6: compat_ulong_t,
    pub a7: compat_ulong_t,
    pub s2: compat_ulong_t,
    pub s3: compat_ulong_t,
    pub s4: compat_ulong_t,
    pub s5: compat_ulong_t,
    pub s6: compat_ulong_t,
    pub s7: compat_ulong_t,
    pub s8: compat_ulong_t,
    pub s9: compat_ulong_t,
    pub s10: compat_ulong_t,
    pub s11: compat_ulong_t,
    pub t3: compat_ulong_t,
    pub t4: compat_ulong_t,
    pub t5: compat_ulong_t,
    pub t6: compat_ulong_t,
}
