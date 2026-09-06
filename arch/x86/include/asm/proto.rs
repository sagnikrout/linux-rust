//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/proto.h
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

// misc architecture specific prototypes
extern "C" {
    pub fn syscall_init();
}

extern "C" {
    pub fn entry_SYSCALL_64();
}
extern "C" {
    pub fn entry_SYSCALL_64_safe_stack();
}
extern "C" {
    pub fn entry_SYSRETQ_unsafe_stack();
}
extern "C" {
    pub fn entry_SYSRETQ_end();
}

extern "C" {
    pub fn entry_INT80_32();
}
extern "C" {
    pub fn entry_SYSENTER_32();
}
extern "C" {
    pub fn __begin_SYSENTER_singlestep_region();
}
extern "C" {
    pub fn __end_SYSENTER_singlestep_region();
}

extern "C" {
    pub fn entry_SYSENTER_compat();
}
extern "C" {
    pub fn __end_entry_SYSENTER_compat();
}
extern "C" {
    pub fn entry_SYSCALL_compat();
}
extern "C" {
    pub fn entry_SYSCALL_compat_safe_stack();
}
extern "C" {
    pub fn entry_SYSRETL_compat_unsafe_stack();
}
extern "C" {
    pub fn entry_SYSRETL_compat_end();
}

extern "C" {
    pub fn x86_configure_nx();
}
extern "C" {
    pub fn do_arch_prctl_64(task: *mut task_struct, option: c_int, arg2: c_ulong) -> c_long;
}
