//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/futex/include/futex2test.h
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
// Futex2 library addons for futex tests
//
// Copyright 2021 Collabora Ltd.
//

pub const __NR_futex_waitv: c_int = 449;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_waitv {
    pub val: __u64,
    pub uaddr: __u64,
    pub flags: __u32,
    pub __reserved: __u32,
}

pub const __NR_futex_wake: c_int = 454;

pub const __NR_futex_wait: c_int = 455;

pub const FUTEX2_SIZE_U32: c_uint = 0x02;

pub const FUTEX2_NUMA: c_uint = 0x04;

pub const FUTEX2_MPOL: c_uint = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex32_numa {
    pub futex: futex_t,
    pub numa: futex_t,
}

//
// futex_waitv - Wait at multiple futexes, wake on any
// @waiters:    Array of waiters
// @nr_waiters: Length of waiters array
// @flags: Operation flags
// @timo:  Optional timeout for operation
//
extern "C" {
    pub fn syscall(_arg: __NR_futex_waitv, _arg: waiters, _arg: nr_waiters, _arg: flags, _arg: &ts, _arg: clockid) -> return;
}
//
// futex_wait() - block on uaddr with optional timeout
// @val:	Expected value
// @flags:	FUTEX2 flags
// @timeout:	Relative timeout
// @clockid:	Clock id for the timeout
//
extern "C" {
    pub fn syscall(_arg: __NR_futex_wait, _arg: uaddr, _arg: val, _arg: ~0U, _arg: flags, _arg: timeout, _arg: clockid) -> return;
}
//
// futex2_wake() - Wake a number of futexes
// @nr:		Number of threads to wake at most
// @flags:	FUTEX2 flags
//
extern "C" {
    pub fn syscall(_arg: __NR_futex_wake, _arg: uaddr, _arg: ~0U, _arg: nr, _arg: flags) -> return;
}
