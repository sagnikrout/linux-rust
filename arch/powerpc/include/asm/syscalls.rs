//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/syscalls.h
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

extern "C" {
    pub fn sys_ni_syscall() -> c_long;
}

extern "C" {
    pub fn sys_ni_syscall(regs: *const pt_regs) -> c_long;
}

//
// long long munging:
// The 32 bit ABI passes long longs in an odd even register pair.
// High and low parts are swapped depending on endian mode,
// so define a macro (similar to mips linux32) to handle that.
//

//
// PowerPC architecture-specific syscalls
//

extern "C" {
    pub fn sys_rtas(uargs: *mut rtas_args __user) -> c_long;
}

extern "C" {
    pub fn sys_ppc64_personality(personality: c_ulong) -> c_long;
}

extern "C" {
    pub fn compat_sys_ppc64_personality(personality: c_ulong) -> c_long;
}

extern "C" {
    pub fn sys_switch_endian() -> c_long;
}

extern "C" {
    pub fn sys_sigreturn() -> c_long;
}

extern "C" {
    pub fn sys_rt_sigreturn() -> c_long;
}

extern "C" {
    pub fn compat_sys_sigreturn() -> c_long;
}
extern "C" {
    pub fn compat_sys_rt_sigreturn() -> c_long;
}

//
// Architecture specific signatures required by long long munging:
// The 32 bit ABI passes long longs in an odd even register pair.
// The following signatures provide a machine long parameter for
// each register that will be supplied. The implementation is
// responsible for combining parameter pairs.
//

extern "C" {
    pub fn entry(regs: *const pt_regs) -> c_long;
}

