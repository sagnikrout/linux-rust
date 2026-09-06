//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/x86/include/uapi/asm/unistd_32.h
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

pub const __NR_fork: c_int = 2;

pub const __NR_execve: c_int = 11;

pub const __NR_getppid: c_int = 64;

pub const __NR_getpgid: c_int = 132;

pub const __NR_capget: c_int = 184;

pub const __NR_gettid: c_int = 224;

pub const __NR_futex: c_int = 240;

pub const __NR_getcpu: c_int = 318;

pub const __NR_perf_event_open: c_int = 336;

pub const __NR_setns: c_int = 346;

pub const __NR_seccomp: c_int = 354;
