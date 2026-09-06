//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/clone3/clone3_selftests.h
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

pub const __NR_clone3: c_int = 435;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __clone_args {
    pub flags: __aligned_u64,
    pub pidfd: __aligned_u64,
    pub child_tid: __aligned_u64,
    pub parent_tid: __aligned_u64,
    pub exit_signal: __aligned_u64,
    pub stack: __aligned_u64,
    pub stack_size: __aligned_u64,
    pub tls: __aligned_u64,
    pub set_tid: __aligned_u64,
    pub set_tid_size: __aligned_u64,
    pub cgroup: __aligned_u64,
}

extern "C" {
    pub fn syscall(_arg: __NR_clone3, _arg: args, _arg: size) -> return;
}
// Set to something that will always cause EINVAL.
